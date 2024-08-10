#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
pub(crate) struct Coordinate {
    pub(crate) x: usize,
    pub(crate) y: usize,
}

impl Coordinate {
    pub(crate) fn next(&self, direction: &Direction) -> Option<Self> {
        let new = match direction {
            Direction::Up => Coordinate {
                y: self.y.checked_sub(1)?,
                ..*self
            },
            Direction::Down => Coordinate {
                y: self.y + 1,
                ..*self
            },
            Direction::Right => Coordinate {
                x: self.x + 1,
                ..*self
            },
            Direction::Left => Coordinate {
                x: self.x.checked_sub(1)?,
                ..*self
            },
        };
        Some(new)
    }
}

#[derive(PartialEq, Clone, Copy)]
pub(crate) enum Direction {
    Up,
    Down,
    Right,
    Left,
}
