use std::ops::Range;

struct Map {
    entries: Vec<(Range<u64>, u64)>,
}

impl Map {
    fn map(&self, input: u64) -> u64 {
        if let Some(entry) = self.entries.iter().find(|entry| entry.0.contains(&input)) {
            let offset = input - entry.0.start;
            entry.1 + offset
        } else {
            input
        }
    }
}

fn get_seeds(s: &str) -> Vec<u64> {
    let first_line = s.lines().next().unwrap();
    first_line[7..]
        .split(' ')
        .map(|s| s.parse::<u64>().unwrap())
        .collect()
}

fn get_map_entry(s: &str) -> (Range<u64>, u64) {
    let mut parts = s.split(' ');
    let to = parts.next().unwrap().parse::<u64>().unwrap();
    let from = parts.next().unwrap().parse::<u64>().unwrap();
    let length = parts.next().unwrap().parse::<u64>().unwrap();
    (from..from + length, to)
}

fn get_map(s: &str, from: &str, to: &str) -> Map {
    let title = format!("{from}-to-{to} map:\n");
    let location = s.find(&title);
    let entries = s[location.unwrap() + title.len()..]
        .lines()
        .take_while(|s| !s.is_empty())
        .map(get_map_entry)
        .collect::<Vec<_>>();
    Map { entries }
}

fn main() {
    let input = include_str!("../data/input.txt");
    let seeds = get_seeds(input);
    let seed_to_soil_map = get_map(input, "seed", "soil");
    let soil_to_fertilizer_map = get_map(input, "soil", "fertilizer");
    let fertilizer_to_water_map = get_map(input, "fertilizer", "water");
    let water_to_light_map = get_map(input, "water", "light");
    let light_to_temperature_map = get_map(input, "light", "temperature");
    let temperature_to_humidity_map = get_map(input, "temperature", "humidity");
    let humidity_to_location_map = get_map(input, "humidity", "location");

    let locations = seeds
        .iter()
        .map(|&seed| seed_to_soil_map.map(seed))
        .map(|soil| soil_to_fertilizer_map.map(soil))
        .map(|fertilizer| fertilizer_to_water_map.map(fertilizer))
        .map(|water| water_to_light_map.map(water))
        .map(|light| light_to_temperature_map.map(light))
        .map(|temperature| temperature_to_humidity_map.map(temperature))
        .map(|humidity| humidity_to_location_map.map(humidity))
        .collect::<Vec<_>>();

    let minimum_location = locations.iter().min().unwrap();
    println!("The minimum location is at {}.", minimum_location);
}
