use crate::coordinate::*;
use std::str::FromStr;

#[derive(PartialEq, Clone, Copy)]
enum Tile {
    Path,
    Forest,
    Slope(Direction),
}

struct PositionNeighbors {
    field: Coordinate,
    step: u8,
    direction: Direction,
}

impl Iterator for PositionNeighbors {
    type Item = Step;

    fn next(&mut self) -> Option<Self::Item> {
        while self.step < 4 {
            self.step += 1;

            let next_direction = match self.step {
                1 if self.direction != Direction::Up => Direction::Down,
                2 if self.direction != Direction::Down => Direction::Up,
                3 if self.direction != Direction::Right => Direction::Left,
                4 if self.direction != Direction::Left => Direction::Right,
                _ => continue,
            };
            if let Some(coordinate) = self.field.next(&next_direction) {
                return Some(Step {
                    coordinate,
                    direction: next_direction,
                });
            }
        }
        None
    }
}
pub(crate) struct Map {
    height: usize,
    width: usize,
    values: Vec<Tile>,
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
                    '<' => Tile::Slope(Direction::Left),
                    '>' => Tile::Slope(Direction::Right),
                    'v' => Tile::Slope(Direction::Down),
                    '^' => Tile::Slope(Direction::Up),
                    _ => unreachable!(),
                })
            })
            .collect::<Vec<_>>();
        let width = input.chars().position(|c| c == '\n').unwrap();
        let height = tiles.len() / width;
        Ok(Map {
            width,
            height,
            values: tiles,
        })
    }
}

impl Map {
    pub(crate) fn start(&self) -> Coordinate {
        let x = self
            .values
            .iter()
            .position(|tile| tile == &Tile::Path)
            .unwrap();
        Coordinate { x, y: 0 }
    }

    pub(crate) fn goal(&self) -> Coordinate {
        let x = self
            .values
            .iter()
            .rposition(|tile| tile == &Tile::Path)
            .unwrap();
        Coordinate {
            x: self.width - (self.values.len() - x),
            y: self.height - 1,
        }
    }
    pub(crate) fn next_steps(&self, step: &Step) -> Vec<Step> {
        if let Tile::Slope(direction) = self.value(&step.coordinate).unwrap() {
            if *direction != step.direction {
                return vec![];
            }
            match step.coordinate.next(direction) {
                Some(coordinate) => vec![Step {
                    direction: *direction,
                    coordinate,
                }],
                None => vec![],
            };
        }

        PositionNeighbors {
            field: step.coordinate,
            step: 0,
            direction: step.direction,
        }
        .into_iter()
        .filter(|step| {
            self.value(&step.coordinate) != Some(&Tile::Forest)
                && self.value(&step.coordinate).is_some()
        })
        .collect::<Vec<_>>()
    }

    fn value(&self, coordinate: &Coordinate) -> Option<&Tile> {
        let index = coordinate.y * self.width + coordinate.x;
        self.values.get(index)
    }
}

#[derive(Clone, Copy)]
pub(crate) struct Step {
    pub(crate) coordinate: Coordinate,
    pub(crate) direction: Direction,
}
