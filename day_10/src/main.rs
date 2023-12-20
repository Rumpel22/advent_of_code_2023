use std::str::FromStr;

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

struct Map<T> {
    fields: Vec<T>,
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

impl FromStr for Map<MapTile> {
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

impl Map<MapTile> {
    fn get_start(&self) -> Coordinate {
        let index = self
            .fields
            .iter()
            .position(|pipe| pipe == &MapTile::Start)
            .unwrap();
        Coordinate {
            x: index % self.width,
            y: index / self.width,
        }
    }
}

impl<T: PartialEq + Copy> Map<T> {
    fn get(&self, coordinate: Coordinate) -> Option<T> {
        if !(0..self.width).contains(&coordinate.x) || !(0..self.height).contains(&coordinate.y) {
            return None;
        }
        self.fields
            .get(coordinate.y * self.width + coordinate.x)
            .copied()
    }

    fn iter(&self, coordinate: Coordinate, direction: Direction) -> MapWalker<T> {
        MapWalker {
            map: &self,
            coordinate,
            direction,
        }
    }
}

struct MapWalker<'a, T> {
    coordinate: Coordinate,
    direction: Direction,
    map: &'a Map<T>,
}

impl Iterator for MapWalker<'_, MapTile> {
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
    let input = include_str!("../data/demo_input4.txt");
    let map = input.parse::<Map<MapTile>>().unwrap();

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
    let min_x = path_fields
        .iter()
        .map(|coordinate| coordinate.x)
        .min()
        .unwrap();
    let max_x = path_fields
        .iter()
        .map(|coordinate| coordinate.x)
        .max()
        .unwrap();

    // println!("There are {count} fields within the loop."); // 1408 too high
}
