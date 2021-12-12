fn neighbours(pos: (i8, i8)) -> [(i8, i8); 8] {
    [
        (pos.0 - 1, pos.1 - 1),
        (pos.0 - 1, pos.1 + 0),
        (pos.0 - 1, pos.1 + 1),
        (pos.0 + 0, pos.1 - 1),
        (pos.0 + 0, pos.1 + 1),
        (pos.0 + 1, pos.1 - 1),
        (pos.0 + 1, pos.1 + 0),
        (pos.0 + 1, pos.1 + 1),
    ]
}

#[derive(Clone)]
struct OctopusGrid {
    levels: Vec<u8>,
    dim_x: u8,
    dim_y: u8,
}

impl OctopusGrid {
    fn index(&self, (x, y): (i8, i8)) -> Option<usize> {
        if x < 0 || x >= self.dim_x as i8 {
            None
        } else if y < 0 || y >= self.dim_y as i8 {
            None
        } else {
            Some((x + y * self.dim_x as i8) as usize)
        }
    }

    fn step(&mut self) -> i32 {
        // Rule 1: all energy levels start by increasing by 1
        // Rule 2: Any octopus with an energy level greater than 9 flashes, increasing the energy
        //         of its neighbour octopuses by 1 also.
        // Rule 3: Any octopus that flashed has its energy reset to 0

        let mut flash_queue = Vec::new();
        for (idx, o) in self.levels.iter_mut().enumerate() {
            *o += 1;

            if *o > 9 {
                flash_queue.push(idx);
            }
        }

        // Process the effects of the flashes one by one
        while let Some(flash_idx) = flash_queue.pop() {
            let x = (flash_idx as usize % self.dim_x as usize) as i8;
            let y = (flash_idx as usize / self.dim_x as usize) as i8;

            for n in neighbours((x, y)) {
                let n = match self.index(n) {
                    Some(n) => n,
                    None => continue,
                };

                self.levels[n] += 1;

                // Exact equality check, such that a flashing octopus is only ever added to the
                // queue once for this step.
                if self.levels[n] == 10 {
                    flash_queue.push(n);
                }
            }
        }

        let mut flashes = 0;
        for o in self.levels.iter_mut() {
            if *o >= 10 {
                *o = 0;
                flashes += 1;
            }
        }

        flashes
    }
}

fn parse_grid(s: &str) -> OctopusGrid {
    let levels = s
        .bytes()
        .filter(|c| c.is_ascii_digit())
        .map(|c| c - b'0')
        .collect();

    let dim_x = s.lines().next().unwrap().len();

    let dim_y = s.lines().count();

    OctopusGrid {
        levels,
        dim_x: dim_x as u8,
        dim_y: dim_y as u8,
    }
}

fn input() -> OctopusGrid {
    parse_grid(include_str!("../input.txt"))
}

fn part_1(initial: &OctopusGrid) -> i32 {
    let mut state = initial.clone();

    let mut flashes = 0;
    for _ in 0..100 {
        flashes += state.step();
    }

    flashes
}

fn part_2(initial: &OctopusGrid) -> i32 {
    let mut state = initial.clone();

    let mut steps = 1;
    while state.step() != state.levels.len() as i32 {
        steps += 1;
    }

    steps
}

fn main() {
    let input = input();
    dbg!(part_1(&input));
    dbg!(part_2(&input));
}

#[cfg(test)]
mod tests {
    use crate::{parse_grid, part_1, OctopusGrid};

    fn example_input() -> OctopusGrid {
        let s = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";
        parse_grid(s)
    }

    #[test]
    fn test_example() {
        let input = example_input();
        assert_eq!(part_1(&input), 1656);
    }
}
