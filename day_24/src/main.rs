use core::f64;
use is_close::is_close;
use math_vector::Vector;
use regex::Regex;
use std::{ops::RangeInclusive, str::FromStr};

struct Hailstone<T> {
    position: Vector<T>,
    velocity: Vector<T>,
}

impl Hailstone<i64> {
    fn as_f64s(&self) -> Hailstone<f64> {
        Hailstone {
            position: self.position.as_f64s(),
            velocity: self.velocity.as_f64s(),
        }
    }
}

impl FromStr for Hailstone<i64> {
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
            position: Vector::new(x, y, z),
            velocity: Vector::new(vx, vy, vz),
        })
    }
}

fn parse_hailstones(input: &str) -> Vec<Hailstone<i64>> {
    input
        .lines()
        .map(|line| Hailstone::from_str(line).unwrap())
        .collect::<Vec<_>>()
}

fn trajectories_intersect(
    h1: &Hailstone<f64>,
    h2: &Hailstone<f64>,
    range: &RangeInclusive<f64>,
) -> Option<Vector<f64>> {
    // y = a*x + b
    let a1 = h1.velocity.y / h1.velocity.x;
    let b1 = h1.position.y - a1 * h1.position.x;
    let a2 = h2.velocity.y / h2.velocity.x;
    let b2 = h2.position.y - a2 * h2.position.x;

    if is_close!(a1, a2) {
        // Parallel (a1==a2), maybe identical (if b1==b2)
        return None;
    }
    let x = (b2 - b1) / (a1 - a2);
    let y = a1 * x + b1;
    if !range.contains(&x) || !range.contains(&y) {
        // Out of area of interest
        return None;
    }
    let t1 = (x - h1.position.x) / h1.velocity.x;
    let t2 = (x - h2.position.x) / h2.velocity.x;
    if t1 < 0.0 || t2 < 0.0 {
        // Crossed in the past
        return None;
    }
    Some(h1.position + h1.velocity * t1)
}

fn count_intersections(hailstones: &[Hailstone<i64>], range: &RangeInclusive<f64>) -> usize {
    hailstones
        .iter()
        .enumerate()
        .flat_map(|(index, h1)| hailstones[index + 1..].iter().map(move |h2| (h1, h2)))
        .filter(|(h1, h2)| trajectories_intersect(&h1.as_f64s(), &h2.as_f64s(), range).is_some())
        .count()
}

fn intersect(h1: &Hailstone<f64>, h2: &Hailstone<f64>) -> bool {
    // r1 + t*v1 == r2 + t*v2, for any t?
    // r2 - r1 == t*(v1 - v2) => (r2 - r1) must be parallel to (v1 - v2)
    let a = h2.position - h1.position;
    let b = h1.velocity - h2.velocity;

    let t = a.x / b.x;
    return is_close!(t * b.y, a.y) && is_close!(t * b.z, a.z);
}

fn find<F>(hailstones: &[Hailstone<i64>], f: F) -> Vec<Vector<i64>>
where
    F: Fn(Vector<i64>) -> Vector<i64>,
{
    use divisors_fixed::Divisors;

    let (a, b) = hailstones
        .iter()
        .enumerate()
        .flat_map(|(index, h1)| hailstones[index + 1..].iter().map(move |h2| (h1, h2)))
        .filter(|(a, b)| f(a.velocity) == f(b.velocity))
        .next()
        .unwrap();

    let p1 = f(a.position).as_f64s().length() as i64;
    let p2 = f(b.position).as_f64s().length() as i64;
    let divisors = ((p2 - p1).abs() as u64).divisors();
    let unit = Vector::<i64>::from([
        f(a.velocity).x.checked_div(f(a.velocity).x).unwrap_or(0),
        f(a.velocity).y.checked_div(f(a.velocity).y).unwrap_or(0),
        f(a.velocity).z.checked_div(f(a.velocity).z).unwrap_or(0),
    ]);

    let v = divisors
        .iter()
        .flat_map(|divisor| [*divisor as i64, -(*divisor as i64)])
        .map(|divisor| unit * divisor + f(a.velocity))
        .collect::<Vec<_>>();
    v
}

