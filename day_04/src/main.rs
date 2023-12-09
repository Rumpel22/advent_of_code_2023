use std::str::FromStr;

#[derive(Debug)]
struct Game {
    id: u32,
    winning_numbers: Vec<u32>,
    having_numbers: Vec<u32>,
}

impl Game {
    fn points(&self) -> u32 {
        match self.number_of_wins() as u32 {
            0 => 0,
            x => 2u32.pow(x - 1),
        }
    }

    fn number_of_wins(&self) -> usize {
        self.winning_numbers
            .iter()
            .filter(|winning_number| self.having_numbers.contains(&winning_number))
            .count()
    }
}

impl FromStr for Game {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let id_pos = s.find(':').unwrap();
        let id = s[5..id_pos].trim().parse::<u32>().unwrap();

        let split_pos = s.find('|').unwrap();
        let winning_numbers = s[id_pos + 1..split_pos]
            .split_ascii_whitespace()
            .map(|s| s.trim().parse::<u32>().unwrap())
            .collect::<Vec<_>>();
        let having_numbers = s[split_pos + 1..]
            .split_ascii_whitespace()
            .map(|s| s.trim().parse::<u32>().unwrap())
            .collect::<Vec<_>>();
        Ok(Game {
            id,
            winning_numbers,
            having_numbers,
        })
    }
}

fn main() {
    let input = include_str!("../data/demo_input.txt");
    let games = input
        .lines()
        .map(|line| line.parse::<Game>().unwrap())
        .collect::<Vec<_>>();

    let total_points = games.iter().map(|game| game.points()).sum::<u32>();
    println!("In total, there are {} points.", total_points);
}
