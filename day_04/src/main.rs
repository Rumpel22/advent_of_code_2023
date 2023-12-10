use std::{collections::HashMap, str::FromStr};

#[derive(Debug)]
struct Card {
    id: u32,
    winning_numbers: Vec<u32>,
    having_numbers: Vec<u32>,
}

impl Card {
    fn points(&self) -> u32 {
        match self.number_of_wins() as u32 {
            0 => 0,
            x => 2u32.pow(x - 1),
        }
    }

    fn number_of_wins(&self) -> usize {
        self.winning_numbers
            .iter()
            .filter(|winning_number| self.having_numbers.contains(winning_number))
            .count()
    }
}

impl FromStr for Card {
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
        Ok(Card {
            id,
            winning_numbers,
            having_numbers,
        })
    }
}

fn main() {
    let input = include_str!("../data/input.txt");
    let cards = input
        .lines()
        .map(|line| line.parse::<Card>().unwrap())
        .collect::<Vec<_>>();

    let total_points = cards.iter().map(|card| card.points()).sum::<u32>();
    println!("In total, there are {} points.", total_points);

    let max_card_id = cards
        .iter()
        .max_by(|card1, card2| card1.id.cmp(&card2.id))
        .map(|card| card.id)
        .unwrap();

    let mut card_total = cards
        .iter()
        .map(|card| (card.id, 1))
        .collect::<HashMap<_, _>>();

    for card_id in 1..=max_card_id {
        let card = cards.iter().find(|card| card.id == card_id).unwrap();
        let wins = card.number_of_wins();
        let card_amount = *card_total.get(&card_id).unwrap();

        for win in card_id + 1..=card_id + (wins as u32) {
            if let Some(x) = card_total.get_mut(&win) {
                *x += card_amount;
            }
        }
    }

    let total_number_of_cards = card_total.values().sum::<usize>();
    println!(
        "At the end, there are {} scratchcards.",
        total_number_of_cards
    );
}
