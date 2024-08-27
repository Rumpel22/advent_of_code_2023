use core::f64;
use is_close::default;
use regex::Regex;
use std::{ops::RangeInclusive, str::FromStr};
use vector3d::Vector3d;

struct Hailstone {
    position: Vector3d<i128>,
    velocity: Vector3d<i128>,
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
    let a1 = h1.velocity.y as f64 / h1.velocity.x as f64;
    let b1 = h1.position.y as f64 - a1 * h1.position.x as f64;
    let a2 = h2.velocity.y as f64 / h2.velocity.x as f64;
    let b2 = h2.position.y as f64 - a2 * h2.position.x as f64;

    if default().is_close(a1, a2) {
        // Parallel (a1==a2), maybe identical (if b1==b2)
        return default().is_close(b1, b2);
    }
    let x = (b2 - b1) / (a1 - a2);
    let y = a1 * x + b1;
    if !range.contains(&x) || !range.contains(&y) {
        // Out of area of interest
        return false;
    }
    let t1 = (x as i128 - h1.position.x) / h1.velocity.x;
    let t2 = (x as i128 - h2.position.x) / h2.velocity.x;
    if t1 < 0 || t2 < 0 {
        // Crossed in the past
        return false;
    }
    true
}

fn count_intersections(hailstones: &[Hailstone], range: &RangeInclusive<f64>) -> usize {
    hailstones
        .iter()
        .enumerate()
        .flat_map(|(index, h1)| hailstones[index + 1..].iter().map(move |h2| (h1, h2)))
        .filter(|(h1, h2)| trajectories_intersect(h1, h2, range))
        .count()
}

fn do_intersect(h1: &Hailstone, h2: &Hailstone) -> bool {
    // r1 + t*v1 == r2 + t*v2, for any t?
    // r2 - r1 == t*(v1 - v2) => (r2 - r1) must be parallel to (v1 - v2)
    // if a||b => a/|a| ° b/|b| == 1 => a°b == |a|*|b|
    let a = h2.position - h1.position;
    let b = h1.velocity - h2.velocity;

    let t = a.x as f64 / b.x as f64;
    default().is_close(t, a.y as f64 / b.y as f64) && default().is_close(t, a.z as f64 / b.z as f64)
    // a / a.norm2() == b / b.norm2()
    // a.dot(b).pow(2) == a.norm2() * b.norm2()
}

fn distance(h1: &Hailstone, h2: &Hailstone) -> f64 {
    let n = h1.velocity.cross(h2.velocity);
    if n == Vector3d::<i128>::default() {
        panic!("asdf");
    }
    (n.dot(h1.position - h2.position)).abs() as f64 / (n.norm2() as f64).sqrt()
}

