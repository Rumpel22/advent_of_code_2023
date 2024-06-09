use std::{iter::once, str::FromStr};

#[derive(PartialEq, Copy, Clone)]
enum MapTile {
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
    fields: Vec<MapTile>,
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
                        '|' => MapTile::Vertical,
                        '-' => MapTile::Horizontal,
                        'L' => MapTile::NorthEast,
                        'J' => MapTile::NorthWest,
                        '7' => MapTile::SouthWest,
                        'F' => MapTile::SouthEast,
                        '.' => MapTile::Ground,
                        'S' => MapTile::Start,
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
    fn get_start(&self) -> (Coordinate, Direction, MapTile) {
        let index = self
            .fields
            .iter()
            .position(|pipe| pipe == &MapTile::Start)
            .unwrap();
        let start_coordinate = Coordinate {
            x: index % self.width,
            y: index / self.width,
        };

        let directions = [
            Direction::Left,
            Direction::Right,
            Direction::Up,
            Direction::Down,
        ];

        let start_directions = directions
            .iter()
            .filter(|direction| {
                if let Some(field) = start_coordinate.get(**direction) {
                    if let Some(pipe) = self.get(field) {
                        return match direction {
                            Direction::Left => {
                                pipe == MapTile::Horizontal
                                    || pipe == MapTile::NorthEast
                                    || pipe == MapTile::SouthEast
                            }
                            Direction::Right => {
                                pipe == MapTile::Horizontal
                                    || pipe == MapTile::NorthWest
                                    || pipe == MapTile::SouthWest
                            }
                            Direction::Up => {
                                pipe == MapTile::Vertical
                                    || pipe == MapTile::SouthWest
                                    || pipe == MapTile::SouthEast
                            }
                            Direction::Down => {
                                pipe == MapTile::Vertical
                                    || pipe == MapTile::NorthWest
                                    || pipe == MapTile::NorthEast
                            }
                        };
                    }
                }
                false
            })
            .collect::<Vec<_>>();
        let pipe = start_directions
            .windows(2)
            .map(|x| match x {
                [Direction::Left, Direction::Up] => MapTile::NorthWest,
                [Direction::Left, Direction::Down] => MapTile::SouthWest,
                [Direction::Right, Direction::Up] => MapTile::NorthEast,
                [Direction::Right, Direction::Down] => MapTile::SouthEast,
                _ => unreachable!(),
            })
            .nth(0)
            .unwrap();

        let start_direction = start_directions[0];

        (start_coordinate, *start_direction, pipe)
    }

    fn get(&self, coordinate: Coordinate) -> Option<MapTile> {
        if !(0..self.width).contains(&coordinate.x) || !(0..self.height).contains(&coordinate.y) {
            return None;
        }
        self.fields
            .get(coordinate.y * self.width + coordinate.x)
            .copied()
    }

    fn iter(&self, coordinate: Coordinate, direction: Direction) -> MapWalker {
        MapWalker {
            map: self,
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
                    MapTile::Vertical => self.direction,
                    MapTile::Horizontal => self.direction,
                    MapTile::NorthEast if self.direction == Direction::Down => Direction::Right,
                    MapTile::NorthEast => Direction::Up,
                    MapTile::NorthWest if self.direction == Direction::Down => Direction::Left,
                    MapTile::NorthWest => Direction::Up,
                    MapTile::SouthWest if self.direction == Direction::Up => Direction::Left,
                    MapTile::SouthWest => Direction::Down,
                    MapTile::SouthEast if self.direction == Direction::Up => Direction::Right,
                    MapTile::SouthEast => Direction::Down,
                    MapTile::Ground => return None,
                    MapTile::Start => self.direction,
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

    let (start_position, start_direction, start_pipe) = map.get_start();

    let path_fields = map
        .iter(start_position, start_direction)
        .take_while(|coordinate| coordinate != &start_position)
        .chain(once(start_position))
        .collect::<Vec<_>>();

    let steps = path_fields.len();
    println!(
        "It takes {} steps. The maximum distance is {}.",
        steps,
        steps / 2
    );
    let min_y = path_fields
        .iter()
        .map(|coordinate| coordinate.y)
        .min()
        .unwrap();
    let max_y = path_fields
        .iter()
        .map(|coordinate| coordinate.y)
        .max()
        .unwrap();

    let count = (min_y..=max_y)
        .map(|y| {
            let min_x = path_fields
                .iter()
                .filter(|coordinate| coordinate.y == y)
                .map(|coordinate| coordinate.x)
                .min()
                .unwrap();
            let max_x = path_fields
                .iter()
                .filter(|coordinate| coordinate.y == y)
                .map(|coordinate| coordinate.x)
                .max()
                .unwrap();

            (min_x..=max_x)
                .map(move |x| Coordinate { x, y })
                .scan((false, None), |(inside, downwards), coordinate| {
                    let mut map_tile = map.get(coordinate).unwrap();
                    if map_tile == MapTile::Start {
                        map_tile = start_pipe;
                    }

                    if path_fields.contains(&coordinate) {
                        match map_tile {
                            MapTile::Vertical => *inside = !*inside,
                            MapTile::NorthEast => *downwards = Some(true),
                            MapTile::SouthEast => *downwards = Some(false),
                            MapTile::SouthWest => {
                                if *downwards == Some(true) {
                                    *inside = !*inside;
                                }
                                *downwards = None;
                            }
                            MapTile::NorthWest => {
                                if *downwards == Some(false) {
                                    *inside = !*inside;
                                }
                                *downwards = None;
                            }
                            MapTile::Horizontal => (),
                            MapTile::Ground => unreachable!(),
                            MapTile::Start => unreachable!(),
                        }
                    } else if *inside {
                        return Some(true);
                    }
                    Some(false)
                })
                .filter(|is_enclosed| *is_enclosed)
                .count()
        })
        .sum::<usize>();

    println!("There are {count} fields within the loop.");
}
