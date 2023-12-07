use std::ops::Range;

use regex::{Match, Regex};

struct Position {
    line: usize,
    col: usize,
}

struct Occupation {
    line: usize,
    range: Range<usize>,
}

impl Occupation {
    fn from_match(m: &Match, line_length: usize) -> Self {
        let line = m.start() / line_length;
        let start = (m.start() - line) % line_length; // "- line" is eliminating the newline characters
        Self {
            line,
            range: start..start + m.len(),
        }
    }
}

struct OccupationNeighborIterator {
    occupation: Occupation,
    last_position: Position,
}

impl Iterator for OccupationNeighborIterator {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        if self.occupation.line == self.last_position.line
            && self.occupation.range.start == self.last_position.col
        {
            // First call
            let new_col = self
                .occupation
                .range
                .start
                .checked_sub(1)
                .unwrap_or(self.occupation.range.start);
            let new_line = self
                .occupation
                .line
                .checked_sub(1)
                .unwrap_or(self.occupation.line);
            let new_position = Position {
                col: new_col,
                line: new_line,
            };
            if new_position == self.last_position {
                // Edge case: The number starts in the top left corner of the field
                Some(Position {
                    col: self.occupation.range.end,
                    line: self.last_position.line,
                })
            } else {
                Some(new_position)
            }
        }
        None
    }
}

enum Thing {
    Character(Position),
    Number(Position, u32),
}

fn main() {
    let input = include_str!("../data/demo_input.txt");

    let line_length = input.find('\n').unwrap();

    let rx = Regex::new(r"(\d+)|([^\.\n])").unwrap();
    let (numbers, characters): (Vec<_>, Vec<_>) = rx
        .captures_iter(input)
        .map(|c| {
            if let Some(number_match) = c.get(1) {
                let value = number_match.as_str().parse::<u32>().unwrap();
                let position = Position::from_match(number_match, line_length);
                Thing::Number(position, value)
            } else {
                let character_match = c.get(2).unwrap();
                let position = Position::from_match(character_match, line_length);
                Thing::Character(position)
            }
        })
        .partition(|thing| matches!(thing, Thing::Number(_, _)));
}
