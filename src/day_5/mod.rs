use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct IntVec {
    x: i32,
    y: i32,
}

impl std::ops::Add for IntVec {
    type Output = IntVec;

    fn add(self, rhs: Self) -> Self::Output {
        IntVec {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Line {
    start: IntVec,
    end: IntVec,
}

impl Line {
    const fn dir(&self) -> IntVec {
        IntVec {
            x: (self.end.x - self.start.x).signum(),
            y: (self.end.y - self.start.y).signum(),
        }
    }

    fn iter_points(&self) -> LineIter {
        LineIter::new(*self)
    }
}

struct LineIter {
    line: Line,
    pos: IntVec,
    dir: IntVec,
}

impl LineIter {
    fn new(line: Line) -> Self {
        Self {
            line,
            dir: line.dir(),
            pos: line.start,
        }
    }
}

impl Iterator for LineIter {
    type Item = IntVec;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos == self.line.end + self.dir {
            None
        } else {
            let ret = self.pos;
            self.pos = self.pos + self.dir;
            Some(ret)
        }
    }
}

fn parse_input(raw: &str) -> Vec<Line> {
    fn parse_intvec(s: &str) -> IntVec {
        let mut parts = s.split(",");
        let x = parts.next().unwrap().parse().unwrap();
        let y = parts.next().unwrap().parse().unwrap();
        IntVec { x, y }
    }

    let mut lines = Vec::new();
    for line in raw.lines() {
        let mut parts = line.split_ascii_whitespace();
        let start = parse_intvec(parts.next().unwrap());
        parts.next();
        let end = parse_intvec(parts.next().unwrap());
        lines.push(Line { start, end });
    }

    lines
}

fn count_overlaps<const DIAG: bool>(lines: &[Line]) -> i32 {
    let mut density_map = HashMap::<IntVec, i32>::new();

    for line in lines.iter() {
        if !DIAG {
            let d = line.dir();
            if d.x != 0 && d.y != 0 {
                continue;
            }
        }

        for point in line.iter_points() {
            *density_map.entry(point).or_insert(0) += 1;
        }
    }

    density_map.values().filter(|v| **v > 1).count() as i32
}

fn part_1(lines: &Vec<Line>) -> i32 {
    count_overlaps::<false>(lines)
}

fn part_2(lines: &Vec<Line>) -> i32 {
    count_overlaps::<true>(lines)
}

impl_day!("2021", "5", "Hydrothermal Venture", Vec<Line>, i32, i32);