use itertools::Itertools;
use std::{collections::HashMap, ops::Range, usize};

use regex::Regex;

#[derive(Hash, Clone, Copy, PartialEq, Eq)]
struct Position(usize, usize);

impl Position {
    fn new(index: usize, line_length: usize) -> Position {
        Position(index % line_length, index / line_length)
    }
    fn neighbors(&self) -> Vec<Position> {
        let mut v = vec![
            Position(self.0 + 1, self.1),
            Position(self.0 + 1, self.1 + 1),
            Position(self.0, self.1 + 1),
        ];
        if self.0 > 0 {
            v.push(Position(self.0 - 1, self.1));
            v.push(Position(self.0 - 1, self.1 + 1));
            if self.1 > 0 {
                v.push(Position(self.0 - 1, self.1 - 1));
            }
        }
        if self.1 > 0 {
            v.push(Position(self.0, self.1 - 1));
            v.push(Position(self.0 + 1, self.1 - 1));
        }
        v
    }
    fn index(&self, line_length: usize) -> usize {
        self.1 * line_length + self.0
    }
}

fn neighbors(range: &Range<usize>, line_length: usize) -> Vec<Position> {
    range
        .to_owned()
        .flat_map(|index| Position::new(index, line_length).neighbors())
        .unique()
        .collect()
}

fn main() {
    let input = include_str!("../data/input.txt");
    let line_length = input.find('\n').unwrap() + 1;

    let characters = input
        .chars()
        .enumerate()
        .filter(|(_, c)| !matches!(c, '.' | '0'..='9' | '\n'))
        .map(|(index, c)| (Position::new(index, line_length), c))
        .collect::<HashMap<_, _>>();

    let rx = Regex::new(r"(\d+)").unwrap();
    let numbers = rx
        .captures_iter(input)
        .filter_map(|caputres| {
            caputres
                .get(1)
                .map(|m| (m.range(), m.as_str().parse::<u32>().unwrap()))
        })
        .collect::<Vec<_>>();

    let sum = numbers
        .iter()
        .filter(|(range, _)| {
            neighbors(range, line_length)
                .iter()
                .any(|pos| characters.get(pos).is_some())
        })
        .map(|(_, number)| number)
        .sum::<u32>();

    println!("The sum of the part numbers is {sum}.");

    fn find_number(index: usize, numbers: &[(Range<usize>, u32)]) -> Option<u32> {
        numbers
            .iter()
            .filter_map(|(range, number)| {
                if range.contains(&index) {
                    Some(*number)
                } else {
                    None
                }
            })
            .next()
    }

    fn product(pos: Position, numbers: &[(Range<usize>, u32)], line_length: usize) -> Option<u32> {
        let part_numbers = pos
            .neighbors()
            .iter()
            .filter_map(|pos| {
                let pos_index = pos.index(line_length);
                find_number(pos_index, numbers)
            })
            .unique()
            .collect::<Vec<u32>>();
        if part_numbers.len() < 2 {
            None
        } else {
            Some(part_numbers.iter().product())
        }
    }

    let x = characters
        .iter()
        .filter_map(|(pos, c)| if *c == '*' { Some(pos) } else { None })
        .filter_map(|pos| product(*pos, &numbers, line_length))
        .sum::<u32>();

    println!("{}", x)
}
