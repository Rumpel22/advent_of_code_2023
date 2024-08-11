use std::collections::{HashMap, HashSet};

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

impl Graph {
    pub(crate) fn longest_path(&self, start: &Coordinate, end: &Coordinate) -> Option<usize> {
        let mut open_arcs = vec![(self.arcs_from(start), 0)];
        let mut longest_path = None;

        while let Some((next_arcs, current_length)) = open_arcs.pop() {
            for next_arc in next_arcs {
                let arc_length = self.arcs.get(&next_arc).unwrap();

                if next_arc.end == *end {
                    longest_path = longest_path.max(Some(current_length + arc_length));
                } else {
                    open_arcs.push((self.arcs_from(&next_arc.end), current_length + arc_length));
                }
            }

            open_arcs.sort_unstable_by(|a, b| a.1.cmp(&b.1));
        }
        longest_path
    }

    fn nodes(&self) -> HashSet<Coordinate> {
        self.arcs
            .keys()
            .flat_map(|arc| [arc.start, arc.end])
            .collect::<HashSet<_>>()
    }

    fn arcs_from(&self, start: &Coordinate) -> Vec<Arc> {
        self.arcs
            .keys()
            .filter(|arc| arc.start == *start)
            .copied()
            .collect()
    }
}
