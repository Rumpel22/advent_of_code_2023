use std::{collections::HashMap, fmt::Display, hash::Hash, str::FromStr, usize};

#[derive(Default, Debug)]
struct Graph {
    nodes: Vec<String>,
    edges: HashMap<Edge, usize>,
}

#[derive(Copy, Clone, Debug, Eq)]
struct Edge {
    start: usize,
    end: usize,
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        (self.start == other.start && self.end == other.end)
            || (self.start == other.end && self.end == other.start)
    }
}

impl Hash for Edge {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        if self.start < self.end {
            self.start.hash(state);
            self.end.hash(state);
        } else {
            self.end.hash(state);
            self.start.hash(state);
        }
    }
}

impl FromStr for Graph {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let key_value_pairs = input.lines().flat_map(|line| {
            let key = &line[..3];
            let values = line[5..].split_ascii_whitespace();
            values.map(move |value| (key, value))
        });
        let mut nodes = vec![];

        let edges = key_value_pairs
            .map(|(a, b)| {
                let start = nodes
                    .iter()
                    .position(|node| *node == a)
                    .or_else(|| {
                        nodes.push(a.to_string());
                        Some(nodes.len() - 1)
                    })
                    .unwrap();
                let end = nodes
                    .iter()
                    .position(|node| *node == b)
                    .or_else(|| {
                        nodes.push(b.to_string());
                        Some(nodes.len() - 1)
                    })
                    .unwrap();
                (Edge { start, end }, 1)
            })
            .collect();

        Ok(Graph { nodes, edges })
    }
}

impl Display for Graph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list()
            .entries(self.edges.iter().map(|(edge, _)| (edge.start, edge.end)))
            .finish()
    }
}

impl Display for Edge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Edge")
            .field(&self.start)
            .field(&self.end)
            .finish()
    }
}
impl Graph {
    fn merge_nodes(&mut self, a: usize, b: usize) {
        let new_node = self.nodes[a].clone() + &self.nodes[b];
        self.nodes.push(new_node);
        let new_index = self.nodes.len() - 1;

        let new_nodes = self
            .edges
            .iter()
            .filter(|(edge, _)| {
                !((edge.start == a && edge.end == b) || (edge.start == b && edge.end == a))
            })
            .filter_map(|(edge, weight)| match (edge.start, edge.end) {
                (start, end) if start == a || start == b => Some((
                    Edge {
                        start: end,
                        end: new_index,
                    },
                    *weight,
                )),
                (start, end) if end == a || end == b => Some((
                    Edge {
                        start,
                        end: new_index,
                    },
                    *weight,
                )),
                _ => None,
            })
            .fold(HashMap::<_, _>::new(), |mut map, (edge, weight)| {
                map.get_mut(&edge)
                    .and_then(|w| {
                        *w += weight;
                        Some(())
                    })
                    .or_else(|| {
                        map.insert(edge, weight);
                        Some(())
                    });
                map
            });

        self.edges.retain(|edge, _| {
            !(edge.start == a || edge.start == b || edge.end == a || edge.end == b)
        });
        self.edges.extend(new_nodes);
    }

    fn is_way_back(&self, node: usize) -> bool {
        self.neighbors(node).next().is_some()
    }

    fn nodes_to_merge(&self) -> (usize, usize) {
        if let Some(connection) = self.edges.iter().filter(|(_, weight)| **weight > 3).next() {
            return (connection.0.start, connection.0.end);
        }

        for node in 0..self.nodes.len() {
            if self.is_way_back(node) {
                return (node, self.neighbors(node).next().unwrap());
            }
        }
        unreachable!("Should never happen");
    }

    fn neighbors(&self, node: usize) -> impl Iterator<Item = usize> + '_ {
        self.edges.iter().filter_map(move |(edge, _)| {
            if edge.start == node {
                return Some(edge.end);
            } else if edge.end == node {
                return Some(edge.start);
            } else {
                return None;
            }
        })
    }

    fn collapse(&mut self) {
        while self.edges.len() > 1 {
            let (node1, node2) = self.nodes_to_merge();
            self.merge_nodes(node1, node2);
        }
    }
}

fn main() {
    let input = include_str!("../data/demo_input.txt");
    let mut graph = Graph::from_str(input).unwrap();
    // println!("{:?}", graph);

    graph.collapse();
    assert!(graph.edges.len() == 1);
    let (edge, weight) = graph.edges.iter().next().unwrap();
    assert!(*weight == 3);

    let node1 = &graph.nodes[edge.start];
    let node2 = &graph.nodes[edge.end];

    let node1_size = node1.len() / 3;
    let node2_size = node2.len() / 3;

    println!("Node 1 ({}): {}", node1_size, node1);
    println!("Node 2 ({}): {}", node2_size, node2);
    println!("Product: {}", node1_size * node2_size);
}
