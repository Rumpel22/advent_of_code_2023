use std::{fmt::Debug, usize};

#[derive(PartialEq, Clone, Copy)]
enum Tile {
    RoundRock,
    CubeRock,
    Space,
}

impl Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RoundRock => write!(f, "O"),
            Self::CubeRock => write!(f, "#"),
            Self::Space => write!(f, "."),
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    North,
    West,
    South,
    East,
}

#[derive(Clone)]
struct Platform {
    width: usize,
    height: usize,
    tiles: Vec<Tile>,
}

impl Debug for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.tiles
            .chunks(self.width)
            .for_each(|x| writeln!(f, "{:?}", x).unwrap());
        Ok(())
    }
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

        let mut coordinates: Vec<_> = self
            .tiles
            .iter()
            .enumerate()
            .filter(|(_, tile)| tile == &&Tile::RoundRock)
            .map(|(index, _)| (index / self.width, index % self.width))
            .collect();

        if direction == Direction::South || direction == Direction::East {
            coordinates.reverse()
        }

        for (row, column) in coordinates {
            let mut next_index = self.get_next_in_direction(row, column, direction);
            let mut actual_index = self.get_index(row, column);

            while tiles[next_index] == Tile::Space {
                tiles.swap(actual_index, next_index);
                actual_index = next_index;
                next_index = self.get_next_index_direction(actual_index, direction);
            }
        }

        Platform {
            tiles,
            height: self.height,
            width: self.width,
        }
    }

    fn spin(&self) -> Self {
        self.tilt(Direction::North)
            .tilt(Direction::West)
            .tilt(Direction::South)
            .tilt(Direction::East)
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

    fn get_next_index_direction(&self, index: usize, direction: Direction) -> usize {
        match direction {
            Direction::North if index >= self.width => index - self.width,
            Direction::West if index % self.width > 0 => index - 1,
            Direction::South if index < self.tiles.len() - self.width => index + self.width,
            Direction::East if index % self.width < self.width - 1 => index + 1,
            _ => index,
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

        let width = input
            .chars()
            .skip_while(|c| !c.is_ascii_graphic())
            .position(|c| c == '\n')
            .unwrap();
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

    let mut spinned_platform = platform;
    let mut past_platforms = vec![];
    for iteration in 0.. {
        let new_platform = spinned_platform.spin();

        if let Some(old_iteration) = past_platforms
            .iter()
            .position(|old_platform: &Platform| old_platform.tiles == new_platform.tiles)
        {
            // println!(
            //     "Iteration {iteration} results in the same pattern as iteration {old_iteration}."
            // );
            let difference = iteration - old_iteration;
            let offset = (1000000000 - old_iteration) % difference;
            let same_platform = old_iteration + offset - 1;
            // println!(
            //     "The 1000000000th platform will look like the {}th",
            //     same_platform
            // );
            spinned_platform = past_platforms.get(same_platform).unwrap().clone();
            break;
        }
        past_platforms.push(new_platform.clone());

        spinned_platform = new_platform;
    }

    let load = spinned_platform.load();
    println!("The total load after spinning is {}", load);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cycle1_test() {
        let input = include_str!("../data/demo_input.txt");
        let platform = parser::parse(input);

        let cycle1 = parser::parse(
            "
            .....#....
            ....#...O#
            ...OO##...
            .OO#......
            .....OOO#.
            .O#...O#.#
            ....O#....
            ......OOOO
            #...O###..
            #..OO#....",
        );
        let spinned_platform = platform.spin();
        assert_eq!(cycle1.tiles, spinned_platform.tiles);

        let cycle2 = parser::parse(
            "
            .....#....
            ....#...O#
            .....##...
            ..O#......
            .....OOO#.
            .O#...O#.#
            ....O#...O
            .......OOO
            #..OO###..
            #.OOO#...O",
        );
        let spinned_platform = spinned_platform.spin();
        assert_eq!(cycle2.tiles, spinned_platform.tiles);

        let cycle3 = parser::parse(
            "
            .....#....
            ....#...O#
            .....##...
            ..O#......
            .....OOO#.
            .O#...O#.#
            ....O#...O
            .......OOO
            #...O###.O
            #.OOO#...O",
        );
        let spinned_platform = spinned_platform.spin();
        assert_eq!(cycle3.tiles, spinned_platform.tiles);
    }
}
