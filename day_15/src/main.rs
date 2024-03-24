use std::collections::HashMap;

#[derive(Debug)]
enum Operation {
    Remove,
    Add(u8),
}

#[derive(Debug)]
struct Step<'a> {
    label: &'a str,
    operation: Operation,
}

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

    let steps = input.split(',').map(|sequence| {
        if let Some(equal) = sequence.chars().position(|c| c == '=') {
            let label = &sequence[..equal];
            let lens = sequence[equal + 1..].parse::<u8>().unwrap();
            Step {
                label,
                operation: Operation::Add(lens),
            }
        } else {
            let label = &sequence[..sequence.len() - 1];
            Step {
                label,
                operation: Operation::Remove,
            }
        }
    });

    let mut boxes: HashMap<u8, Vec<(&str, u8)>> = HashMap::new();
    for step in steps {
        let box_index = hash(step.label);

        if let Some(box_ref) = boxes.get_mut(&box_index) {
            let pos = box_ref.iter().position(|(label, _)| label == &step.label);

            if let Operation::Add(lens) = step.operation {
                match pos {
                    Some(pos) => box_ref[pos] = (step.label, lens),
                    None => box_ref.push((step.label, lens)),
                }
            } else if let Some(pos) = pos {
                box_ref.remove(pos);
            }
        } else if let Operation::Add(lens) = step.operation {
            boxes.insert(box_index, vec![(step.label, lens)]);
        }
    }

    let focusing_power: u32 = boxes
        .iter()
        .flat_map(|(box_index, lenses)| {
            lenses
                .iter()
                .enumerate()
                .map(move |(index, (_, lens))| (*box_index as u32, index as u32, *lens as u32))
        })
        .map(|(box_index, index, lens)| (box_index + 1) * (index + 1) * lens)
        .sum();

    println!("Total focuing power is {}.", focusing_power);
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
