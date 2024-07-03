use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use map::Map;

mod map;

fn main() {
    let input = include_str!("../data/input.txt");
    let map = Map::from_str(input).unwrap();

    let mut visited_fields = HashMap::new();
    visited_fields.insert(map.start, true);
    let mut additional_fields = HashSet::new();
    additional_fields.insert(map.start);

    let steps = 64;

    for step in 1..=steps {
        let new_fields = additional_fields
            .iter()
            .flat_map(|field| {
                map.get_neighbors(&field)
                    .filter(|new_field| !visited_fields.contains_key(new_field))
            })
            .collect::<HashSet<_>>();

        let even_step = step % 2 == 0;

        new_fields.iter().for_each(|coordinate| {
            visited_fields.insert(*coordinate, even_step);
        });

        additional_fields = new_fields;
    }

    let field_count = visited_fields
        .values()
        .filter(|step| **step == (steps % 2 == 0))
        .count();

    println!("There are {} fields.", field_count);
}
