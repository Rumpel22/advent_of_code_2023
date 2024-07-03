use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
enum Tile {
    Plot,
    Rock,
}

enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    fn next(&self, direction: &Direction) -> Self {
        let (x, y) = match direction {
            Direction::North => (self.x, self.y - 1),
            Direction::East => (self.x + 1, self.y),
            Direction::South => (self.x, self.y + 1),
            Direction::West => (self.x - 1, self.y),
        };
        Self { x, y }
    }
}

#[derive(Debug)]
pub struct Map {
    tiles: Vec<Tile>,
    pub start: Coordinate,
    pub width: usize,
    pub height: usize,
}

impl FromStr for Map {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut tiles = input
            .chars()
            .filter_map(|c| match c {
                '.' | 'S' => Some(Tile::Plot),
                '#' => Some(Tile::Rock),
                _ => None,
            })
            .collect::<Vec<_>>();
        let width = input.find(|c| c == '\n').unwrap();
        let height = tiles.len() / width;
        let start_index = input
            .chars()
            .filter(|c| c.is_ascii_graphic())
            .position(|c| c == 'S')
            .unwrap();
        let shift_up = start_index / width;
        let shift_left = start_index % width;
        tiles.rotate_left(shift_up * width);
        for row in 0..height {
            tiles[row * width..(row + 1) * width].rotate_left(shift_left);
        }

        let start = Coordinate { y: 0, x: 0 };
        // let start = Coordinate {
        //     y: (start_index / width) as i32,
        //     x: (start_index % width) as i32,
        // };
        Ok(Map {
            tiles,
            width,
            height,
            start,
        })
    }
}

pub struct NeighbourIterator<'a> {
    field: Coordinate,
    current: u8,
    map: &'a Map,
}

impl<'a> Iterator for NeighbourIterator<'a> {
    type Item = Coordinate;

    fn next(&mut self) -> Option<Self::Item> {
        while self.current < 4 {
            self.current += 1;
            let direction = match self.current {
                1 => Direction::North,
                2 => Direction::East,
                3 => Direction::South,
                4 => Direction::West,
                _ => unreachable!(),
            };
            let new_coordinate = self.field.next(&direction);
            let tile = self.map.get_tile(&new_coordinate);
            match tile {
                Tile::Plot => return Some(new_coordinate),
                Tile::Rock => {}
            }
        }
        None
    }
}

impl Map {
    pub fn get_neighbors(&self, field: &Coordinate) -> NeighbourIterator {
        NeighbourIterator {
            field: *field,
            current: 0,
            map: self,
        }
    }

    fn get_tile(&self, field: &Coordinate) -> Tile {
        let wrapped_field = self.wrap(field);
        // let index = (y * self.width as i32 + x).rem_euclid(self.tiles.len() as i32) as usize;

        let index = (wrapped_field.y * self.width as i32 + wrapped_field.x) as usize;

        self.tiles[index]
    }
    fn wrap(&self, field: &Coordinate) -> Coordinate {
        let x = field.x.rem_euclid(self.width as i32);
        let y = field.y.rem_euclid(self.height as i32);
        Coordinate { x, y }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wrap_tests() {
        let map = Map {
            height: 11,
            width: 9,
            tiles: vec![Tile::Plot; 11 * 9],
            start: Coordinate { x: 0, y: 0 },
        };
        let coordinates = [
            (Coordinate { x: 0, y: 0 }, Coordinate { x: 0, y: 0 }),
            (Coordinate { x: -9, y: 0 }, Coordinate { x: 0, y: 0 }),
            (Coordinate { x: 0, y: -11 }, Coordinate { x: 0, y: 0 }),
            (Coordinate { x: 9, y: 0 }, Coordinate { x: 0, y: 0 }),
            (Coordinate { x: 0, y: 11 }, Coordinate { x: 0, y: 0 }),
            (Coordinate { x: -1, y: 0 }, Coordinate { x: 8, y: 0 }),
            (Coordinate { x: 1, y: 0 }, Coordinate { x: 1, y: 0 }),
            (Coordinate { x: -1, y: -1 }, Coordinate { x: 8, y: 10 }),
        ];

        for (actual, expected) in coordinates {
            let wrapped = map.wrap(&actual);
            assert_eq!(wrapped, expected);
        }
    }
}
