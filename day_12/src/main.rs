#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum State {
    Operational,
    Damaged,
    Unknown,
}

#[derive(Debug)]
pub struct Row {
    springs: Vec<State>,
    groups: Vec<usize>,
}

fn get_combination_count(springs: &[State], groups: &[usize]) -> usize {
    let group = groups[0];

    let needed_space = groups.iter().sum::<usize>() + groups.len() - 1;
    let last_index = springs
        .iter()
        .position(|spring| *spring == State::Damaged)
        .unwrap_or(usize::MAX)
        .min(springs.len() - needed_space);

    (0..=last_index)
        .filter(|index| {
            (*index..*index + group).all(|inner_index| springs[inner_index] != State::Operational)
                && springs.get(index + group).unwrap_or(&State::Operational) != &State::Damaged
        })
        .map(|index| {
            let spring_index = index + group + 1;
            if spring_index >= springs.len() {
                // No more springs
                if groups.len() == 1 {
                    return 1;
                } else {
                    return 0;
                }
            } else if groups.len() > 1 {
                get_combination_count(&springs[spring_index..], &groups[1..])
            } else {
                if springs[spring_index..]
                    .iter()
                    .any(|spring| spring == &State::Damaged)
                {
                    return 0;
                }
                return 1;
            }
        })
        .sum::<usize>()
}

impl Row {
    fn get_combination_count(&self) -> usize {
        get_combination_count(&self.springs, &self.groups)
    }

    fn get_combination_count_unfolded(&self) -> usize {
        let springs = iter::repeat(&self.springs)
            .take(5)
            .interleave_shortest(iter::repeat(&vec![State::Unknown]).take(4))
            .flatten()
            .copied()
            .collect::<Vec<_>>();
        get_combination_count(&springs, &self.groups.repeat(5))
    }
}

mod parser {
    use nom::{
        character::complete::{self, newline, one_of, space1, u32},
        combinator::map,
        multi::{many0, separated_list0},
        sequence::separated_pair,
        IResult, Parser,
    };

    use crate::Row;
    use crate::State;

    fn springs(s: &str) -> IResult<&str, Vec<State>> {
        let states = one_of(".#?").map(|c| match c {
            '.' => State::Operational,
            '#' => State::Damaged,
            '?' => State::Unknown,
            _ => unreachable!(),
        });
        many0(states)(s)
    }

    fn groups(s: &str) -> IResult<&str, Vec<usize>> {
        let usize = map(u32, |number| number as usize);
        separated_list0(complete::char(','), usize)(s)
    }

    pub(crate) fn row(s: &str) -> IResult<&str, Row> {
        map(
            separated_pair(springs, space1, groups),
            |(springs, groups)| Row { springs, groups },
        )(s)
    }

    pub fn parse(s: &str) -> Vec<Row> {
        separated_list0(newline, row)(s).unwrap().1
    }
}

fn main() {
    let input = include_str!("../data/demo_input.txt");
    let rows = parser::parse(input);

    let combination_count = rows
        .iter()
        .map(|row| row.get_combination_count())
        .sum::<usize>();
    println!("There are {combination_count} possible arrangements.");

    // let combination_count_unfolded = rows
    //     .iter()
    //     .map(|row| row.get_combination_count_unfolded())
    //     .inspect(|count| println!("{count}"))
    //     .sum::<usize>();
    // println!("There are {combination_count_unfolded} possible arrangements if unfolded 5 times.");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        let row = parser::row("???.###    1,1,3").unwrap().1;

        assert_eq!(row.get_combination_count_unfolded(), 1);
    }
}
