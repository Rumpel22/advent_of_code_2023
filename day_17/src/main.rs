use std::str::FromStr;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct CityMap {
    height: usize,
    width: usize,
    losses: Vec<u16>,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    fn get_manhatten_distance(&self, end: &Coordinate) -> usize {
        self.x.abs_diff(end.x) + self.y.abs_diff(end.y)
    }
}

type Loss = u16;

#[derive(PartialEq, Eq, Clone, Debug)]
struct OpenNode {
    coordinate: Coordinate,
    cumulated_loss: Loss,
    min_loss: Loss,
    last_direction: Option<Direction>,
    direction_count: u8,
}

impl PartialOrd for OpenNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(PartialEq)]
struct ClosedNode {
    coordinate: Coordinate,
    last_direction: Direction,
    direction_count: u8,
}

impl Ord for OpenNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.min_loss.cmp(&self.min_loss)
    }
}

impl From<&OpenNode> for ClosedNode {
    fn from(node: &OpenNode) -> Self {
        ClosedNode {
            coordinate: node.coordinate,
            direction_count: node.direction_count,
            last_direction: node.last_direction.unwrap_or(Direction::Right),
        }
    }
}

impl FromStr for CityMap {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let losses: Vec<_> = s
            .chars()
            .filter(|c| c.is_ascii_digit())
            .map(|c| c.to_digit(10).unwrap() as u16)
            .collect();
        let width = s.chars().position(|c| c == '\n').unwrap();
        let height = losses.len() / width;
        Ok(CityMap {
            height,
            width,
            losses,
        })
    }
}

struct NeighboursIter {
    next_direction: Option<Direction>,
    coordinate: Coordinate,
    max_height: usize,
    max_width: usize,
}

impl Iterator for NeighboursIter {
    type Item = (Coordinate, Direction);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(direction) = self.next_direction {
            match direction {
                Direction::Up => {
                    self.next_direction = Some(Direction::Down);
                    if self.coordinate.y > 0 {
                        return Some((
                            Coordinate {
                                y: self.coordinate.y - 1,
                                ..self.coordinate
                            },
                            direction,
                        ));
                    }
                }
                Direction::Down => {
                    self.next_direction = Some(Direction::Left);
                    if self.coordinate.y < self.max_height - 1 {
                        return Some((
                            Coordinate {
                                y: self.coordinate.y + 1,
                                ..self.coordinate
                            },
                            direction,
                        ));
                    }
                }
                Direction::Left => {
                    self.next_direction = Some(Direction::Right);
                    if self.coordinate.x > 0 {
                        return Some((
                            Coordinate {
                                x: self.coordinate.x - 1,
                                ..self.coordinate
                            },
                            direction,
                        ));
                    }
                }
                Direction::Right => {
                    self.next_direction = None;
                    if self.coordinate.x < self.max_width - 1 {
                        return Some((
                            Coordinate {
                                x: self.coordinate.x + 1,
                                ..self.coordinate
                            },
                            direction,
                        ));
                    }
                }
            }
        }
        None
    }
}

impl CityMap {
    fn get_loss(&self, coordinate: &Coordinate) -> Loss {
        self.losses[self.get_index(coordinate)]
    }

    fn get_index(&self, coordinate: &Coordinate) -> usize {
        coordinate.y * self.width + coordinate.x
    }

    fn get_neighbours(&self, coordinate: &Coordinate) -> NeighboursIter {
        NeighboursIter {
            max_height: self.height,
            max_width: self.width,
            coordinate: *coordinate,
            next_direction: Some(Direction::Up),
        }
    }
}

fn main() {
    let input = include_str!("../data/input.txt");
    let city_map: CityMap = input.parse().unwrap();

    let mut open_list = Vec::new();

    let mut closed_list = Vec::new();

    let start_node = Coordinate { x: 0, y: 0 };
    let end_node = Coordinate {
        x: city_map.width - 1,
        y: city_map.height - 1,
    };

    open_list.push(OpenNode {
        coordinate: start_node,
        cumulated_loss: 0,
        min_loss: start_node.get_manhatten_distance(&end_node) as u16,
        last_direction: None,
        direction_count: 0,
    });

    while let Some(current_node) = open_list.pop() {
        if current_node.coordinate == end_node && current_node.direction_count >= 4 {
            println!("Found path, total loss is {}", current_node.cumulated_loss);
            return;
        }

        closed_list.push((&current_node).into());

        for (successor, direction) in city_map.get_neighbours(&current_node.coordinate) {
            if current_node.last_direction == Some(direction.opposite()) {
                continue;
            }
            if current_node.last_direction == Some(direction) && current_node.direction_count == 10
            {
                continue;
            }
            if current_node.direction_count < 4
                && current_node.last_direction != Some(direction)
                && current_node.last_direction.is_some()
            {
                continue;
            }

            if closed_list.contains(&ClosedNode {
                coordinate: successor,
                last_direction: direction,
                direction_count: current_node.direction_count + 1,
            }) {
                continue;
            }
            let g = city_map.get_loss(&successor) + current_node.cumulated_loss;

            if let Some(list_node) = open_list.iter_mut().find(|list_node| {
                list_node.coordinate == successor
                    && list_node.direction_count == current_node.direction_count + 1
                    && list_node.last_direction == Some(direction)
            }) {
                if list_node.cumulated_loss < g {
                    continue;
                } else {
                    list_node.cumulated_loss = g;
                    if list_node.direction_count > current_node.direction_count {
                        list_node.direction_count = current_node.direction_count;
                    }
                }
            } else {
                let h = successor.get_manhatten_distance(&end_node) as u16;
                let f = h + g;

                open_list.push(OpenNode {
                    coordinate: successor,
                    cumulated_loss: g,
                    min_loss: f,
                    last_direction: Some(direction),
                    direction_count: if current_node.last_direction == Some(direction) {
                        current_node.direction_count + 1
                    } else {
                        1
                    },
                });
            }
        }
        open_list.sort();
    }

    unreachable!()
}
