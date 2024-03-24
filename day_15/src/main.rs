fn hash(input: &str) -> u8 {
    input
        .chars()
        .map(|c| c as u8)
        .fold(0, |current, c| ((current + c as u32) * 17) % 256) as u8
}

fn main() {
    let input = include_str!("../data/input.txt");
    let result: u32 = input.split(',').map(|input| hash(input) as u32).sum();
    println!("The sum of the hashes is {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_entries() {
        let values = [
            ("rn=1", 30),
            ("cm-", 253),
            ("qp=3", 97),
            ("cm=2", 47),
            ("qp-", 14),
            ("pc=4", 180),
            ("ot=9", 9),
            ("ab=5", 197),
            ("pc-", 48),
            ("pc=6", 214),
            ("ot=7", 231),
        ];
        for (input, result) in values {
            let hash_value = hash(input);
            assert_eq!(hash_value, result);
        }
    }
}
