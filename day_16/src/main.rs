use std::collections::HashSet;

#[derive(Clone, Copy)]
enum Tiles {
    Space,
    Mirror1,
    Mirror2,
    HorizontalSplitter,
    VerticalSplitter,
}

struct Contraption {
    height: i32,
    width: i32,
    tiles: Vec<Tiles>,
}

impl Contraption {
    fn tile(&self, coordinate: &Coordinate) -> Option<Tiles> {
        if (0..self.width).contains(&coordinate.x) && (0..self.height).contains(&coordinate.y) {
            let index = coordinate.y * self.width + coordinate.x;
            self.tiles.get(index as usize).copied()
        } else {
            None
        }
    }
}

fn parse(input: &str) -> Contraption {
    let width = input.chars().position(|c| c == '\n').unwrap() as i32;
    let tiles: Vec<_> = input
        .chars()
        .filter(|c| c.is_ascii_graphic())
        .map(|c| match c {
            '.' => Tiles::Space,
            '\\' => Tiles::Mirror1,
            '/' => Tiles::Mirror2,
            '-' => Tiles::HorizontalSplitter,
            '|' => Tiles::VerticalSplitter,
            _ => unreachable!(),
        })
        .collect();
    let height = tiles.len() as i32 / width;
    Contraption {
        height,
        width,
        tiles,
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct Coordinate {
    x: i32,
    y: i32,
}

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Beam {
    direction: Direction,
    position: Coordinate,
}

impl Beam {
    fn go_on(&self) -> Self {
        match self.direction {
            Direction::Up => Beam {
                position: Coordinate {
                    x: self.position.x,
                    y: self.position.y - 1,
                },
                ..*self
            },
            Direction::Down => Beam {
                position: Coordinate {
                    x: self.position.x,
                    y: self.position.y + 1,
                },
                ..*self
            },
            Direction::Left => Beam {
                position: Coordinate {
                    x: self.position.x - 1,
                    y: self.position.y,
                },
                ..*self
            },
            Direction::Right => Beam {
                position: Coordinate {
                    x: self.position.x + 1,
                    y: self.position.y,
                },
                ..*self
            },
        }
    }

    fn deflect(&self, direction: Direction) -> Self {
        Self { direction, ..*self }.go_on()
    }
}

fn main() {
    let input = include_str!("../data/input.txt");
    let contraption = parse(input);

    let mut beams = vec![Beam {
        direction: Direction::Right,
        position: Coordinate { x: 0, y: 0 },
    }];

    let mut energized = HashSet::new();
    while let Some(beam) = beams.pop() {
        if let Some(tile) = contraption.tile(&beam.position) {
            if energized.insert(beam) == false {
                continue;
            }
            let mut new_beams = match tile {
                Tiles::Space => vec![beam.go_on()],
                Tiles::Mirror1 => {
                    let new_direction = match beam.direction {
                        Direction::Up => Direction::Left,
                        Direction::Down => Direction::Right,
                        Direction::Left => Direction::Up,
                        Direction::Right => Direction::Down,
                    };
                    vec![beam.deflect(new_direction)]
                }
                Tiles::Mirror2 => {
                    let new_direction = match beam.direction {
                        Direction::Up => Direction::Right,
                        Direction::Down => Direction::Left,
                        Direction::Left => Direction::Down,
                        Direction::Right => Direction::Up,
                    };
                    vec![beam.deflect(new_direction)]
                }
                Tiles::HorizontalSplitter
                    if beam.direction == Direction::Left || beam.direction == Direction::Right =>
                {
                    vec![beam.go_on()]
                }
                Tiles::HorizontalSplitter => vec![
                    beam.deflect(Direction::Left),
                    beam.deflect(Direction::Right),
                ],
                Tiles::VerticalSplitter
                    if beam.direction == Direction::Up || beam.direction == Direction::Down =>
                {
                    vec![beam.go_on()]
                }
                Tiles::VerticalSplitter => {
                    vec![beam.deflect(Direction::Up), beam.deflect(Direction::Down)]
                }
            };
            beams.append(&mut new_beams);
        }
    }

    let energized: HashSet<_> = energized.iter().map(|beam| beam.position).collect();

    println!("The are total {} fields energized.", energized.len());
}
