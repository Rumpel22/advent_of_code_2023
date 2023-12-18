use std::str::FromStr;

#[derive(PartialEq, Copy, Clone)]
enum Pipe {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    Start,
}

struct Map {
    fields: Vec<Pipe>,
    height: usize,
    width: usize,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Coordinate {
    x: usize,
    y: usize,
}

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Coordinate {
    fn get(&self, direction: Direction) -> Option<Coordinate> {
        match direction {
            Direction::Left if self.x > 0 => Some(Coordinate {
                x: self.x - 1,
                ..*self
            }),
            Direction::Right => Some(Coordinate {
                x: self.x + 1,
                ..*self
            }),
            Direction::Up if self.y > 0 => Some(Coordinate {
                y: self.y - 1,
                ..*self
            }),
            Direction::Down => Some(Coordinate {
                y: self.y + 1,
                ..*self
            }),
            _ => None,
        }
    }
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let fields = s
            .lines()
            .flat_map(|line| {
                line.chars()
                    .map(|c| match c {
                        '|' => Pipe::Vertical,
                        '-' => Pipe::Horizontal,
                        'L' => Pipe::NorthEast,
                        'J' => Pipe::NorthWest,
                        '7' => Pipe::SouthWest,
                        'F' => Pipe::SouthEast,
                        '.' => Pipe::Ground,
                        'S' => Pipe::Start,
                        _ => unreachable!(),
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let width = s.find('\n').unwrap();
        let height = fields.len();
        Ok(Map {
            fields,
            width,
            height,
        })
    }
}

impl Map {
    fn get_start(&self) -> Coordinate {
        let index = self
            .fields
            .iter()
            .position(|pipe| pipe == &Pipe::Start)
            .unwrap();
        Coordinate {
            x: index % self.width,
            y: index / self.width,
        }
    }
    fn get(&self, coordinate: Coordinate) -> Option<Pipe> {
        if !(0..self.width).contains(&coordinate.x) || !(0..self.height).contains(&coordinate.y) {
            return None;
        }
        self.fields
            .get(coordinate.y * self.width + coordinate.x)
            .copied()
    }

    fn iter(&self, coordinate: Coordinate, direction: Direction) -> MapWalker {
        MapWalker {
            map: &self,
            coordinate,
            direction,
        }
    }
}

struct MapWalker<'a> {
    coordinate: Coordinate,
    direction: Direction,
    map: &'a Map,
}

impl Iterator for MapWalker<'_> {
    type Item = Coordinate;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(candidate) = self.coordinate.get(self.direction) {
            if let Some(pipe) = self.map.get(candidate) {
                let direction = match pipe {
                    Pipe::Vertical => self.direction,
                    Pipe::Horizontal => self.direction,
                    Pipe::NorthEast if self.direction == Direction::Down => Direction::Right,
                    Pipe::NorthEast => Direction::Up,
                    Pipe::NorthWest if self.direction == Direction::Down => Direction::Left,
                    Pipe::NorthWest => Direction::Up,
                    Pipe::SouthWest if self.direction == Direction::Up => Direction::Left,
                    Pipe::SouthWest => Direction::Down,
                    Pipe::SouthEast if self.direction == Direction::Up => Direction::Right,
                    Pipe::SouthEast => Direction::Down,
                    Pipe::Ground => return None,
                    Pipe::Start => self.direction,
                };
                self.coordinate = candidate;
                self.direction = direction;
                return Some(self.coordinate);
            }
        }
        None
    }
}

fn main() {
    let input = include_str!("../data/input.txt");
    let map = input.parse::<Map>().unwrap();

    let start_position = map.get_start();
    let directions = [
        Direction::Left,
        Direction::Right,
        Direction::Up,
        Direction::Down,
    ];

    let path_fields = directions
        .iter()
        .filter(|direction| {
            if let Some(field) = start_position.get(**direction) {
                if let Some(pipe) = map.get(field) {
                    return match direction {
                        Direction::Left => {
                            pipe == Pipe::Horizontal
                                || pipe == Pipe::NorthEast
                                || pipe == Pipe::SouthEast
                        }
                        Direction::Right => {
                            pipe == Pipe::Horizontal
                                || pipe == Pipe::NorthWest
                                || pipe == Pipe::SouthWest
                        }
                        Direction::Up => {
                            pipe == Pipe::Vertical
                                || pipe == Pipe::SouthWest
                                || pipe == Pipe::SouthEast
                        }
                        Direction::Down => {
                            pipe == Pipe::Vertical
                                || pipe == Pipe::NorthWest
                                || pipe == Pipe::NorthEast
                        }
                    };
                }
            }
            false
        })
        .map(|direction| {
            map.iter(start_position, *direction)
                .take_while(|coordinate| coordinate != &start_position)
                .collect::<Vec<_>>()
        })
        .nth(0)
        .unwrap();

    let steps = path_fields.len() + 1;
    println!(
        "It takes {} steps. The maximum distance is {}.",
        steps,
        steps / 2
    );
}
