use std::{cmp::Ordering, collections::BinaryHeap};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Point {
    x: i16,
    y: i16,
}

struct NeighboursIter {
    origin: Point,
    dim: Point,
    idx: u8,
}

impl Iterator for NeighboursIter {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        const OFFSETS: [Point; 4] = [
            Point { x: -1, y: 0 },
            Point { x: 1, y: 0 },
            Point { x: 0, y: -1 },
            Point { x: 0, y: 1 },
        ];

        loop {
            if self.idx >= 4 {
                break None;
            }

            let x = self.origin.x + OFFSETS[self.idx as usize].x;
            let y = self.origin.y + OFFSETS[self.idx as usize].y;
            self.idx += 1;

            if x >= 0 && x < self.dim.x && y >= 0 && y < self.dim.y {
                break Some(Point { x, y });
            }
        }
    }
}

trait Map<T: Copy> {
    fn dim(&self) -> Point;
    fn get(&self, pos: Point) -> T;
    fn get_mut(&mut self, pos: Point) -> &mut T;
    fn neighbours(&self, pos: Point) -> NeighboursIter {
        NeighboursIter {
            origin: pos,
            dim: self.dim(),
            idx: 0,
        }
    }
}

pub struct SimpleMap<T> {
    dim: Point,
    data: Vec<T>,
}

impl<T: Copy> SimpleMap<T> {
    fn idx(&self, pos: Point) -> usize {
        pos.x as usize + pos.y as usize * self.dim.x as usize
    }
}

impl<T: Copy> Map<T> for SimpleMap<T> {
    fn dim(&self) -> Point {
        self.dim
    }

    fn get_mut(&mut self, pos: Point) -> &mut T {
        let idx = self.idx(pos);
        &mut self.data[idx]
    }

    fn get(&self, pos: Point) -> T {
        self.data[self.idx(pos)]
    }
}

pub struct TiledMap<'a, T> {
    tile: &'a SimpleMap<T>,
    copies: Point,
}

impl<'a> Map<u8> for TiledMap<'a, u8> {
    fn dim(&self) -> Point {
        Point {
            x: self.tile.dim.x * self.copies.x,
            y: self.tile.dim.y * self.copies.y,
        }
    }

    fn get(&self, pos: Point) -> u8 {
        let inner_x = pos.x % self.tile.dim.x;
        let inner_y = pos.y % self.tile.dim.y;
        let inner = self.tile.get(Point {
            x: inner_x,
            y: inner_y,
        });

        let tile_x = pos.x / self.tile.dim.x;
        let tile_y = pos.y / self.tile.dim.y;
        let incr = tile_x + tile_y;

        ((inner as i16 + incr - 1) % 9 + 1) as u8
    }

    fn get_mut(&mut self, _pos: Point) -> &mut u8 {
        unimplemented!()
    }
}

fn parse_input(raw: &str) -> SimpleMap<u8> {
    let data = raw
        .bytes()
        .filter(|c| c.is_ascii_digit())
        .map(|c| c - b'0')
        .collect();

    let dim = Point {
        x: raw.lines().next().unwrap().len() as i16,
        y: raw.lines().count() as i16,
    };

    SimpleMap { data, dim }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DjikstraValue {
    Infinite,
    Finite(u32),
}

impl PartialOrd for DjikstraValue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DjikstraValue {
    fn cmp(&self, other: &Self) -> Ordering {
        use DjikstraValue::*;

        match (self, other) {
            (Infinite, Infinite) => return Ordering::Equal,
            (Infinite, _) => return Ordering::Greater,
            (_, Infinite) => return Ordering::Less,
            (Finite(a), Finite(b)) => a.cmp(b),
        }
    }
}

impl PartialEq<u32> for DjikstraValue {
    fn eq(&self, other: &u32) -> bool {
        match (self, other) {
            (Self::Finite(x), y) => x == y,
            _ => false,
        }
    }
}

impl PartialOrd<u32> for DjikstraValue {
    fn partial_cmp(&self, other: &u32) -> Option<Ordering> {
        match self {
            DjikstraValue::Infinite => Some(Ordering::Greater),
            DjikstraValue::Finite(x) => x.partial_cmp(other),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct DjikstraUnvisited {
    point: Point,
    value: u32,
}

impl PartialOrd for DjikstraUnvisited {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // NB backwards on purpose
        other.value.partial_cmp(&self.value)
    }
}

impl Ord for DjikstraUnvisited {
    fn cmp(&self, other: &Self) -> Ordering {
        // NB backwards on purpose
        other.value.cmp(&self.value)
    }
}

fn djikstra(map: &impl Map<u8>, origin: Point, dest: Point) -> u32 {
    let mut djikstra_values = SimpleMap {
        data: vec![DjikstraValue::Infinite; map.dim().x as usize * map.dim().y as usize],
        dim: map.dim(),
    };

    *djikstra_values.get_mut(origin) = DjikstraValue::Finite(0);
    let mut unvisited = BinaryHeap::with_capacity(djikstra_values.data.len());
    unvisited.push(DjikstraUnvisited {
        point: origin,
        value: 0,
    });

    while let Some(DjikstraUnvisited { point, value }) = unvisited.pop() {
        if point == dest {
            return value;
        }

        if djikstra_values.get(point) < value {
            // Already found something better for this node elsewhere
            continue;
        }

        for neighbour_point in map.neighbours(point) {
            let next = DjikstraUnvisited {
                point: neighbour_point,
                value: value + map.get(neighbour_point) as u32,
            };

            if djikstra_values.get(neighbour_point) > next.value {
                unvisited.push(next);
                *djikstra_values.get_mut(next.point) = DjikstraValue::Finite(next.value);
            }
        }
    }

    panic!("Did not find solution");
}

fn part_1(input: &SimpleMap<u8>) -> u32 {
    let origin = Point { x: 0, y: 0 };
    let dest = Point {
        x: input.dim.x - 1,
        y: input.dim.y - 1,
    };
    djikstra(input, origin, dest)
}

fn part_2(input: &SimpleMap<u8>) -> u32 {
    let origin = Point { x: 0, y: 0 };
    let dest = Point {
        x: input.dim.x * 5 - 1,
        y: input.dim.y * 5 - 1,
    };
    let tiled_map = TiledMap {
        tile: input,
        copies: Point { x: 5, y: 5 },
    };

    djikstra(&tiled_map, origin, dest)
}

impl_day!("2021", "15", "Chiton", SimpleMap<u8>, u32, u32);
