use std::collections::HashMap;

use num::Integer;

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

fn get_steps(start: &str, directions: &Vec<Direction>, map: &HashMap<&str, (&str, &str)>) -> usize {
    directions
        .iter()
        .cycle()
        .scan(start, |current, direction| {
            let next = match direction {
                Direction::Left => map[current].0,
                Direction::Right => map[current].1,
            };
            *current = next;
            Some(next)
        })
        // It's not the correct end condition for part 1, but it works for part 1 and 2
        .take_while(|current| !current.ends_with("Z"))
        .count()
        + 1
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

    let count = get_steps("AAA", &directions, &field_map);
    println!("It takes {count} steps.");

    let starts = field_map.keys().filter(|key| key.ends_with("A"));
    let counts = starts
        .map(|start| get_steps(start, &directions, &field_map))
        .collect::<Vec<_>>();

    let count = counts
        .iter()
        .copied()
        .reduce(|prev, count| prev.lcm(&count))
        .unwrap();

    println!("As a ghost, it takes {count} steps.");
}
