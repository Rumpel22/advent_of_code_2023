use std::str::FromStr;

use itertools::iproduct;

#[derive(Clone)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Clone)]
struct Space {
    galaxies: Vec<Position>,
    height: usize,
    width: usize,
}

impl FromStr for Space {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let width = input.find('\n').unwrap();
        let height = (input.len() + 1) / (width + 1);
        let galaxies = input
            .chars()
            .enumerate()
            .filter_map(|(pos, c)| match c {
                '#' => Some(Position {
                    x: pos % (width + 1),
                    y: pos / (width + 1),
                }),
                _ => None,
            })
            .collect::<Vec<_>>();
        Ok(Space {
            galaxies,
            height,
            width,
        })
    }
}

impl Space {
    fn expand(mut self, expansion: usize) -> Self {
        let empty_rows = self.empty_rows();
        let empty_columns = self.empty_columns();
        let height = self.height + empty_rows.len();
        let width = self.width + empty_columns.len();

        empty_rows.iter().for_each(|row_index| {
            self.galaxies
                .iter_mut()
                .filter(|galaxy| galaxy.y > *row_index)
                .for_each(|galaxy| galaxy.y += expansion)
        });
        empty_columns.iter().for_each(|column_index| {
            self.galaxies
                .iter_mut()
                .filter(|galaxy| galaxy.x > *column_index)
                .for_each(|galaxy| galaxy.x += expansion)
        });
        Self {
            height,
            width,
            ..self
        }
    }

    fn num_galaxies(&self) -> usize {
        self.galaxies.len()
    }

    fn empty_rows(&self) -> Vec<usize> {
        (0..self.height)
            .filter(|column| self.galaxies.iter().all(|galaxy| galaxy.y != *column))
            .rev()
            .collect()
    }
    fn empty_columns(&self) -> Vec<usize> {
        (0..self.width)
            .filter(|column| self.galaxies.iter().all(|galaxy| galaxy.x != *column))
            .rev()
            .collect()
    }

    fn galaxy_distance(&self, idx1: usize, idx2: usize) -> usize {
        let galaxy1 = &self.galaxies[idx1];
        let galaxy2 = &self.galaxies[idx2];
        galaxy1.x.abs_diff(galaxy2.x) + galaxy1.y.abs_diff(galaxy2.y)
    }
}

fn main() {
    let input = include_str!("../data/input.txt");
    let space = input.parse::<Space>().unwrap();
    {
        let space = space.clone().expand(1);

        let sum_distances = iproduct!(0..space.num_galaxies(), 0..space.num_galaxies())
            .filter(|(galaxy_1, galaxy2)| galaxy_1 < galaxy2)
            .map(|(galaxy1, galaxy2)| space.galaxy_distance(galaxy1, galaxy2))
            .sum::<usize>();

        println!("The sum of the distances is {sum_distances:?}.");
    }
    {
        let space = space.clone().expand(1000000 - 1);

        let sum_distances = iproduct!(0..space.num_galaxies(), 0..space.num_galaxies())
            .filter(|(galaxy_1, galaxy2)| galaxy_1 < galaxy2)
            .map(|(galaxy1, galaxy2)| space.galaxy_distance(galaxy1, galaxy2))
            .sum::<usize>();

        println!("The sum of the distances is {sum_distances:?}."); // 857987707407
    }
}
