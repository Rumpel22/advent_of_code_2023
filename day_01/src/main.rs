fn get_digits(line: &str) -> Vec<u32> {
    line.chars().filter_map(|c| c.to_digit(10)).collect()
}

fn main() {
    let input = include_str!("../data/input.txt");

    let sum = input
        .lines()
        .map(|line| {
            let digits = get_digits(line);
            let first_digit = digits.first().unwrap();
            let second_digit = digits.last().unwrap();

            first_digit * 10 + second_digit
        })
        .sum::<u32>();

    println!("The sum of the arabic numbers is {sum}.");
}