fn find_start(hailstones: &[Hailstone]) -> (Vector3d<i128>, Vector3d<i128>) {
    let h1 = &hailstones[0];
    let h2 = &hailstones[1];
    let h3 = &hailstones[2];

    let mut d = f64::MAX;
    let mut t1 = 0;
    let mut t2 = 0;

    let mut overshoot = false;
    let mut step = 1;
    let mut change_t1 = true;

    while d != 0.0 {
        *if change_t1 { &mut t1 } else { &mut t2 } += step;

        let p1 = h1.position + h1.velocity * t1;
        let p2 = h2.position + h2.velocity * t2;

        let v = p2 - p1;
        let stone = Hailstone {
            position: p1,
            velocity: v,
        };

        let new_d = distance(&stone, h3);
        println!(
            "t1: {:15}, t2: {:15}, step: {:15}, new_d: {:15}, d:{:20}",
            t1, t2, step, new_d, d
        );

        if new_d < d && t1 >= 0 && t2 >= 0 {
            if !overshoot {
                step *= 2;
                if step.abs() > 10000000 {
                    step = 1;
                    change_t1 = !change_t1;
                    overshoot = false;
                }
            }
            d = new_d;
        } else {
            *if change_t1 { &mut t1 } else { &mut t2 } -= step;

            overshoot = true;
            if step == 1 {
                step = -1;
                overshoot = false;
            } else {
                step /= 2;
                if step == 0 {
                    step = 1;
                    change_t1 = !change_t1;
                    overshoot = false;
                }
            }
        }
    }
    let p1 = h1.position + h1.velocity * t1;
    let p2 = h2.position + h2.velocity * t2;
    let v = p2 - p1;
    let start_position = p1 - v * t1;
    (start_position, v);

    // loop {
    //     let p1 = h1.position + h1.velocity * t1;
    //     let p2 = h2.position + h2.velocity * t2;

    //     let v = p2 - p1;
    //     let stone = Hailstone {
    //         position: p1,
    //         velocity: v,
    //     };
    //     let new_d = distance(&stone, h3);
    //     if new_d == 0.0 {
    //         let start_position = p1 - v * t1;
    //         return (start_position, v);
    //     }
    //     if new_d > d {
    //         change_t1 = !change_t1;
    //     }
    //     let step = (new_d / (new_d - d)) as i128;
    //     let step = step.clamp(-10000000, 10000000);
    //     d = new_d;

    //     if change_t1 {
    //         t1 = 0.max(step + t1);
    //     } else {
    //         t2 = 0.max(step + t2);
    //     }
    // }
    // for t1 in 0.. {
    //     let p1 = h1.position + h1.velocity * t1;
    //     for t2 in 0..1000000 {
    //         let p2 = h2.position + h2.velocity * t2;

    //         let v = p2 - p1;
    //         let stone = Hailstone {
    //             position: p1,
    //             velocity: v,
    //         };
    //         if do_intersect(&stone, &h3) {
    //             let start_position = p1 - v * t1;
    //             return (start_position, v);
    //         }
    //     }
    // }

    // fn d(h1: &Hailstone, h2: &Hailstone) -> f64 {
    //     let n = h1.velocity.cross(h2.velocity);
    //     n.dot(h1.position - h2.position).abs() as f64 / (n.norm2() as f64).sqrt()
    // }

    // fn d_vx(h1: &Hailstone, h2: &Hailstone) -> f64 {
    //     (h1.velocity.z * (h1.position.y - h2.position.y)
    //         - h1.velocity.y * (h1.position.z - h2.position.z)) as f64
    // }
    // fn d_vy(h1: &Hailstone, h2: &Hailstone) -> f64 {
    //     (h1.velocity.x * (h1.position.z - h2.position.z)
    //         - h1.velocity.z * (h1.position.x - h2.position.x)) as f64
    // }
    // fn d_vz(h1: &Hailstone, h2: &Hailstone) -> f64 {
    //     (h1.velocity.y * (h1.position.x - h2.position.x)
    //         - h1.velocity.x * (h1.position.y - h2.position.y)) as f64
    // }
    // fn d_rx(h1: &Hailstone, h2: &Hailstone) -> f64 {
    //     (h1.velocity.z * h2.velocity.y - h1.velocity.y * h2.velocity.z) as f64
    // }
    // fn d_ry(h1: &Hailstone, h2: &Hailstone) -> f64 {
    //     (h1.velocity.x * h2.velocity.z - h1.velocity.z * h2.velocity.x) as f64
    // }
    // fn d_rz(h1: &Hailstone, h2: &Hailstone) -> f64 {
    //     (h1.velocity.y * h2.velocity.x - h1.velocity.x * h2.velocity.y) as f64
    // }

    // let mut v = Vector3d::new(1, 1, 1);
    // let mut p = Vector3d::new(0, 0, 0);
    // loop {
    //     let stone = Hailstone {
    //         position: p,
    //         velocity: v,
    //     };
    //     let dvx = hailstones
    //         .iter()
    //         .map(|h| match d_vx(h, &stone) {
    //             0.0 => 0.0,
    //             x => d(h, &stone) / x,
    //         })
    //         .sum::<f64>() as i128;
    //     let dvy = hailstones
    //         .iter()
    //         .map(|h| match d_vy(h, &stone) {
    //             0.0 => 0.0,
    //             x => d(h, &stone) / x,
    //         })
    //         .sum::<f64>() as i128;
    //     let dvz = hailstones
    //         .iter()
    //         .map(|h| match d_vz(h, &stone) {
    //             0.0 => 0.0,
    //             x => d(h, &stone) / x,
    //         })
    //         .sum::<f64>() as i128;
    //     let drx = hailstones
    //         .iter()
    //         .map(|h| match d_rx(h, &stone) {
    //             0.0 => 0.0,
    //             x => d(h, &stone) / x,
    //         })
    //         .sum::<f64>() as i128;
    //     let dry = hailstones
    //         .iter()
    //         .map(|h| match d_ry(h, &stone) {
    //             0.0 => 0.0,
    //             x => d(h, &stone) / x,
    //         })
    //         .sum::<f64>() as i128;
    //     let drz = hailstones
    //         .iter()
    //         .map(|h| match d_rz(h, &stone) {
    //             0.0 => 0.0,
    //             x => d(h, &stone) / x,
    //         })
    //         .sum::<f64>() as i128;

    //     let d = dvx + dvy + dvz + drx + dry + drz;
    //     if d == 0 {
    //         return (p, v);
    //     }
    //     v.x += -dvx.clamp(-10, 10);
    //     v.y += -dvy.clamp(-10, 10);
    //     v.z += -dvz.clamp(-10, 10);
    //     p.x += -drx / hailstones.len() as i128;
    //     p.y += -dry / hailstones.len() as i128;
    //     p.z += -drz / hailstones.len() as i128;
    // }

    unreachable!();
}

fn is_valid(stone: &Hailstone, hailstones: &[Hailstone]) -> bool {
    hailstones.iter().all(|h| do_intersect(h, &stone))
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

    let (position, velocity) = find_start(&hailstones);

    let stone = Hailstone { position, velocity };
    let valid_solution = is_valid(&stone, &hailstones);
    println!("Valid solution: {valid_solution}");

    println!(
        "The stone should be started at {} with a velocity of {}.",
        position, velocity
    );
    let sum_of_parts = position.x + position.y + position.z;
    println!("The sum of the position parts is {}.", sum_of_parts);
}
