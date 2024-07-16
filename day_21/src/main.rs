use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use map::{Coordinate, Map};

mod map;

fn get_visited_fields(
    map: &Map,
    start: Coordinate,
    steps: usize,
    mut filter: impl FnMut(&Coordinate) -> bool,
) -> (usize, usize) {
    let mut visited_fields = HashMap::new();
    visited_fields.insert(start, true);
    let mut additional_fields = HashSet::new();
    additional_fields.insert(start);

    for step in 1..=steps {
        let new_fields = additional_fields
            .iter()
            .flat_map(|field| {
                map.get_neighbors(&field)
                    .filter(|new_field| !visited_fields.contains_key(new_field))
            })
            .filter(&mut filter)
            .collect::<HashSet<_>>();

        let even_step = step % 2 == 0;

        new_fields.iter().for_each(|coordinate| {
            let x = visited_fields.insert(*coordinate, even_step);
            assert!(x.is_none());
        });

        additional_fields = new_fields;
    }

    let (evens, odds): (Vec<_>, Vec<_>) = visited_fields
        .iter()
        .partition(|(_, even_step)| **even_step);

    let even_count = evens.iter().count();
    let odd_count = odds.iter().count();

    (even_count, odd_count)
}

fn main() {
    let input = include_str!("../data/input.txt");
    let map = Map::from_str(input).unwrap();
    let steps = 64;

    let (even_count, odd_count) =
        get_visited_fields(&map, Coordinate { x: 0, y: 0 }, steps, |_| true);
    let field_count = if steps % 2 == 0 {
        even_count
    } else {
        odd_count
    };

    println!("There are {} fields.", field_count);

    // part II, works only with real input (The provided demo input can be verified with the updated solution of part 1)
    // Solution 1 works also for small step count. This can be used to verify that both solutions return the same result.
    // Some observations by analyzing at the input:
    //   - The start point is in the center of the map
    //   - There are no rock-tiles on the horizontal and vertical lines through the start point
    //   - There are no rock-tiles around the non-repeated map
    //   - Not every plot-tile is reachable!
    //   => The fastest way to go from the start point to a corner of a single map is to go straight in one direction and turn 90° and go straight in that direction. It takes width/2 + height/2 steps. This works for all 4 corners.
    //   => The neighbor (repeated) maps are reached after width/2 or height/2 moves, respectivally. We start their exploration in the middle of one of their borders. After (width + height/2) or (height + width/2) moves, the complete map has been visited
    //   => The repeated maps diagonally of the start map are explored from a corner. To visit all fields on the map, it takes (width + height) moves.
    //   => For all other repeated maps, either of the last two possibilities applies.

    let steps = 26501365;
    if map.height != map.width {
        eprintln!("Solution expects quadratic map.");
        return;
    }

    if (steps - map.width / 2) % map.width != 0 {
        eprintln!("Special condition not fulfilled for this step-count.");
        return;
    }

    let map_half = (map.width / 2) as i16;
    let corners = [
        Coordinate {
            x: -map_half,
            y: map_half,
        },
        Coordinate {
            x: map_half,
            y: map_half,
        },
        Coordinate {
            x: map_half,
            y: -map_half,
        },
        Coordinate {
            x: -map_half,
            y: -map_half,
        },
    ];
    let subtract_corners = corners
        .iter()
        .map(|corner| {
            get_visited_fields(&map, *corner, map.width / 2, |coordinate: &Coordinate| {
                let abs_x = coordinate.x.abs();
                let abs_y = coordinate.y.abs();
                let distance = abs_x + abs_y;
                distance > map_half
                    && abs_x > 0
                    && abs_x <= map_half
                    && abs_y > 0
                    && abs_y <= map_half
            })
        })
        .fold((0, 0), |sum, corner| (sum.0 + corner.0, sum.1 + corner.1));

    let additive_corners = corners
        .iter()
        .map(|corner| {
            let sign_x = corner.x.signum();
            let sign_y = corner.y.signum();
            get_visited_fields(&map, *corner, map.width / 2, |coordinate: &Coordinate| {
                let abs_x = coordinate.x.abs();
                let abs_y = coordinate.y.abs();
                let distance = abs_x + abs_y;
                distance >= map_half
                    && sign_x * coordinate.x >= 0
                    && abs_x <= map_half
                    && sign_y * coordinate.y >= 0
                    && abs_y <= map_half
            })
        })
        .fold((0, 0), |sum, corner| (sum.0 + corner.0, sum.1 + corner.1));

    let in_bounds = |coordinate: &Coordinate| {
        (-map_half..=map_half).contains(&coordinate.x)
            && (-map_half..=map_half).contains(&coordinate.y)
    };

    let (total_fields_even, total_fields_odd) =
        get_visited_fields(&map, Coordinate { x: 0, y: 0 }, map.width, in_bounds);
    let n = (steps - map.width / 2) / map.width;
    let x1 = n.pow(2);
    let x2 = (n + 1).pow(2);

    let total_covered_maps = 2 * n * n + 2 * n + 1;
    assert_eq!(total_covered_maps, x1 + x2);
    let more_evens = n % 2 == 1;

    let total_covered_even_maps = if more_evens { x2 } else { x1 };
    let total_covered_odd_maps = if more_evens { x1 } else { x2 };
    let total_additive_corners = if more_evens {
        additive_corners.1
    } else {
        additive_corners.0
    };
    let total_subtractive_corners = if more_evens {
        subtract_corners.0
    } else {
        subtract_corners.1
    };
    let total_fields = total_covered_even_maps * total_fields_even
        + total_covered_odd_maps * total_fields_odd
        - (n + 1) * total_subtractive_corners
        + n * total_additive_corners;

    println!("There are {} fields.", total_fields);
}
