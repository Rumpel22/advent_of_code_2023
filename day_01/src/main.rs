const WORDS: [(&str, u32); 9] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

fn get_first_digit(line: &str) -> Option<u32> {
    for (word, value) in WORDS {
        if line.starts_with(word) {
            return Some(value);
        }
    }
    if line.chars().next().unwrap().is_ascii_digit() {
        line.chars().next().unwrap().to_digit(10)
    } else {
        get_first_digit(&line[1..])
    }
}

fn get_last_digit(line: &str) -> Option<u32> {
    for (word, value) in WORDS {
        if line.ends_with(word) {
            return Some(value);
        }
    }
    if line.chars().last().unwrap().is_ascii_digit() {
        line.chars().last().unwrap().to_digit(10)
    } else {
        get_last_digit(&line[..line.len() - 1])
    }
}

fn main() {
    let input = include_str!("../data/input.txt");

    let sum = input
        .lines()
        .map(|line| {
            let first_digit = get_first_digit(line).unwrap();
            let last_digit = get_last_digit(line).unwrap();

            first_digit * 10 + last_digit
        })
        .sum::<u32>();

    println!("The sum of the numbers is {sum}.");
}
