mod part;
mod workflow;

use std::str::FromStr;

use part::Part;

use crate::part::Parts;
use crate::workflow::Workflows;
use workflow::Condition;

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

fn process_part(part: &Part, workflows: &Workflows, state: &str) -> bool {
    let workflow = &workflows.0[state];
    for step in workflow {
        let next = match step {
            workflow::Next::Check(condition, next) => {
                if eval_condition(&condition, part) {
                    next
                } else {
                    continue;
                }
            }
            workflow::Next::Else(next) => next,
        };
        match next.as_str() {
            "A" => return true,
            "R" => return false,
            next_state => return process_part(part, workflows, next_state),
        }
    }
    unreachable!()
}

fn is_accepted(part: &Part, workflows: &Workflows) -> bool {
    process_part(part, workflows, "in")
}

fn main() {
    let input = include_str!("../data/input.txt");
    let (workflow_input, part_input) = input.split_once("\n\n").unwrap();

    let workflows = Workflows::from_str(workflow_input).unwrap();

    let parts = Parts::from_str(part_input).unwrap();

    let sum = parts
        .0
        .iter()
        .filter(|part| is_accepted(part, &workflows))
        .map(|part| part.x + part.m + part.a + part.s)
        .sum::<u32>();
    println!("Sum of rating numbers of each part is {}", sum);
}
