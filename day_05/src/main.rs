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

    fn map_range(&self, range: &Range<u64>) -> Vec<Range<u64>> {
        if range.is_empty() {
            return vec![];
        }

        if let Some(entry) = self
            .entries
            .iter()
            .find(|entry| entry.0.contains(&range.start))
        {
            let offset = range.start - entry.0.start;
            let start = entry.1 + offset;

            let entry_length = entry.0.end - entry.0.start;
            let range_length = range.end - range.start;
            let end = start + entry_length.min(range_length);
            let mut new_ranges = vec![start..end];
            new_ranges.append(&mut self.map_range(&(range.start + offset..range.end)));
            return new_ranges;
        }

        // start of range does not match into map
        if let Some(followup_entry) = self
            .entries
            .iter()
            .filter(|entry| entry.0.start > range.start)
            .next()
        {
            let mut new_ranges = vec![range.start..followup_entry.0.start];
            new_ranges.append(&mut self.map_range(&(followup_entry.0.start..range.end)));
            return new_ranges;
        };

        vec![range.clone()]
    }
}

fn get_seeds(s: &str) -> Vec<u64> {
    let first_line = s.lines().next().unwrap();
    first_line[7..]
        .split(' ')
        .map(|s| s.parse::<u64>().unwrap())
        .collect()
}

fn get_seeds_ranges(s: &str) -> Vec<Range<u64>> {
    let numbers = get_seeds(s);
    numbers
        .windows(2)
        .step_by(2)
        .map(|range| range[0]..(range[0] + range[1]))
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
    let input = include_str!("../data/demo_input.txt");
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

    let seeds_ranges = get_seeds_ranges(input);
    let locations = seeds_ranges
        .iter()
        .flat_map(|seeds| seed_to_soil_map.map_range(&seeds))
        .flat_map(|soils| soil_to_fertilizer_map.map_range(&soils))
        .flat_map(|fertilizers| fertilizer_to_water_map.map_range(&fertilizers))
        .flat_map(|waters| water_to_light_map.map_range(&waters))
        .flat_map(|lights| light_to_temperature_map.map_range(&lights))
        .flat_map(|temperatures| temperature_to_humidity_map.map_range(&temperatures))
        .flat_map(|humiditys| humidity_to_location_map.map_range(&humiditys))
        .collect::<Vec<_>>();

    let minimum_location = locations.iter().map(|range| range.start).min().unwrap();
    println!("The minimum location is at {}.", minimum_location);
}
