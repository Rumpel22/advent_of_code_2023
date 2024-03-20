#[derive(PartialEq, Clone, Copy)]
enum Tile {
    RoundRock,
    CubeRock,
    Space,
}

#[derive(Clone, Copy)]
enum Direction {
    North,
    West,
    South,
    East,
}

struct Platform {
    width: usize,
    height: usize,
    tiles: Vec<Tile>,
}

impl Platform {
    fn load(&self) -> usize {
        (0..self.height)
            .map(|row| {
                let distance = self.height - row;
                let start_tile = row * self.width;
                let end_tile = start_tile + self.width;
                let rocks_on_row = self.tiles[start_tile..end_tile]
                    .iter()
                    .filter(|tile| tile == &&Tile::RoundRock)
                    .count();

                rocks_on_row * distance
            })
            .sum()
    }

    fn tilt(&self, direction: Direction) -> Self {
        let mut tiles = self.tiles.clone();

        let coordinates = self
            .tiles
            .iter()
            .enumerate()
            .filter(|(_, tile)| tile == &&Tile::RoundRock)
            .map(|(index, _)| (index / self.width, index % self.width));

        for (row, column) in coordinates {
            let mut upper_index = self.get_next_in_direction(row, column, direction);
            let mut actual_index = self.get_index(row, column);

            while tiles[upper_index] == Tile::Space {
                tiles.swap(actual_index, upper_index);
                actual_index = upper_index;
                let row = actual_index / self.width;
                upper_index = row.saturating_sub(1) * self.height + column;
            }
        }

        Platform {
            tiles,
            height: self.height,
            width: self.width,
        }
    }

    fn get_next_in_direction(&self, row: usize, column: usize, direction: Direction) -> usize {
        match direction {
            Direction::North if row > 0 => self.get_index(row - 1, column),
            Direction::West if column > 0 => self.get_index(row, column - 1),
            Direction::South if row < self.height - 1 => self.get_index(row + 1, column),
            Direction::East if column < self.width - 1 => self.get_index(row, column + 1),
            _ => self.get_index(row, column),
        }
    }

    fn get_index(&self, row: usize, column: usize) -> usize {
        row * self.width + column
    }
}

mod parser {
    use crate::{Platform, Tile};

    pub(crate) fn parse(input: &str) -> Platform {
        let tiles: Vec<_> = input
            .chars()
            .filter(|c| c.is_ascii_graphic())
            .map(|c| match c {
                'O' => Tile::RoundRock,
                '#' => Tile::CubeRock,
                '.' => Tile::Space,
                _ => unreachable!(),
            })
            .collect();

        let width = input.chars().position(|c| c == '\n').unwrap();
        let height = tiles.len() / width;
        Platform {
            width,
            height,
            tiles,
        }
    }
}

fn main() {
    let input = include_str!("../data/input.txt");
    let platform = parser::parse(input);

    let tilted_platform = platform.tilt(Direction::North);

    let load = tilted_platform.load();
    println!("The total load is {}", load);
}
