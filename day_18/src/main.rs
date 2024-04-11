use std::{collections::HashSet, iter};

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Command {
    length: u32,
    direction: Direction,
}

impl Command {
    fn from_str(line: &str) -> Self {
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

    fn from_str2(line: &str) -> Self {
        let color = &line.split_ascii_whitespace().nth(2).unwrap()[2..8];
        let length = u32::from_str_radix(&color[..5], 16).unwrap();
        let direction = match color.chars().nth(5).unwrap() {
            '3' => Direction::Up,
            '1' => Direction::Down,
            '2' => Direction::Left,
            '0' => Direction::Right,
            _ => unreachable!(),
        };
        Command { direction, length }
    }
}

#[derive(Debug)]
struct Commands(Vec<Command>);

struct DigPlan(HashSet<(i32, i32)>);

fn execute(commands: &[Command]) -> DigPlan {
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
        .collect::<HashSet<_>>();

    // Find empty field in the plan
    let min_x = fields.iter().map(|(x, _)| *x).min().unwrap();
    let min_y = fields.iter().map(|(_, y)| *y).min().unwrap();
    let x = (min_x..)
        .skip_while(|x| fields.get(&(*x, min_y)).is_none())
        .nth(1)
        .unwrap();
    let start_field = (x, min_y + 1);
    println!("{:?}", start_field);
    assert!(fields.get(&start_field).is_none());

    let mut open_fields = vec![start_field];
    while let Some((x, y)) = open_fields.pop() {
        fields.insert((x, y));
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
        Commands(input.lines().map(Command::from_str).collect())
    }
    fn from_str2(input: &str) -> Self {
        Commands(input.lines().map(Command::from_str2).collect())
    }
}

fn main() {
    let input = include_str!("../data/demo_input.txt");
    let commands = Commands::from_str(input);
    let dig_plan = execute(&commands.0);

    let field_count = dig_plan.0.len();
    println!("There are {} fields in the dig plan", field_count);

    let commands = Commands::from_str2(input);
    // println!("{:?}", commands);
    // // let dig_plan = execute(&commands.0);

    // let field_count = dig_plan.0.len();
    // println!("There are {} fields in the dig plan", field_count);
}
