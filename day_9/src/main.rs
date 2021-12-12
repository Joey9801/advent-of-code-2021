use std::collections::HashSet;

struct Map {
    values: Vec<u8>,
    dim_x: i8,
    dim_y: i8,
}

impl Map {
    fn get(&self, (x, y): (i8, i8)) -> Option<u8> {
        if x < 0 || x >= self.dim_x {
            None
        } else if y < 0 || y >= self.dim_y {
            None
        } else {
            let idx = x as usize + self.dim_x as usize * y as usize;
            Some(self.values[idx])
        }
    }
}

fn neighbours(pos: (i8, i8)) -> [(i8, i8); 4] {
    [
        (pos.0 - 1, pos.1),
        (pos.0 + 1, pos.1),
        (pos.0, pos.1 - 1),
        (pos.0, pos.1 + 1),
    ]
}

fn input() -> Map {
    let values = include_str!("../input.txt")
        .bytes()
        .filter(|c| c.is_ascii_digit())
        .map(|c| c - b'0')
        .collect();

    Map {
        values,
        dim_x: 100,
        dim_y: 100,
    }
}

fn part_1(map: &Map) -> i32 {
    let mut out = 0;
    for x in 0..(map.dim_x) {
        for y in 0..(map.dim_y) {
            let val = map.get((x, y)).unwrap();
            let min_neighbour_val = neighbours((x, y))
                .iter()
                .filter_map(|pos| map.get(*pos))
                .min()
                .expect("Expected value to have at least one neighbour");

            if val < min_neighbour_val {
                out += val as i32 + 1;
            }
        }
    }

    out
}

fn part_2(map: &Map) -> usize {
    let mut mins = Vec::new();
    for x in 0..(map.dim_x) {
        for y in 0..(map.dim_y) {
            let val = map.get((x, y)).unwrap();
            let min_neighbour_val = neighbours((x, y))
                .iter()
                .filter_map(|pos| map.get(*pos))
                .min()
                .expect("Expected value to have at least one neighbour");

            if val < min_neighbour_val {
                mins.push((x, y));
            }
        }
    }

    let mut basin_sizes = Vec::with_capacity(mins.len());

    let mut basin_elements = HashSet::new();
    let mut work_queue = Vec::new();
    for min_pos in mins {
        basin_elements.clear();
        basin_elements.insert(min_pos);

        work_queue.clear();
        work_queue.push(min_pos);

        while let Some(pos) = work_queue.pop() {
            for neighbour_pos in neighbours(pos) {
                match map.get(neighbour_pos) {
                    None | Some(9) => continue,
                    _ => (),
                }

                if !basin_elements.contains(&neighbour_pos) {
                    work_queue.push(neighbour_pos);
                    basin_elements.insert(neighbour_pos);
                }
            }
        }

        basin_sizes.push(basin_elements.len());
    }

    basin_sizes.sort_by(|a, b| b.cmp(a));
    basin_sizes[..3].iter().product()
}

fn main() {
    let input = input();
    dbg!(part_1(&input));
    dbg!(part_2(&input));
}
