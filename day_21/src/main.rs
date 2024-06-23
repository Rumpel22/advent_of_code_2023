use std::{collections::HashSet, str::FromStr};

use map::Map;

mod map;

fn main() {
    let input = include_str!("../data/input.txt");
    let map = Map::from_str(input).unwrap();
    let mut fields = HashSet::new();
    fields.insert(map.start);
    for _ in 0..64 {
        let new_fields = fields
            .iter()
            .flat_map(|field| map.get_neighbors(field))
            .collect::<HashSet<_>>();

        fields = new_fields;
    }

    println!("There are {} fields.", fields.len());
}
