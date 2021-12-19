#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl std::ops::Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Target {
    bottom_left: Point,
    top_right: Point,
}

impl Target {
    fn contains(&self, p: Point) -> bool {
        p.x >= self.bottom_left.x
            && p.x <= self.top_right.x
            && p.y >= self.bottom_left.y
            && p.y <= self.top_right.y
    }
}

fn parse_input(raw: &str) -> Target {
    assert!(raw.starts_with("target area: "));

    let raw = &raw["target area: ".len()..];

    let mut parts = raw.split(",");

    let x_part = parts.next().unwrap();
    assert!(x_part.starts_with("x="));
    let (x_min, x_max) = {
        let mut parts = x_part["x=".len()..].split("..");
        let x_min = parts.next().unwrap().parse().unwrap();
        let x_max = parts.next().unwrap().parse().unwrap();
        (x_min, x_max)
    };

    let y_part = parts.next().unwrap();
    assert!(y_part.starts_with(" y="));
    let (y_min, y_max) = {
        let mut parts = y_part["y= ".len()..].split("..");
        let y_min = parts.next().unwrap().parse().unwrap();
        let y_max = parts.next().unwrap().parse().unwrap();
        (y_min, y_max)
    };

    Target {
        bottom_left: Point { x: x_min, y: y_min },
        top_right: Point { x: x_max, y: y_max },
    }
}

#[derive(Debug)]
struct State {
    pos: Point,
    vel: Point,
}

impl State {
    fn step(&mut self) {
        self.pos = self.pos + self.vel;
        self.vel.x -= self.vel.x.signum();
        self.vel.y -= 1;
    }
}

fn trial(target: &Target, init_vel: Point) -> Option<i32> {
    let mut state = State {
        pos: Point { x: 0, y: 0 },
        vel: init_vel,
    };

    let mut max_height = 0;
    loop {
        if target.contains(state.pos) {
            break Some(max_height);
        }

        if state.pos.y < target.bottom_left.y && state.vel.y <= 0 {
            // Falling below target
            break None;
        }

        if state.pos.x < target.bottom_left.x && state.vel.x <= 0 {
            // Falling short
            break None;
        }

        if state.pos.x > target.top_right.x && state.vel.x >= 0 {
            // Overshot
            break None;
        }

        state.step();
        max_height = std::cmp::max(max_height, state.pos.y);
    }
}

fn part_1(target: &Target) -> i32 {
    let mut max = 0;
    for init_x in 0..(target.top_right.x + 1) {
        for init_y in (target.bottom_left.y - 1)..1000 {
            if let Some(height) = trial(
                target,
                Point {
                    x: init_x,
                    y: init_y,
                },
            ) {
                max = std::cmp::max(max, height);
            }
        }
    }

    max
}

fn part_2(target: &Target) -> i32 {
    let mut count = 0;
    for init_x in 0..(target.top_right.x + 1) {
        for init_y in (target.bottom_left.y - 1)..1000 {
            if trial(
                target,
                Point {
                    x: init_x,
                    y: init_y,
                },
            )
            .is_some()
            {
                count += 1;
            }
        }
    }

    count
}

impl_day!("2021", "17", "Trick shot", Target, i32, i32);
