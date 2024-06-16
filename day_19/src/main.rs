mod part;
mod workflow;

use std::str::FromStr;

use part::{Part, PossibilityPart};

use crate::part::Parts;
use crate::workflow::Workflows;
use workflow::{Condition, Next};

fn eval_condition(condition: &Condition, part: &Part) -> bool {
    type CompFn = dyn Fn(&u32, &u32) -> bool;
    let (op, var, val) = match condition {
        Condition::Less(c, v) => (&u32::lt as &CompFn, c, v),
        Condition::Greater(c, v) => (&u32::gt as &CompFn, c, v),
    };
    let part_value = match var {
        'x' => part.x,
        'm' => part.m,
        'a' => part.a,
        's' => part.s,
        _ => unreachable!(),
    };

    op(&part_value, val)
}

fn split_part(part: &PossibilityPart, condition: &Condition) -> (PossibilityPart, PossibilityPart) {
    let (var, val) = match condition {
        Condition::Less(c, v) => (c, *v as u16),
        Condition::Greater(c, v) => (c, *v as u16),
    };
    let range = match var {
        'x' => &part.x,
        'm' => &part.m,
        'a' => &part.a,
        's' => &part.s,
        _ => unreachable!(),
    };
    let (than_range, else_range) = match condition {
        Condition::Less(_, _) => {
            let then_start = *range.start();
            let then_end = (*range.end()).min(val - 1);
            let else_start = (*range.start()).max(val);
            let else_end = *range.end();
            ((then_start..=then_end), (else_start..=else_end))
        }
        Condition::Greater(_, _) => {
            let then_start = (*range.start()).max(val + 1);
            let then_end = *range.end();
            let else_start = *range.start();
            let else_end = (*range.end()).min(val);
            ((then_start..=then_end), (else_start..=else_end))
        }
    };

    let (mut than_part, mut else_part) = (part.clone(), part.clone());

    match var {
        'x' => {
            than_part.x = than_range;
            else_part.x = else_range;
        }
        'm' => {
            than_part.m = than_range;
            else_part.m = else_range;
        }
        'a' => {
            than_part.a = than_range;
            else_part.a = else_range;
        }
        's' => {
            than_part.s = than_range;
            else_part.s = else_range;
        }
        _ => unreachable!(),
    }
    (than_part, else_part)
}

fn process_part(part: &Part, workflows: &Workflows, steps: &[Next]) -> bool {
    let step = &steps[0];

    let next = match step {
        workflow::Next::Check(condition, next) => {
            if eval_condition(&condition, part) {
                next
            } else {
                return process_part(part, workflows, &steps[1..]);
            }
        }
        workflow::Next::Else(next) => next,
    };

    match next.as_str() {
        "A" => true,
        "R" => false,
        next_state => process_part(part, workflows, &workflows.0[next_state]),
    }
}

fn is_accepted(part: &Part, workflows: &Workflows) -> bool {
    process_part(part, workflows, &workflows.0["in"])
}

fn eval_possibilities_workflow(
    part: &PossibilityPart,
    workflows: &Workflows,
    steps: &[Next],
) -> Vec<PossibilityPart> {
    if part.is_empty() {
        return vec![];
    }

    let step = &steps[0];
    let next = match step {
        workflow::Next::Check(condition, next) => {
            let (than_part, else_part) = split_part(part, &condition);
            let mut possible_parts =
                eval_possibilities_workflow(&than_part, workflows, &workflows.0[next.as_str()]);
            possible_parts.append(&mut eval_possibilities_workflow(
                &else_part,
                workflows,
                &steps[1..],
            ));
            return possible_parts;
        }
        workflow::Next::Else(next) => next,
    };

    match next.as_str() {
        "A" => vec![part.clone()],
        "R" => vec![],
        _ => eval_possibilities_workflow(&part, workflows, &workflows.0[next.as_str()]),
    }
}

fn eval_possibilities(workflows: &Workflows) -> Vec<PossibilityPart> {
    let part = PossibilityPart::new();

    eval_possibilities_workflow(&part, workflows, &workflows.0["in"])
}

fn main() {
    let input = include_str!("../data/input.txt");
    let (workflow_input, part_input) = input.split_once("\n\n").unwrap();

    let workflows = Workflows::from_str(workflow_input).unwrap();
    let parts = Parts::from_str(part_input).unwrap();

    let sum = parts
        .0
        .iter()
        .filter_map(|part| match is_accepted(part, &workflows) {
            true => Some(part.x + part.m + part.a + part.s),
            false => None,
        })
        .sum::<u32>();
    println!("Sum of rating numbers of each part is {}", sum);

    let mut workflows = workflows;
    workflows
        .0
        .insert("A".to_string(), vec![Next::Else("A".to_string())]);
    workflows
        .0
        .insert("R".to_string(), vec![Next::Else("R".to_string())]);
    let possibilities = eval_possibilities(&workflows);

    let possibility_count = possibilities
        .iter()
        .map(|possibility| possibility.possibilities())
        .sum::<usize>();
    println!(
        "There are {} distinct combinations possible",
        possibility_count
    );
}
