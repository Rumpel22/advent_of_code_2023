mod coordinate;
mod graph;
mod map;

use graph::Graph;
use map::Map;
use std::str::FromStr;

fn main() {
    let input = include_str!("../data/demo_input.txt");
    let map = Map::from_str(input).unwrap();

    let graph = Graph::from(&map);
    let longest_path = graph.longest_path(&map.start(), &map.goal()).unwrap();

    println!("The longest path has {} steps.", longest_path)
}
