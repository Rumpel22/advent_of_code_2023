#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Command {
    length: i64,
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
        let length = i64::from_str_radix(&color[..5], 16).unwrap();
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
    fn step(&self, direction: Direction, step: i64) -> Self {
        let (x, y) = match direction {
            Direction::Up => (self.x, self.y + step),
            Direction::Down => (self.x, self.y - step),
            Direction::Left => (self.x - step, self.y),
            Direction::Right => (self.x + step, self.y),
        };
        Coordinate { x, y }
    }
}

#[derive(Default, PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Coordinate {
    x: i64,
    y: i64,
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
    fn length(&self) -> i64 {
        if self.horizontal() {
            self.end.x.abs_diff(self.start.x) as i64
        } else {
            self.end.y.abs_diff(self.start.y) as i64
        }
    }

    fn direction(&self) -> Direction {
        if self.horizontal() {
            if self.end.x > self.start.x {
                Direction::Right
            } else {
                Direction::Left
            }
        } else {
            if self.end.y > self.start.y {
                Direction::Up
            } else {
                Direction::Down
            }
        }
    }
}

#[derive(Debug)]
struct Rectangle {
    height: i64,
    width: i64,
    positive: bool,
}

struct DigPlan {
    rectangles: Vec<Rectangle>,
}
impl DigPlan {
    fn size(&self) -> u64 {
        self.rectangles
            .iter()
            .map(|rectangle| {
                rectangle.height * rectangle.width * if rectangle.positive { 1 } else { -1 }
            })
            .sum::<i64>()
            .abs() as u64
    }
}

fn get_dig_plan(commands: &Commands) -> DigPlan {
    let lines = Lines::from(commands);
    let lines = lines.shift_to_baseline();

    let mut rectangles = vec![];

    for i in 0..lines.0.len() {
        let line = &lines.0[i];
        if !line.horizontal() {
            continue;
        }

        let prev_index = (i + lines.0.len() - 1) % lines.0.len();
        let next_index = (i + 1) % lines.0.len();
        let prev_direction = lines.0[prev_index].direction();
        let next_direction = lines.0[next_index].direction();
        let same_directions = prev_direction == next_direction;
        let y = line.start.y;

        let positive = line.start.x < line.end.x;
        let (width, height) = match (positive, same_directions, prev_direction) {
            (true, true, _) => (line.length(), y + 1),
            (true, false, Direction::Up) => (line.length() + 1, y + 1),
            (true, false, Direction::Down) => (line.length() - 1, y + 1),
            (false, true, _) => (line.length(), y),
            (false, false, Direction::Up) => (line.length() - 1, y),
            (false, false, Direction::Down) => (line.length() + 1, y),
            (_, _, _) => unreachable!(),
        };

        rectangles.push(Rectangle {
            height,
            width,
            positive,
        });
    }

    DigPlan { rectangles }
}

struct Lines(Vec<Line>);

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

impl Lines {
    fn shift_to_baseline(self) -> Self {
        let offset = self
            .0
            .iter()
            .min_by_key(|line| line.start.y)
            .unwrap()
            .start
            .y;
        let new_lines = self
            .0
            .iter()
            .map(|line| Line {
                start: Coordinate {
                    x: line.start.x,
                    y: line.start.y - offset,
                },
                end: Coordinate {
                    x: line.end.x,
                    y: line.end.y - offset,
                },
            })
            .collect();
        Lines(new_lines)
    }
}

fn main() {
    let input = include_str!("../data/input.txt");

    let commands = Commands::from_str(input);
    let dig_plan = get_dig_plan(&commands);
    let field_count = dig_plan.size();
    println!("There are {} fields in the dig plan", field_count);

    // ==============

    let commands = Commands::from_str2(input);

    let dig_plan = get_dig_plan(&commands);
    let field_count = dig_plan.size();
    println!("There are {} fields in the dig plan", field_count);
}
