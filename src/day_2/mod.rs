enum Dir {
    Forward,
    Up,
    Down,
}

pub struct Command {
    dir: Dir,
    num: i32,
}

fn parse_input(s: &str) -> Vec<Command> {
    fn parse_line(line: &str) -> Result<Command, ()> {
        let mut parts = line.splitn(2, ' ');
        let (dir_str, num_str) = match (parts.next(), parts.next()) {
            (Some(a), Some(b)) => (a, b),
            _ => return Err(()),
        };

        let dir = match dir_str {
            "forward" => Dir::Forward,
            "up" => Dir::Up,
            "down" => Dir::Down,
            _ => return Err(()),
        };

        let num = num_str.parse().map_err(|_| ())?;

        Ok(Command { dir, num })
    }

    s.trim()
        .split('\n')
        .map(parse_line)
        .collect::<Result<Vec<_>, _>>()
        .expect("Expected input to parse")
}

fn part_1(cmds: &[Command]) -> i32 {
    let mut horizontal = 0;
    let mut depth = 0;

    for cmd in cmds {
        match cmd.dir {
            Dir::Forward => horizontal += cmd.num as i32,
            Dir::Up => depth -= cmd.num as i32,
            Dir::Down => depth += cmd.num as i32,
        }
    }

    horizontal * depth
}

fn part_2(cmds: &[Command]) -> i32 {
    let mut aim = 0;
    let mut horizontal = 0;
    let mut depth = 0;

    for cmd in cmds {
        match cmd.dir {
            Dir::Forward => {
                horizontal += cmd.num;
                depth += cmd.num * aim;
            }
            Dir::Up => aim -= cmd.num,
            Dir::Down => aim += cmd.num,
        }
    }

    horizontal * depth
}

impl_day!("2021", "2", "Dive!", Vec<Command>, i32, i32);
