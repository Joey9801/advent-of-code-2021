use std::collections::HashSet;

const fn winning_masks() -> [u32; 10] {
    let mut m = [0; 10];

    let mut row = 0;
    while row < 5 {
        m[row] = 0b11111 << (5 * row);
        row += 1;
    }

    let mut col = 0;
    while col < 5 {
        m[col + 5] = 0b00001_00001_00001_00001_00001 << col;
        col += 1;
    }

    m
}

const WINNING_MASKS: [u32; 10] = winning_masks();

#[derive(Clone, Debug)]
struct Board {
    values: [i32; 25],

    // Bitmask, the least significant bit maps to the first element of values
    marked: u32,
}

impl Board {
    fn mark(&mut self, num: i32) {
        if let Some(i) = self.values.iter().position(|x| *x == num) {
            self.marked |= 1 << i;
        }
    }

    fn winning(&self) -> bool {
        for mask in WINNING_MASKS.iter() {
            if self.marked & *mask == *mask {
                return true;
            }
        }

        false
    }

    fn unmarked_sum(&self) -> i32 {
        (0..25)
            .filter(|i| self.marked & (1 << *i) == 0)
            .map(|i| self.values[i])
            .sum()
    }
}

fn input() -> (Vec<i32>, Vec<Board>) {
    const INPUT_STR: &'static str = include_str!("../input.txt");

    let mut lines = INPUT_STR.lines();
    let numbers = lines
        .next()
        .unwrap()
        .split(",")
        .map(|s| s.parse())
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    let mut boards = Vec::new();

    loop {
        // Consume the blank line
        if let None = lines.next() {
            break;
        }
        let mut board = Board {
            values: [0; 25],
            marked: 0u32,
        };

        for row in 0..5 {
            let offset = 5 * row;
            let dest = &mut board.values[offset..(offset + 5)];
            for (i, num) in lines
                .next()
                .unwrap()
                .split_ascii_whitespace()
                .map(|s| s.parse())
                .map(Result::unwrap)
                .enumerate()
            {
                dest[i] = num;
            }
        }

        boards.push(board);
    }

    (numbers, boards)
}

fn part_1(numbers: &[i32], boards: &mut [Board]) -> i32 {
    for number in numbers {
        for board in boards.iter_mut() {
            board.mark(*number);
            if board.winning() {
                return board.unmarked_sum() * *number;
            }
        }
    }

    panic!("No winners");
}

fn part_2(numbers: &[i32], boards: &mut [Board]) -> i32 {
    let mut losing_boards = (0..(boards.len())).collect::<HashSet<_>>();

    let mut last_win = None;
    let mut to_remove = Vec::new();
    for number in numbers {
        to_remove.clear();
        for board in losing_boards.iter() {
            boards[*board].mark(*number);
            if boards[*board].winning() {
                to_remove.push(*board);
                last_win = Some((*board, *number));
            }
        }

        for board in to_remove.iter() {
            losing_boards.remove(board);
        }

        if losing_boards.len() == 0 {
            break;
        }
    }

    match last_win {
        Some((idx, number)) => boards[idx].unmarked_sum() * number,
        None => panic!("No winners"),
    }
}

fn main() {
    let (numbers, boards) = input();

    dbg!(part_1(&numbers, &mut boards.clone()));
    dbg!(part_2(&numbers, &mut boards.clone()));
}
