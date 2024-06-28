use std::str::FromStr;

#[derive(Debug)]
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
        let tiles = input
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
        let start = Coordinate {
            y: (start_index / width) as i32,
            x: (start_index % width) as i32,
        };
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
            let new_coordinate = self.map.get_neighbor(&self.field, &direction);
            if new_coordinate.is_some() {
                return new_coordinate;
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

    fn get_neighbor(&self, field: &Coordinate, direction: &Direction) -> Option<Coordinate> {
        let (x, y) = match direction {
            Direction::North => (field.x, field.y - 1),
            Direction::East => (field.x + 1, field.y),
            Direction::South => (field.x, field.y + 1),
            Direction::West => (field.x - 1, field.y),
        };
        let index_x = x.rem_euclid(self.width as i32);
        let index_y = y.rem_euclid(self.height as i32);
        let index = (index_y * self.width as i32 + index_x) as usize;

        match self.tiles[index] {
            Tile::Plot => Some(Coordinate { x, y }),
            Tile::Rock => None,
        }
    }
}
