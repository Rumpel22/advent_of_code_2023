use std::iter::{self};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PatternType {
    Ash,
    Rock,
}

#[derive(Debug)]
struct Pattern {
    width: usize,
    height: usize,
    fields: Vec<PatternType>,
}

#[derive(Debug)]
enum MirrorLine {
    Vertical(usize),
    Horizontal(usize),
}

struct IteratorElement {
    x: usize,
    y: usize,
    t: PatternType,
}

impl Pattern {
    fn get_mirror_line(&self) -> MirrorLine {
        let vertical = (1..self.width)
            .filter(|column| self.vertical_iter(*column).all(|(f1, f2)| f1.t == f2.t))
            .next();
        if let Some(index) = vertical {
            return MirrorLine::Vertical(index);
        }

        let horizontal = (1..self.height)
            .filter(|row| self.horizontal_iter(*row).all(|(f1, f2)| f1.t == f2.t))
            .next();
        if let Some(index) = horizontal {
            return MirrorLine::Horizontal(index);
        }
        unreachable!()
    }

    fn get_smudged_mirror_line(&self) -> MirrorLine {
        let vertical = (1..self.width)
            .filter(|column| {
                self.vertical_iter(*column)
                    .filter(|(f1, f2)| f1.t != f2.t)
                    .count()
                    == 1
            })
            .next();
        if let Some(index) = vertical {
            return MirrorLine::Vertical(index);
        }

        let horizontal = (1..self.height)
            .filter(|row| {
                self.horizontal_iter(*row)
                    .filter(|(f1, f2)| f1.t != f2.t)
                    .count()
                    == 1
            })
            .next();
        if let Some(index) = horizontal {
            return MirrorLine::Horizontal(index);
        }
        unreachable!()
    }

    fn get(&self, x: usize, y: usize) -> Option<PatternType> {
        let index = y * self.width + x;
        self.fields.get(index).copied()
    }

    fn vertical_iter(
        &self,
        column: usize,
    ) -> impl Iterator<Item = (IteratorElement, IteratorElement)> + '_ {
        let start_element1 = IteratorElement {
            x: column - 1,
            y: 0,
            t: self.fields[column - 1],
        };
        let iter1 = iter::successors(Some(start_element1), |element| {
            let mut y = element.y;
            let mut x = element.x;

            y = (y + 1) % self.height;
            if y == 0 {
                if let Some(new_x) = (x).checked_sub(1) {
                    x = new_x;
                } else {
                    return None;
                }
            }
            if let Some(field) = self.get(x, y) {
                Some(IteratorElement { x, y, t: field })
            } else {
                None
            }
        });

        let start_element2 = IteratorElement {
            x: column,
            y: 0,
            t: self.fields[column],
        };
        let iter2 = iter::successors(Some(start_element2), |element| {
            let mut y = element.y;
            let mut x = element.x;

            y = (y + 1) % self.height;
            if y == 0 {
                x += 1;
                if x >= self.width {
                    return None;
                }
            }
            if let Some(field) = self.get(x, y) {
                Some(IteratorElement { x, y, t: field })
            } else {
                None
            }
        });

        iter::zip(iter1, iter2)
    }

    fn horizontal_iter(
        &self,
        row: usize,
    ) -> impl Iterator<Item = (IteratorElement, IteratorElement)> + '_ {
        let start_element1 = IteratorElement {
            x: 0,
            y: row - 1,
            t: self.get(0, row - 1).unwrap(),
        };
        let iter1 = iter::successors(Some(start_element1), |element| {
            let mut y = element.y;
            let mut x = element.x;

            x = (x + 1) % self.width;
            if x == 0 {
                if let Some(new_y) = y.checked_sub(1) {
                    y = new_y;
                } else {
                    return None;
                }
            }
            if let Some(field) = self.get(x, y) {
                Some(IteratorElement { x, y, t: field })
            } else {
                None
            }
        });

        let start_element2 = IteratorElement {
            x: 0,
            y: row,
            t: self.get(0, row).unwrap(),
        };
        let iter2 = iter::successors(Some(start_element2), |element| {
            let mut y = element.y;
            let mut x = element.x;

            x = (x + 1) % self.width;
            if x == 0 {
                y += 1;
                if y >= self.height {
                    return None;
                }
            }
            if let Some(field) = self.get(x, y) {
                Some(IteratorElement { x, y, t: field })
            } else {
                None
            }
        });
        iter::zip(iter1, iter2)
    }
}

impl MirrorLine {
    fn get_value(&self) -> usize {
        match self {
            MirrorLine::Vertical(line) => *line,
            MirrorLine::Horizontal(row) => *row * 100,
        }
    }
}

mod parser {
    use crate::PatternType;

    fn get_pattern(input: &str) -> crate::Pattern {
        let width = input.lines().next().map(|line| line.len()).unwrap();
        let height = input.len() / (width + 1) + 1;
        let fields = input
            .chars()
            .filter(|c| c.is_ascii_graphic())
            .map(|c| match c {
                '.' => PatternType::Ash,
                '#' => PatternType::Rock,
                _ => unreachable!(),
            })
            .collect();
        crate::Pattern {
            width,
            height,
            fields,
        }
    }
    pub(crate) fn parse(input: &str) -> Vec<crate::Pattern> {
        input
            .split_terminator("\n\n")
            .map(|input| get_pattern(input))
            .collect()
    }
}

fn main() {
    let input = include_str!("../data/input.txt");
    let patterns = parser::parse(input);

    let notes_sum = patterns
        .iter()
        // .inspect(|pattern| println!("{:?}", pattern))
        .map(|pattern| pattern.get_mirror_line())
        // .inspect(|mirror_line| println!("{:?}", mirror_line))
        .map(|mirror_line| mirror_line.get_value())
        .sum::<usize>();
    println!("Summerizing all notes results in {}", notes_sum);

    let notes_sum = patterns
        .iter()
        // .inspect(|pattern| println!("{:?}", pattern))
        .map(|pattern| pattern.get_smudged_mirror_line())
        // .inspect(|mirror_line| println!("{:?}", mirror_line))
        .map(|mirror_line| mirror_line.get_value())
        .sum::<usize>();
    println!("Summerizing all notes results in {}", notes_sum);
}
