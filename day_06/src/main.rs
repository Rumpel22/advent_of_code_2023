use std::ops::RangeInclusive;

fn get_time_range((time, distance): &(u64, u64)) -> Option<RangeInclusive<u64>> {
    let time = *time as f64;
    let distance = *distance as f64;

    let discriminant = time * time - 4f64 * distance;
    let range = match discriminant.total_cmp(&0.0) {
        std::cmp::Ordering::Less => None,
        std::cmp::Ordering::Equal => Some((distance / 2.0).floor()..(distance / 2.0).ceil()),
        std::cmp::Ordering::Greater => {
            let lower = (time - discriminant.sqrt()) / 2.0 + 1e-10;
            let upper = (time + discriminant.sqrt()) / 2.0 - 1e-10;
            Some(lower..upper)
        }
    }?;
    Some(range.start.ceil() as u64..=range.end.floor() as u64)
}

fn main() {
    let input = include_str!("../data/input.txt");

    let mut lines = input.lines();
    let time_line = lines
        .next()
        .unwrap()
        .split(' ')
        .filter(|s| !s.is_empty())
        .skip(1);
    let distance_line = lines
        .next()
        .unwrap()
        .split(' ')
        .filter(|s| !s.is_empty())
        .skip(1);
    let time_distances = time_line
        .zip(distance_line)
        .map(|(time, distance)| {
            (
                time.parse::<u64>().unwrap(),
                distance.parse::<u64>().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    let range_lengths = time_distances
        .iter()
        .filter_map(get_time_range)
        .map(|range| range.end() - range.start() + 1)
        .collect::<Vec<_>>();
    let product = range_lengths.iter().product::<u64>();

    println!("The product of the valid solution counts is {product}.");

    let mut lines = input.lines();
    let time_line = lines.next().unwrap().split(':').nth(1).unwrap();
    let distance_line = lines.next().unwrap().split(':').nth(1).unwrap();
    let time: String = time_line
        .chars()
        .filter(|c| !c.is_ascii_whitespace())
        .collect();
    let distance: String = distance_line
        .chars()
        .filter(|c| !c.is_ascii_whitespace())
        .collect();

    let range = get_time_range(&(
        time.parse::<u64>().unwrap(),
        distance.parse::<u64>().unwrap(),
    ))
    .unwrap();
    let range_length = range.end() - range.start() + 1;
    println!("There are {range_length} solutions.");
}
