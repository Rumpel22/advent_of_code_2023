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

fn count_energized_fields(contraption: &Contraption, initial_beam: &Beam) -> usize {
    let mut beams = vec![*initial_beam];

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
    energized.len()
}

fn main() {
    let input = include_str!("../data/input.txt");
    let contraption = parse(input);

    let energized_count = count_energized_fields(
        &contraption,
        &Beam {
            direction: Direction::Right,
            position: Coordinate { x: 0, y: 0 },
        },
    );

    println!("The are total {} fields energized.", energized_count);

    let mut beams = vec![];

    let mut down_beams: Vec<_> = (0..contraption.width)
        .map(|x| Beam {
            position: Coordinate { x, y: 0 },
            direction: Direction::Down,
        })
        .collect();
    beams.append(&mut down_beams);

    let mut up_beams: Vec<_> = (0..contraption.width)
        .map(|x| Beam {
            position: Coordinate {
                x,
                y: contraption.height - 1,
            },
            direction: Direction::Up,
        })
        .collect();
    beams.append(&mut up_beams);

    let mut left_beams: Vec<_> = (0..contraption.height)
        .map(|y| Beam {
            position: Coordinate {
                x: contraption.width - 1,
                y,
            },
            direction: Direction::Left,
        })
        .collect();
    beams.append(&mut left_beams);

    let mut right_beams: Vec<_> = (0..contraption.height)
        .map(|y| Beam {
            position: Coordinate { x: 0, y },
            direction: Direction::Right,
        })
        .collect();
    beams.append(&mut right_beams);

    let max_energy = beams
        .iter()
        .map(|initial_beam| count_energized_fields(&contraption, initial_beam))
        .max()
        .unwrap();

    println!("The maximum number energized tiles is {}.", max_energy);
}
