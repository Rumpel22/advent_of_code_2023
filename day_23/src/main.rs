use std::str::FromStr;

#[derive(PartialEq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

#[derive(PartialEq, Clone, Copy)]
enum Tile {
    Path,
    Forest,
    Slop(Direction),
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    fn next(&self, direction: &Direction) -> Option<Self> {
        let new = match direction {
            Direction::Up => Coordinate {
                y: self.y.checked_sub(1)?,
                ..*self
            },
            Direction::Down => Coordinate {
                y: self.y + 1,
                ..*self
            },
            Direction::Right => Coordinate {
                x: self.x + 1,
                ..*self
            },
            Direction::Left => Coordinate {
                x: self.x.checked_sub(1)?,
                ..*self
            },
        };
        Some(new)
    }

    fn next_steps(&self, tile: &Tile) -> PositionNeighbors {
        let fix_direction = match tile {
            Tile::Path => None,
            Tile::Slop(direction) => Some(*direction),
            Tile::Forest => unreachable!(),
        };

        PositionNeighbors {
            field: self,
            step: 0,
            direction: fix_direction,
        }
    }
}

struct PositionNeighbors<'a> {
    field: &'a Coordinate,
    step: u8,
    direction: Option<Direction>,
}

impl Iterator for PositionNeighbors<'_> {
    type Item = Coordinate;

    fn next(&mut self) -> Option<Self::Item> {
        while self.step < 4 {
            self.step += 1;

            if self.direction.is_some() {
                self.step = 4;
                return self.field.next(&self.direction.unwrap());
            }

            let x = match self.step {
                1 => self.field.next(&Direction::Down),
                2 => self.field.next(&Direction::Up),
                3 => self.field.next(&Direction::Left),
                4 => self.field.next(&Direction::Right),
                _ => unreachable!(),
            };
            if x.is_some() {
                return x;
            }
        }
        None
    }
}
struct Map {
    height: usize,
    width: usize,
    tiles: Vec<Tile>,
}

struct Path(Vec<Coordinate>);

impl Path {
    fn at_position(&self, goal: &Coordinate) -> bool {
        self.position() == *goal
    }
    fn len(&self) -> usize {
        self.0.len()
    }
    fn position(&self) -> Coordinate {
        *self.0.last().unwrap()
    }
    fn append(&self, next: &Coordinate) -> Self {
        let mut coords = self.0.clone();
        coords.push(*next);
        Self(coords)
    }
}

impl Path {
    fn new(start: &Coordinate) -> Self {
        let path = vec![*start];
        Self(path)
    }
}

impl FromStr for Map {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let tiles = input
            .lines()
            .flat_map(|line| {
                line.chars().map(|c| match c {
                    '#' => Tile::Forest,
                    '.' => Tile::Path,
                    '<' => Tile::Slop(Direction::Left),
                    '>' => Tile::Slop(Direction::Right),
                    'v' => Tile::Slop(Direction::Down),
                    '^' => Tile::Slop(Direction::Up),
                    _ => unreachable!(),
                })
            })
            .collect::<Vec<_>>();
        let width = input.chars().position(|c| c == '\n').unwrap();
        let height = tiles.len() / width;
        Ok(Map {
            width,
            height,
            tiles,
        })
    }
}

impl Map {
    fn start(&self) -> Coordinate {
        let x = self
            .tiles
            .iter()
            .position(|tile| tile == &Tile::Path)
            .unwrap();
        Coordinate { x, y: 0 }
    }

    fn goal(&self) -> Coordinate {
        let x = self
            .tiles
            .iter()
            .rposition(|tile| tile == &Tile::Path)
            .unwrap();
        Coordinate {
            x: self.width - (self.tiles.len() - x),
            y: self.height - 1,
        }
    }

    fn tile(&self, coordinate: &Coordinate) -> Option<Tile> {
        let index = coordinate.y * self.width + coordinate.x;
        self.tiles.get(index).copied()
    }
}

fn main() {
    let input = include_str!("../data/input.txt");
    let map = Map::from_str(input).unwrap();
    let start = map.start();
    let goal = map.goal();

    let mut finished_paths = vec![];
    let mut paths = vec![Path::new(&start)];
    while let Some(path) = paths.pop() {
        if path.at_position(&goal) {
            finished_paths.push(path);
        } else {
            let position = path.position();
            let current_tile = map.tile(&position).unwrap();
            // let current_tile = Tile::Path;
            for next in position.next_steps(&current_tile) {
                let next_tile = map.tile(&next);
                match next_tile {
                    Some(Tile::Path) | Some(Tile::Slop(_)) => {
                        if !path.0.contains(&next) {
                            let new_path = path.append(&next);
                            paths.push(new_path);
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    let longest_path = finished_paths
        .iter()
        .map(|path| path.len() - 1)
        // .inspect(|len| println!("{}", len))
        .max()
        .unwrap();
    println!("The longest path has {} steps.", longest_path)
}
