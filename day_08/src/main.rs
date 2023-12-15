use std::{collections::HashMap, str::FromStr};

enum Direction {
    Left,
    Right,
}

type Field<'a> = &'a str;

fn get_map(input: &str) -> HashMap<Field, (Field, Field)> {
    input
        .lines()
        .skip(2)
        .map(|line| {
            let first = &line[..3];
            let left = &line[7..10];
            let right = &line[12..15];
            (first, (left, right))
        })
        .collect()
}

fn main() {
    let input = include_str!("../data/input.txt");

    let directions = input
        .chars()
        .take_while(|c| !c.is_ascii_whitespace())
        .map(|c| match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => unreachable!(),
        })
        .collect::<Vec<_>>();

    let field_map = get_map(input);
    let count = directions
        .iter()
        .cycle()
        .scan("AAA", |current, direction| {
            let next = match direction {
                Direction::Left => field_map[current].0,
                Direction::Right => field_map[current].1,
            };
            *current = next;
            Some(next)
        })
        .take_while(|current| current != &"ZZZ")
        .count()
        + 1;

    println!("It takes {count} steps.");
}
