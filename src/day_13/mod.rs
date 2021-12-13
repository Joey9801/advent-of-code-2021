use std::collections::HashSet;


#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

#[derive(Clone, Copy)]
pub enum Fold {
    X(i64),
    Y(i64),
}

pub struct Input {
    pub points: HashSet<Point>,
    pub folds: Vec<Fold>,
}

pub fn parse_input(raw: &str) -> Input {
    let mut input = Input {
        points: HashSet::new(),
        folds: Vec::new(),
    };

    let mut lines = raw.lines();
    for line in &mut lines {
        if line.len() == 0 {
            break;
        }

        let mut parts = line.split(",");
        let x = parts.next().unwrap().parse().unwrap();
        let y = parts.next().unwrap().parse().unwrap();
        input.points.insert(Point {x, y});
    }
    
    for line in lines {
        debug_assert!(line.is_ascii());
        debug_assert!(line.starts_with("fold along "));
        
        let mut parts = line.split("=");
        parts.next();
        let value = parts.next().unwrap().parse().unwrap();

        let fold = match line.as_bytes()["fold along ".len()] {
            b'x' => Fold::X(value),
            b'y' => Fold::Y(value),
            _ => panic!("Invalid input string"),
        };
        
        input.folds.push(fold);
    }
    
    input
}

fn do_fold(points: &mut HashSet<Point>, fold: Fold) {
    let mut new_points = HashSet::new();

    match fold {
        Fold::X(fold_x) => {    
            for point in points.drain() {
                if point.x <= fold_x {
                    new_points.insert(point);
                } else {
                    debug_assert!(point.x <= 2 * fold_x);
                    new_points.insert(Point {
                        x: 2 * fold_x - point.x,
                        y: point.y
                    });
                }
            }
        }
        Fold::Y(fold_y) => {    
            for point in points.drain() {
                if point.y <= fold_y {
                    new_points.insert(point);
                } else {
                    debug_assert!(point.y <= 2 * fold_y);
                    new_points.insert(Point {
                        x: point.x,
                        y: 2 * fold_y - point.y,
                    });
                }
            }
        }
    }
    
    *points = new_points;
}

pub fn part_1(input: &Input) -> usize {
    let mut points = input.points.clone();
    do_fold(&mut points, input.folds[0]);
    points.len()
}

pub fn part_2_impl(points: &mut HashSet<Point>, folds: &[Fold]) {
    for fold in folds {
        do_fold(points, *fold);
    }
}

pub fn part_2(input: &Input) -> &'static str {
    part_2_impl(&mut input.points.clone(), &input.folds);
    "lol ocr"
}

impl_day!("2021", "13", "Transparent Origami", Input, usize, &'static str);