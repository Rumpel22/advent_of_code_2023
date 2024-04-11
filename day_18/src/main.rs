use std::{collections::HashMap, iter};

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Command {
    length: u8,
    direction: Direction,
}

impl Command {
    fn from_str(line: &'_ str) -> Self {
        let mut iter = line.split_ascii_whitespace();
        let direction = match iter.next().unwrap() {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => unreachable!(),
        };

        let length = iter.next().unwrap().parse().unwrap();

        Command { direction, length }
    }
}

#[derive(Debug)]
struct Commands(Vec<Command>);

struct DigPlan<'a>(HashMap<(i32, i32), &'a str>);

fn execute<'a>(commands: &'a [Command]) -> DigPlan<'a> {
    let mut fields = commands
        .iter()
        .flat_map(|command| iter::repeat(command.direction).take(command.length as usize))
        .scan((0, 0), |(x, y), direction| {
            (*x, *y) = match direction {
                Direction::Up => (*x, *y - 1),
                Direction::Down => (*x, *y + 1),
                Direction::Left => (*x - 1, *y),
                Direction::Right => (*x + 1, *y),
            };
            Some((*x, *y))
        })
        // .inspect(|(x, y)| println!("{} | {}", x, y))
        .map(|(x, y)| ((x, y), ""))
        .collect::<HashMap<_, _>>();

    // Find empty field in the plan
    let min_x = fields.keys().map(|(x, _)| *x).min().unwrap();
    let min_y = fields.keys().map(|(_, y)| *y).min().unwrap();
    let x = (min_x..)
        .skip_while(|x| fields.get(&(*x, min_y)).is_none())
        .skip(1)
        .next()
        .unwrap();
    let start_field = (x, min_y + 1);
    println!("{:?}", start_field);
    assert!(fields.get(&start_field).is_none());

    let mut open_fields = vec![start_field];
    while let Some((x, y)) = open_fields.pop() {
        fields.insert((x, y), "");
        if fields.get(&(x + 1, y)).is_none() {
            open_fields.push((x + 1, y));
        }
        if fields.get(&(x - 1, y)).is_none() {
            open_fields.push((x - 1, y));
        }
        if fields.get(&(x, y + 1)).is_none() {
            open_fields.push((x, y + 1));
        }
        if fields.get(&(x, y - 1)).is_none() {
            open_fields.push((x, y - 1));
        }
    }

    DigPlan(fields)
}

impl Commands {
    fn from_str(input: &str) -> Self {
        Commands(input.lines().map(|line| Command::from_str(line)).collect())
    }
}

fn main() {
    let input = include_str!("../data/input.txt");
    let commands = Commands::from_str(input);
    let dig_plan = execute(&commands.0);

    let field_count = dig_plan.0.len();
    println!("There are {} fields in the dig plan", field_count);
}
