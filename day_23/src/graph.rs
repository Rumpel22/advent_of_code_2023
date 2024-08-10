use std::collections::{HashMap, HashSet};

use crate::{
    coordinate::{Coordinate, Direction},
    map::{Map, Step},
};

#[derive(PartialEq, Eq, Hash)]
struct Arc {
    start: Coordinate,
    end: Coordinate,
}

pub(crate) struct Graph {
    arcs: HashMap<Arc, usize>,
}

impl From<&Map> for Graph {
    fn from(map: &crate::map::Map) -> Self {
        let mut arcs = HashMap::new();

        let mut open_steps = vec![Step {
            coordinate: map.start(),
            direction: Direction::Down,
        }];

        while let Some(mut step) = open_steps.pop() {
            let start = step.coordinate;
            let mut length = 0;

            loop {
                length += 1;

                let mut next_steps = map.next_steps(&step);
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
                    1 => step = next_steps[0],
                    2 | 3 => {
                        if arcs.insert(
                            Arc {
                                start,
                                end: step.coordinate,
                            },
                            length,
                        ) == None
                        {
                            open_steps.append(&mut next_steps);
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
        self.longest_path_internal(start, end, &self.nodes())
    }

    fn longest_path_internal(
        &self,
        start: &Coordinate,
        end: &Coordinate,
        nodes: &HashSet<Coordinate>,
    ) -> Option<usize> {
        Some(0)
    }

    fn nodes(&self) -> HashSet<Coordinate> {
        self.arcs
            .iter()
            .flat_map(|(arc, _)| [arc.start, arc.end])
            .collect::<HashSet<_>>()
    }

    fn arc_len(&self, start: &Coordinate, end: &Coordinate) -> Option<usize> {
        self.arcs
            .get(&Arc {
                start: *start,
                end: *end,
            })
            .copied()
    }
}
