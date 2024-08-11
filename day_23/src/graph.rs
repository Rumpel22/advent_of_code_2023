use std::collections::HashMap;

use crate::{
    coordinate::{Coordinate, Direction},
    map::{Map, Step},
};

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct Arc {
    start: Coordinate,
    end: Coordinate,
}

pub(crate) struct Graph {
    arcs: HashMap<Arc, usize>,
}

impl From<&Map> for Graph {
    fn from(map: &crate::map::Map) -> Self {
        let mut arcs = HashMap::<Arc, usize>::new();

        let mut start_fields = vec![Step {
            coordinate: map.start(),
            direction: Direction::Down,
        }];

        while let Some(mut step) = start_fields.pop() {
            let start = step.coordinate;
            step.coordinate = step.coordinate.next(&step.direction).unwrap();
            let mut length = 1;

            loop {
                let next_steps = map.next_steps(&step);
                match next_steps.len() {
                    0 => {
                        if step.coordinate == map.goal() {
                            arcs.insert(
                                Arc {
                                    start,
                                    end: step.coordinate,
                                },
                                length,
                            );
                        }
                        break;
                    }
                    1 => {
                        step = next_steps[0];
                        length += 1
                    }
                    2 | 3 => {
                        if let Some(old_arc) = arcs.insert(
                            Arc {
                                start,
                                end: step.coordinate,
                            },
                            length,
                        ) {
                            assert!(old_arc == length);
                        } else {
                            for next_step in next_steps {
                                start_fields.push(Step {
                                    coordinate: step.coordinate,
                                    direction: next_step.direction,
                                });
                            }
                        }

                        break;
                    }
                    _ => unreachable!(),
                };
            }
        }

        Self { arcs }
    }
}

#[derive(Default, Clone)]
struct Path {
    length: usize,
    visited: Vec<Coordinate>,
}

impl Graph {
    pub(crate) fn longest_path(&self, start: &Coordinate, end: &Coordinate) -> Option<usize> {
        let mut open_arcs = vec![(self.arcs_from(start)[0], Path::default())];
        let mut longest_path = Path::default();

        while let Some((next_arc, current_path)) = open_arcs.pop() {
            if current_path.visited.contains(&next_arc.start) {
                continue;
            }

            let arc_length = self.arcs.get(&next_arc).unwrap();
            let mut visited = current_path.visited.to_vec();
            visited.push(next_arc.start);

            let new_path = Path {
                length: arc_length + current_path.length,
                visited,
            };

            if next_arc.end == *end {
                if new_path.length > longest_path.length {
                    longest_path = new_path;
                }
            } else {
                self.arcs_from(&next_arc.end).iter().for_each(|next_arc| {
                    open_arcs.push((*next_arc, new_path.clone()));
                });
            }
        }

        Some(longest_path.length)
    }

    fn arcs_from(&self, start: &Coordinate) -> Vec<Arc> {
        self.arcs
            .keys()
            .filter(|arc| arc.start == *start)
            .copied()
            .collect()
    }
}
