use std::collections::HashMap;

use fxhash::FxHashMap;

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
pub struct Map {
    nodes: Vec<Node>,

    // Parallel vec to nodes, element N is the set of nodes connected to element N
    edges: Vec<NodeSet>,

    start: NodeIdx,
    end: NodeIdx,
    small_nodes: NodeSet,
}

impl Map {
    fn find_name(&self, name: &str) -> Option<NodeIdx> {
        self.nodes
            .iter()
            .position(|n| n.name == name)
            .map(|idx| NodeIdx(idx))
    }
}

pub fn parse_input(raw: &str) -> Map {
    let mut names = HashMap::new();
    let mut map = Map {
        nodes: Vec::new(),
        edges: Vec::new(),
        start: NodeIdx(0),
        end: NodeIdx(0),
        small_nodes: NodeSet::new_empty(),
    };

    for line in raw.lines() {
        for name in line.split("-") {
            if !names.contains_key(name) {
                let idx = NodeIdx(names.len());
                names.insert(name.to_string(), idx);
                let is_small = name.chars().next().unwrap().is_ascii_lowercase();
                map.nodes.push(Node {
                    name: name.to_string(),
                    is_small,
                    idx,
                });
                map.edges.push(NodeSet::new_empty());

                if is_small {
                    map.small_nodes.add(idx);
                }
            }
        }
    }

    for line in raw.lines() {
        let mut parts = line.split("-");
        let a = names[parts.next().unwrap()];
        let b = names[parts.next().unwrap()];

        map.edges[a.0].add(b);
        map.edges[b.0].add(a);
    }

    map.start = map.find_name("start").unwrap();
    map.end = map.find_name("end").unwrap();

    // Remove all links back to the start node
    for edges in map.edges.iter_mut() {
        edges.remove(map.start);
    }

    map
}

pub fn input() -> Map {
    parse_input(include_str!("./input.txt"))
}

pub fn part_1(map: &Map) -> i32 {
    let start = map.start;
    let end = map.end;

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

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct MemoKey {
    head: NodeIdx,
    visited: NodeSet,
}

const VISITED_SMALL_TWICE_SIGIL: u16 = 1 << 15;

pub fn part_2(map: &Map) -> u32 {
    fn count_paths(map: &Map, state: MemoKey, memo: &mut FxHashMap<MemoKey, u32>) -> u32 {
        match memo.get(&state) {
            Some(cached) => return *cached,
            None => (),
        }

        let mut paths = 0;
        for next_head in map.edges[state.head.0] {
            if next_head == map.end {
                paths += 1;
                continue;
            }

            let mask = next_head.mask() & map.small_nodes.0;

            let mut next_state = state.clone();
            next_state.head = next_head;

            if next_state.visited.0 & mask == 0 {
                next_state.visited.0 |= mask;
                paths += count_paths(map, next_state, memo);
            } else if next_state.visited.0 & VISITED_SMALL_TWICE_SIGIL == 0 {
                next_state.visited.0 |= VISITED_SMALL_TWICE_SIGIL;
                paths += count_paths(map, next_state, memo);
            }
        }

        memo.insert(state, paths);

        paths
    }

    let state = MemoKey {
        head: map.start,
        visited: NodeSet::new_empty(),
    };
    let mut memo = FxHashMap::default();
    memo.reserve(512);
    count_paths(map, state, &mut memo)
}

impl_day!("2021", "12", "Passage Pathing", Map, i32, u32);