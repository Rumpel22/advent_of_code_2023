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
    if groups.is_empty() {
        if springs.iter().all(|spring| *spring != State::Damaged) {
            return 1;
        } else {
            return 0;
        }
    }

    let group = groups[0];

    let needed_space = groups.iter().sum::<usize>() + groups.len() - 1;
    let last_possible_index = match springs.len().checked_sub(needed_space) {
        Some(value) => springs
            .iter()
            .position(|spring| *spring == State::Damaged)
            .unwrap_or(value)
            .min(value),
        None => return 0,
    };

    (0..=last_possible_index)
        .map(|index| match springs[index] {
            State::Operational => 0,
            _ => {
                if springs[index..index + group]
                    .iter()
                    .all(|spring| *spring != State::Operational)
                    && springs
                        .get(index + group)
                        .map_or(true, |end| *end != State::Damaged)
                {
                    let next_start = index + group + springs.get(index + group).map_or(0, |_| 1);
                    get_combination_count(&springs[next_start..], &groups[1..])
                } else {
                    0
                }
            }
        })
        .sum::<usize>()
}

impl Row {
    fn get_combination_count(&self) -> usize {
        get_combination_count(&self.springs, &self.groups)
    }

    fn get_combination_count_unfolded(&self) -> usize {
        let x = get_combination_count(&self.springs, &self.groups);
        let mut springs = self.springs.clone();
        springs.push(State::Unknown);
        springs.extend_from_slice(&self.springs);
        let mut groups = self.groups.clone();
        groups.extend_from_slice(&self.groups);
        let y = get_combination_count(&springs, &groups);
        let factor = y / x;
        factor.pow(4) * x
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
    let input = include_str!("../data/input.txt");
    let rows = parser::parse(input);

    let combination_count = rows
        .iter()
        .map(|row| row.get_combination_count())
        // .inspect(|count| println!("{count}"))
        .sum::<usize>();
    println!("There are {combination_count} possible arrangements.");

    let combination_count_unfolded = rows
        .iter()
        .map(|row| row.get_combination_count_unfolded())
        // .inspect(|count| println!("{count}"))
        .sum::<usize>();
    println!("There are {combination_count_unfolded} possible arrangements if unfolded 5 times.");
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
