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
    length: i32,
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
        let length = i32::from_str_radix(&color[..5], 16).unwrap();
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

impl Coordinate {
    fn step(&self, direction: Direction, step: i32) -> Self {
        let (x, y) = match direction {
            Direction::Up => (self.x, self.y - step),
            Direction::Down => (self.x, self.y + step),
            Direction::Left => (self.x - step, self.y),
            Direction::Right => (self.x + step, self.y),
        };
        Coordinate { x, y }
    }
    fn next(self, direction: Direction) -> Self {
        self.step(direction, 1)
    }
}

#[derive(Default, PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Coordinate {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Commands(Vec<Command>);

struct Line {
    start: Coordinate,
    end: Coordinate,
}

struct Lines(Vec<Line>);

fn execute(commands: &[Command]) -> HashSet<Coordinate> {
    let mut fields = commands
        .iter()
        .flat_map(|command| iter::repeat(command.direction).take(command.length as usize))
        .scan(Coordinate::default(), |coordinate, direction| {
            *coordinate = coordinate.next(direction);
            Some(*coordinate)
        })
        // .inspect(|coordinate| println!("{:?} ", coordinate))
        .collect::<HashSet<_>>();

    // Find empty field in the plan
    let min_x = fields.iter().map(|coordinate| coordinate.x).min().unwrap();
    let min_y = fields.iter().map(|coordinate| coordinate.y).min().unwrap();
    let mut start_field = Coordinate { x: min_x, y: min_y };
    while fields.get(&start_field).is_none() {
        start_field = start_field.next(Direction::Right);
    }
    // We have found the top-left corner, so the first field within the digplan is 1 field diagonally down-right
    start_field = start_field.next(Direction::Down).next(Direction::Right);
    assert!(fields.get(&start_field).is_none());

    let mut open_fields = vec![start_field];
    while let Some(field) = open_fields.pop() {
        fields.insert(field);
        if fields.get(&field.next(Direction::Right)).is_none() {
            open_fields.push(field.next(Direction::Right));
        }
        if fields.get(&field.next(Direction::Left)).is_none() {
            open_fields.push(field.next(Direction::Left));
        }
        if fields.get(&field.next(Direction::Down)).is_none() {
            open_fields.push(field.next(Direction::Down));
        }
        if fields.get(&field.next(Direction::Up)).is_none() {
            open_fields.push(field.next(Direction::Up));
        }
    }

    fields
}

impl Commands {
    fn from_str(input: &str) -> Self {
        Commands(input.lines().map(Command::from_str).collect())
    }
    fn from_str2(input: &str) -> Self {
        Commands(input.lines().map(Command::from_str2).collect())
    }
}

impl From<&Commands> for Lines {
    fn from(commands: &Commands) -> Self {
        let current = Coordinate::default();
        let lines = commands
            .0
            .iter()
            .scan(Coordinate::default(), |start, command| {
                let end = start.step(command.direction, command.length);
                let line = Line { start: *start, end };
                *start = end;
                Some(line)
            })
            .collect::<Vec<_>>();
        Lines(lines)
    }
}

fn main() {
    let input = include_str!("../data/demo_input.txt");

    let commands = Commands::from_str(input);
    let dig_plan = execute(&commands.0);
    let field_count = dig_plan.len();
    println!("There are {} fields in the dig plan", field_count);

    let commands = Commands::from_str2(input);
    let lines = Lines::from(&commands);
    println!("There are {} fields in the dig plan", field_count);
}
