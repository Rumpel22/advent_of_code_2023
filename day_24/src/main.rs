use itertools::Itertools;
use regex::Regex;
use std::{ops::RangeInclusive, str::FromStr};
use vector3d::Vector3d;

struct Hailstone {
    position: Vector3d<f64>,
    velocity: Vector3d<f64>,
}

impl FromStr for Hailstone {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let rx = Regex::new(r"(-?\d+)").unwrap();
        let mut matches = rx.find_iter(input);

        let x = matches.next().unwrap().as_str().parse().unwrap();
        let y = matches.next().unwrap().as_str().parse().unwrap();
        let z = matches.next().unwrap().as_str().parse().unwrap();
        let vx = matches.next().unwrap().as_str().parse().unwrap();
        let vy = matches.next().unwrap().as_str().parse().unwrap();
        let vz = matches.next().unwrap().as_str().parse().unwrap();

        Ok(Hailstone {
            position: Vector3d::new(x, y, z),
            velocity: Vector3d::new(vx, vy, vz),
        })
    }
}

fn parse_hailstones(input: &str) -> Vec<Hailstone> {
    input
        .lines()
        .map(|line| Hailstone::from_str(line).unwrap())
        .collect::<Vec<_>>()
}

fn trajectories_intersect(h1: &Hailstone, h2: &Hailstone, range: &RangeInclusive<f64>) -> bool {
    // y = a*x + b
    let a1 = h1.velocity.y / h1.velocity.x;
    let b1 = h1.position.y - a1 * h1.position.x;
    let a2 = h2.velocity.y / h2.velocity.x;
    let b2 = h2.position.y - a2 * h2.position.x;

    if a1 == a2 {
        // Parallel (a1==a2), maybe identical (if b1==b2)
        return b1 == b2;
    }
    let x = (b2 - b1) / (a1 - a2);
    let y = a1 * x + b1;
    if !range.contains(&x) || !range.contains(&y) {
        // Out of area of interest
        return false;
    }
    let t1 = (x - h1.position.x) / h1.velocity.x;
    let t2 = (x - h2.position.x) / h2.velocity.x;
    if t1 < 0.0 || t2 < 0.0 {
        // Crossed in the past
        return false;
    }
    true
}

fn count_intersections(hailstones: &[Hailstone], range: &RangeInclusive<f64>) -> usize {
    hailstones
        .iter()
        .combinations(2)
        .filter(|pair| trajectories_intersect(pair[0], pair[1], range))
        .count()
}

fn main() {
    let input = include_str!("../data/input.txt");
    let hailstones = parse_hailstones(input);

    // let intersections = count_intersections(&hailstones, &(7.0..=27.0));
    let intersections = count_intersections(&hailstones, &(200000000000000.0..=400000000000000.0));
    println!(
        "There are {} possible intersections in the area",
        intersections
    );
}
