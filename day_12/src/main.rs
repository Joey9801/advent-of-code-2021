use std::{collections::HashMap, time::Instant};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct NodeIdx(usize);

impl NodeIdx {
    const fn mask(&self) -> u16 {
        1 << self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct NodeSet(u16);

impl NodeSet {
    fn new_empty() -> Self {
        Self(0)
    }

    fn add(&mut self, node: NodeIdx) {
        self.0 |= node.mask();
    }

    fn remove(&mut self, node: NodeIdx) {
        self.0 &= !node.mask();
    }

    fn remove_all(&mut self, other_set: NodeSet) {
        self.0 &= !other_set.0;
    }

    fn any(&self) -> bool {
        self.0 != 0
    }

    fn contains(&self, node: NodeIdx) -> bool {
        self.0 & node.mask() != 0
    }
}

impl Iterator for NodeSet {
    type Item = NodeIdx;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0.trailing_zeros() {
            16 => None,
            idx => {
                let idx = NodeIdx(idx as usize);
                self.remove(idx);
                Some(idx)
            }
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Node {
    name: String,
    is_small: bool,
    idx: NodeIdx,
}

#[derive(Clone, Debug)]
struct Map {
    nodes: Vec<Node>,

    // Parallel vec to nodes, element N is the set of nodes connected to element N
    edges: Vec<NodeSet>,
}

impl Map {
    fn find_name(&self, name: &str) -> Option<NodeIdx> {
        self.nodes
            .iter()
            .position(|n| n.name == name)
            .map(|idx| NodeIdx(idx))
    }

    fn start(&self) -> NodeIdx {
        self.find_name("start")
            .expect("Expected there to be a node named \"start\"")
    }

    fn end(&self) -> NodeIdx {
        self.find_name("end")
            .expect("Expected there to be a node named \"end\"")
    }
}

fn input() -> Map {
    const INPUT_STR: &'static str = include_str!("../input.txt");

    let mut names = HashMap::new();
    let mut map = Map {
        nodes: Vec::new(),
        edges: Vec::new(),
    };

    for line in INPUT_STR.lines() {
        for name in line.split("-") {
            if !names.contains_key(name) {
                let idx = NodeIdx(names.len());
                names.insert(name.to_string(), idx);
                map.nodes.push(Node {
                    name: name.to_string(),
                    is_small: name.chars().next().unwrap().is_ascii_lowercase(),
                    idx,
                });
                map.edges.push(NodeSet::new_empty());
            }
        }
    }

    for line in INPUT_STR.lines() {
        let mut parts = line.split("-");
        let a = names[parts.next().unwrap()];
        let b = names[parts.next().unwrap()];

        map.edges[a.0].add(b);
        map.edges[b.0].add(a);
    }

    map
}

fn part_1(map: &Map) -> i32 {
    let start = map.start();
    let end = map.end();

    let mut path_count = 0;

    let mut path = vec![start];
    let mut to_visit = vec![map.edges[start.0]];
    let mut smalls_visited = NodeSet::new_empty();
    smalls_visited.add(start);

    while let Some(path_head) = path.last().cloned() {
        if path_head == end {
            path_count += 1;

            // Don't consider any extensions to the path
            path.pop();
            to_visit.pop();
            smalls_visited.remove(path_head);
            continue;
        }

        match to_visit.last_mut().unwrap().next() {
            Some(node) => {
                path.push(node);
                if map.nodes[node.0].is_small {
                    smalls_visited.add(node);
                }
                let mut to_visit_set = map.edges[node.0];
                to_visit_set.remove_all(smalls_visited);
                to_visit.push(to_visit_set);
            }
            None => {
                to_visit.pop();
                path.pop();
                smalls_visited.remove(path_head);
            }
        }
    }

    path_count
}

fn part_2(map: &Map) -> i32 {
    let start = map.start();
    let end = map.end();

    let mut path_count = 0;

    let mut path = vec![start];
    let mut to_visit = vec![map.edges[start.0]];
    let mut smalls_visited = NodeSet::new_empty();
    smalls_visited.add(start);
    let mut smalls_visited_twice = NodeSet::new_empty();

    while let Some(path_head) = path.last().cloned() {
        // debug_assert!(smalls_visited_twice.0.count_ones() <= 1);

        if path_head == end {
            path_count += 1;

            // Don't consider any extensions to the path
            path.pop();
            to_visit.pop();
            smalls_visited.remove(end);
            continue;
        }

        match to_visit.last_mut().unwrap().next() {
            Some(node) => {
                path.push(node);
                if map.nodes[node.0].is_small {
                    if smalls_visited.contains(node) {
                        smalls_visited_twice.add(node);
                    } else {
                        smalls_visited.add(node);
                    }
                }
                let mut to_visit_set = map.edges[node.0];
                if smalls_visited_twice.any() {
                    to_visit_set.remove_all(smalls_visited);
                }

                // The start node is more special than the other small nodes
                // The end node is never iterated beyond anyway.
                to_visit_set.remove(start);
                to_visit.push(to_visit_set);
            }
            None => {
                to_visit.pop();
                path.pop();

                if smalls_visited_twice.contains(path_head) {
                    smalls_visited_twice.remove(path_head);
                } else {
                    smalls_visited.remove(path_head);
                }
            }
        }
    }

    path_count
}

fn main() {
    let input = input();

    // So the 16 bit NodeSet impl works
    assert!(input.nodes.len() <= 16);

    let sw = Instant::now();
    let p1_ans = part_1(&input);
    let p1_time = sw.elapsed();
    dbg!(p1_ans);
    dbg!(p1_time);

    let sw = Instant::now();
    let p2_ans = part_2(&input);
    let p2_time = sw.elapsed();
    dbg!(p2_ans);
    dbg!(p2_time);
}
