use itertools::{self, Itertools};
use std::{cmp, str::FromStr};

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
enum Card {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
}

#[derive(Debug, Ord, PartialEq, Eq, Clone, Copy, PartialOrd)]
enum Type {
    Five,
    Four,
    FullHouse,
    Three,
    TwoPair,
    Pair,
    HighCard,
}

#[derive(Debug, Clone, Copy, PartialEq, Ord, Eq)]
struct Hand {
    cards: [Card; 5],
    type_: Type,
}

impl FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cards = s
            .chars()
            .map(|c| match c {
                'A' => Card::Ace,
                'K' => Card::King,
                'Q' => Card::Queen,
                'J' => Card::Jack,
                'T' => Card::Ten,
                '9' => Card::Nine,
                '8' => Card::Eight,
                '7' => Card::Seven,
                '6' => Card::Six,
                '5' => Card::Five,
                '4' => Card::Four,
                '3' => Card::Three,
                '2' => Card::Two,
                _ => unreachable!(),
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        Ok(Self {
            cards,
            type_: type_(&cards),
        })
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        match self.type_.partial_cmp(&other.type_) {
            Some(core::cmp::Ordering::Equal) => self.cards.partial_cmp(&other.cards),
            ord => ord,
        }
    }
}

fn type_(cards: &[Card]) -> Type {
    let cards: Vec<_> = cards
        .iter()
        .sorted()
        .dedup_with_count()
        .sorted_by(
            |(count1, card1), (count2, card2)| match count2.cmp(count1) {
                std::cmp::Ordering::Equal => card1.cmp(card2),
                c => c,
            },
        )
        .collect_vec();

    match cards[0] {
        (5, _) => Type::Five,
        (4, _) => Type::Four,
        (3, _) if cards[1].0 == 2 => Type::FullHouse,
        (3, _) => Type::Three,
        (2, _) if cards[1].0 == 2 => Type::TwoPair,
        (2, _) => Type::Pair,
        (1, _) => Type::HighCard,
        _ => unreachable!(),
    }
}

fn main() {
    let input = include_str!("../data/input.txt");
    let hands_bids = input
        .lines()
        .map(|line| {
            (
                line[..5].parse::<Hand>().unwrap(),
                line[6..].parse::<u32>().unwrap(),
            )
        })
        .sorted_by_key(|(hand, _)| *hand)
        .rev()
        .collect::<Vec<_>>();

    let winnings = hands_bids
        .iter()
        .enumerate()
        .map(|(rank, (_, bid))| (rank as u32 + 1) * bid)
        .sum::<u32>();

    println!("The total winnings are {winnings}");
}
