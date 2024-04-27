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

impl Line {
    fn horizontal(&self) -> bool {
        self.start.y == self.end.y
    }
}

struct Rectangle {
    corner_1: Coordinate,
    corner_2: Coordinate,
}

impl Rectangle {
    fn size(&self) -> u64 {
        self.height() * self.width()
    }

    fn width(&self) -> u64 {
        (self.corner_1.x.abs_diff(self.corner_2.x) + 1) as u64
    }
    fn height(&self) -> u64 {
        (self.corner_1.y.abs_diff(self.corner_2.y) + 1) as u64
    }
}

struct DigPlan {
    rectangles: Vec<Rectangle>,
}
impl DigPlan {
    fn size(&self) -> u64 {
        self.rectangles
            .iter()
            .map(|rectangle| rectangle.size())
            .sum()
    }
}

fn get_dig_plan(lines: &Lines) -> DigPlan {
    // Horizontal lines, sorted from top-left to bottom-right
    let mut horizontal_lines: Vec<_> = lines
        .0
        .iter()
        .filter(|line| line.horizontal())
        .map(|line| {
            if line.start.x < line.end.x {
                (line.start.y, line.start.x..=line.end.x)
            } else {
                (line.start.y, line.end.x..=line.start.x)
            }
        })
        .collect();
    horizontal_lines.sort_by(|(y_a, _), (y_b, _)| y_b.cmp(y_a));

    let mut rectangles = Vec::new();

    while let Some((current_y, current_range)) = horizontal_lines.pop() {
        let bottom_rev_index = horizontal_lines
            .iter()
            .rev()
            .position(|(y, range)| {
                y > &current_y
                    && (current_range.contains(range.start())
                        || current_range.contains(range.end()))
            })
            .expect("No bottom line found for current line.");
        let bottom_index = horizontal_lines.len() - bottom_rev_index - 1;
        let (bottom_y, bottom_range) = horizontal_lines.remove(bottom_index);

        if bottom_range.start() == current_range.end() {
            let corner_1 = Coordinate {
                x: *current_range.start(),
                y: current_y,
            };
            let corner_2 = Coordinate {
                x: *current_range.end(),
                y: bottom_y - 1,
            };
            rectangles.push(Rectangle { corner_1, corner_2 });
            let new_line = *current_range.start()..=*bottom_range.end();
            horizontal_lines.push((bottom_y, new_line));
        } else if bottom_range.end() == current_range.start() {
            let corner_1 = Coordinate {
                x: *current_range.start(),
                y: current_y,
            };
            let corner_2 = Coordinate {
                x: *current_range.end(),
                y: bottom_y - 1,
            };
            rectangles.push(Rectangle { corner_1, corner_2 });
            let new_line = *bottom_range.start()..=*current_range.end();
            horizontal_lines.push((bottom_y, new_line));
        } else if bottom_range.end() == current_range.end() {
            let corner_1 = Coordinate {
                x: *current_range.start(),
                y: current_y,
            };
            let corner_2 = Coordinate {
                x: *current_range.end(),
                y: bottom_y,
            };
            rectangles.push(Rectangle { corner_1, corner_2 });

            if current_range != bottom_range {
                let new_line = *current_range.start()..=*bottom_range.start();
                horizontal_lines.push((bottom_y + 1, new_line));
            }
        } else if bottom_range.start() == current_range.start() {
            let corner_1 = Coordinate {
                x: *current_range.start(),
                y: current_y,
            };
            let corner_2 = Coordinate {
                x: *current_range.end(),
                y: bottom_y,
            };
            rectangles.push(Rectangle { corner_1, corner_2 });

            if bottom_range != current_range {
                let new_line = *bottom_range.end()..=*current_range.end();
                horizontal_lines.push((bottom_y + 1, new_line));
            }
        } else {
            let corner_1 = Coordinate {
                x: *current_range.start(),
                y: current_y,
            };
            let corner_2 = Coordinate {
                x: *current_range.end(),
                y: bottom_y,
            };
            rectangles.push(Rectangle { corner_1, corner_2 });
            let new_line1 = *current_range.start()..=*bottom_range.start();
            let new_line2 = *bottom_range.end()..=*current_range.end();
            horizontal_lines.push((bottom_y + 1, new_line1));
            horizontal_lines.push((bottom_y + 1, new_line2));
        }
        horizontal_lines.sort_by(|(y_a, _), (y_b, _)| y_b.cmp(y_a));
    }
    DigPlan { rectangles }
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

        assert!(lines.last().unwrap().end == Coordinate::default());

        Lines(lines)
    }
}

fn main() {
    let input = include_str!("../data/input.txt");

    let commands = Commands::from_str(input);
    let dig_plan = execute(&commands.0);
    let field_count = dig_plan.len();
    println!("There are {} fields in the dig plan", field_count);

    let commands = Commands::from_str2(input);
    let lines = Lines::from(&commands);

    let dig_plan = get_dig_plan(&lines);
    let field_count = dig_plan.size();
    println!("There are {} fields in the dig plan", field_count);
}
