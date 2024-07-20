use std::{collections::HashSet, ops::RangeInclusive, str::FromStr};

#[derive(Debug)]
struct Cube {
    x: RangeInclusive<i16>,
    y: RangeInclusive<i16>,
    z: RangeInclusive<i16>,

    supported_by: Vec<usize>,
}

impl FromStr for Cube {
    type Err = ();

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut numbers = line
            .split(|c| !char::is_ascii_digit(&c))
            .map(|number| number.parse::<i16>().unwrap());
        let x1 = numbers.next().unwrap();
        let y1 = numbers.next().unwrap();
        let z1 = numbers.next().unwrap();
        let x2 = numbers.next().unwrap();
        let y2 = numbers.next().unwrap();
        let z2 = numbers.next().unwrap();
        assert!(x1 <= x2);
        assert!(y1 <= y2);
        assert!(z1 <= z2);

        Ok(Cube {
            x: x1..=x2,
            y: y1..=y2,
            z: z1..=z2,
            supported_by: vec![],
        })
    }
}

fn intersect<T: std::cmp::PartialOrd>(a: &RangeInclusive<T>, b: &RangeInclusive<T>) -> bool {
    a.end() >= b.start() && b.end() >= a.start()
}

fn main() {
    let input = include_str!("../data/input.txt");
    let mut cubes = input
        .lines()
        .map(|line| Cube::from_str(line).unwrap())
        .collect::<Vec<_>>();
    cubes.sort_unstable_by(|a, b| a.z.start().cmp(b.z.start()));

    let mut fallen_cubes: Vec<Cube> = Vec::new();
    for cube in cubes {
        let supported_by = fallen_cubes
            .iter()
            .enumerate()
            .filter_map(|(index, fallen_cube)| {
                if intersect(&cube.x, &fallen_cube.x) && intersect(&cube.y, &fallen_cube.y) {
                    Some(index)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        let height = supported_by
            .iter()
            .map(|c| fallen_cubes[*c].z.end())
            .max()
            .unwrap_or(&0);

        let new_cube = Cube {
            z: height + 1..=height + cube.z.len() as i16,
            supported_by,
            ..cube
        };
        let supported_by = new_cube
            .supported_by
            .iter()
            .filter(|index| *fallen_cubes[**index].z.end() == (new_cube.z.start() - 1))
            .cloned()
            .collect::<Vec<_>>();
        let new_cube = Cube {
            supported_by,
            ..new_cube
        };
        fallen_cubes.push(new_cube);
    }
    // println!("{:?}", fallen_cubes);

    let needed_cubes = fallen_cubes
        .iter()
        .filter_map(|cube| match cube.supported_by.len() {
            1 => Some(cube.supported_by.clone()),
            _ => None,
        })
        .flatten()
        .collect::<HashSet<_>>();
    let dispensable_cubes_count = fallen_cubes.len() - needed_cubes.len();

    println!(
        "{} cubes can be disintegrated safely.",
        dispensable_cubes_count
    );
}