fn find_start_velocity(hailstones: &[Hailstone<i64>]) -> Option<Vector<i64>> {
    let v_x = find(hailstones, &Vector::<i64>::abscissa);
    let v_y = find(hailstones, &Vector::<i64>::ordinate);
    let v_z = find(hailstones, &Vector::<i64>::applicate);
    let possible_velocities = v_x
        .iter()
        .flat_map(|x| v_y.iter().map(move |y| x + y))
        .flat_map(|x| v_z.iter().map(move |z| x + *z))
        .collect::<Vec<_>>();

    for v in possible_velocities {
        let v_norm = v.as_f64s().normalize();
        let e_1 = Vector::new(v_norm.y, -v_norm.x, 0.0).normalize();
        assert!(is_close::is_close!(
            Vector::dot(v_norm, e_1),
            0.0,
            abs_tol = 1e-10
        ));
        let e_2 = Vector::cross(v_norm, e_1).normalize();

        let mapped_stones = hailstones
            .iter()
            .map(|h| {
                let mapped_p = e_1 * Vector::dot(e_1, h.position.as_f64s())
                    + e_2 * Vector::dot(e_2, h.position.as_f64s());
                let mapped_v = e_1 * Vector::dot(e_1, h.velocity.as_f64s())
                    + e_2 * Vector::dot(e_2, h.velocity.as_f64s());

                Hailstone {
                    position: mapped_p,
                    velocity: mapped_v,
                }
            })
            .collect::<Vec<_>>();

        let mut pairs = mapped_stones
            .iter()
            .enumerate()
            .flat_map(|(index, h1)| mapped_stones[index + 1..].iter().map(move |h2| (h1, h2)));

        let (h1, h2) = pairs.next().unwrap();
        if let Some(intersection_mapped) = trajectories_intersect(h1, h2, &(f64::MIN..=f64::MAX)) {
            if pairs
                .filter_map(|(h1, h2)| trajectories_intersect(h1, h2, &(f64::MIN..=f64::MAX)))
                .all(|x| {
                    is_close!(x.x, intersection_mapped.x)
                        && is_close!(x.y, intersection_mapped.y)
                        && is_close!(x.z, intersection_mapped.z)
                })
            {
                return Some(v);
            }
        }
    }

    None
}

fn find_start_position(
    hailstones: &[Hailstone<i64>],
    velocity: &Vector<i64>,
) -> Option<Vector<i64>> {
    let h1 = &hailstones[0];
    let h2 = &hailstones[1];
    let v1x = h1.velocity.x;
    let v1y = h1.velocity.y;
    let r1x = h1.position.x;
    let r1y = h1.position.y;

    let v2x = h2.velocity.x;
    let v2y = h2.velocity.y;
    let r2x = h2.position.x;
    let r2y = h2.position.y;
    let vx = velocity.x;
    let vy = velocity.y;

    let t2 = ((v1x - vx) * (r2y - r1y) - (v1y - vy) * (r2x - r1x))
        / ((v2x - vx) * (v1y - vy) - (v2y - vy) * (v1x - vx));
    let p = h2.position + h2.velocity * t2;
    let r = p - velocity * t2;
    Some(r)
}

fn is_valid(stone: &Hailstone<i64>, hailstones: &[Hailstone<i64>]) -> bool {
    hailstones
        .iter()
        .all(|h| intersect(&h.as_f64s(), &stone.as_f64s()))
}

fn main() {
    let input = include_str!("../data/demo_input.txt");
    let hailstones = parse_hailstones(input);

    // let intersections = count_intersections(&hailstones, &(7.0..=27.0));
    let intersections = count_intersections(&hailstones, &(200000000000000.0..=400000000000000.0));
    println!(
        "There are {} possible intersections in the area.",
        intersections
    );

    let velocity = find_start_velocity(&hailstones).unwrap();
    let position = find_start_position(&hailstones, &velocity).unwrap();
    let stone = Hailstone { velocity, position };

    let valid_solution = is_valid(&stone, &hailstones);
    println!("Valid solution: {valid_solution}");

    println!(
        "The stone should be started at {:?} with a velocity of {:?}.",
        position, velocity
    );

    let sum_of_parts = position.x + position.y + position.z;
    println!("The sum of the position parts is {}.", sum_of_parts);
}
