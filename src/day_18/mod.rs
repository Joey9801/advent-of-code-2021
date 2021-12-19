use std::str::FromStr;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct SnailNum {
    // Consider an [i8; 4] representing a snail number of max depth 2
    //     "[[a, b], [c, d]]" => [a,  b,  c,  d]
    //     "[a, [c, d]]"      => [a, -1,  c,  d]
    //     "[[a, b], c]"      => [a,  b,  c, -1]
    //     "[a, b]"           => [a, -1,  b, -1]
    //
    // Consider an [i8; 8] representing a snail number of max depth 3
    //     "[[[a, b], [c, d]], [[e, f], [g, h]]]" => [a,  b,  c,  d,  e,  f,  g,  h]
    //     "[[[a, b], [c, d]], [e, [g, h]]]"      => [a,  b,  c,  d,  e, -1,  g,  h]
    //     "[a, [[e, f], [g, h]]]"                => [a, -1, -1, -1,  e,  f,  g,  h]
    //     "[[a, b], [c, d]]"                     => [a, -1,  b, -1,  c, -1,  d, -1]
    //     "[[a, b], c]"                          => [a, -1,  b, -1,  c, -1, -1, -1]
    //     "[[a, [b, c]"                          => [a, -1, -1, -1,  b, -1,  c, -1]
    //     "[a, b]"                               => [a, -1, -1, -1,  b, -1, -1, -1]
    //
    // This field follows the same pattern, but for a max depth of 5
    contents: [i8; 32],
}

impl SnailNum {
    const MAX_DEPTH: u8 = 5;

    pub fn magnitude(&self) -> i32 {
        fn magnitude_chunk(chunk: &[i8]) -> i32 {
            match *chunk {
                [-1, -1] => -1,
                [x, -1] => x as i32,
                [x, y] => 3 * x as i32 + 2 * y as i32,
                _ => {
                    let split = chunk.len() / 2;
                    let x = magnitude_chunk(&chunk[..split]);
                    let y = magnitude_chunk(&chunk[split..]);
                    match (x, y) {
                        (-1, -1) => -1,
                        (x, -1) => x,
                        (x, y) => 3 * x + 2 * y,
                    }
                }
            }
        }

        magnitude_chunk(&self.contents)
    }

    /// The index into self.contents of the left elemnt of a depth 5 pair
    pub fn first_depth_5(&self) -> Option<usize> {
        for i in 0..16 {
            let i = i * 2;
            if self.contents[i] != -1 && self.contents[i + 1] != -1 {
                return Some(i);
            }
        }

        None
    }

    pub fn first_gte_10(&self) -> Option<usize> {
        self.contents.iter().position(|&x| x >= 10)
    }

    fn next(&self, idx: usize) -> Option<usize> {
        self.contents[(idx + 1)..]
            .iter()
            .position(|&x| x != -1)
            .map(|x| x + idx + 1)
    }

    fn prev(&self, idx: usize) -> Option<usize> {
        self.contents[..idx]
            .iter()
            .rev()
            .position(|&x| x != -1)
            .map(|x| idx - x - 1)
    }

    pub fn reduce(&mut self) {
        loop {
            if let Some(idx) = self.first_depth_5() {
                // Explode the pair at the deepest level
                let left = idx;
                let right = idx + 1;
                debug_assert!(self.contents[left] != -1);
                debug_assert!(self.contents[right] != -1);

                if let Some(prev) = self.prev(left) {
                    self.contents[prev] += self.contents[left];
                }

                if let Some(next) = self.next(right) {
                    self.contents[next] += self.contents[right];
                }

                self.contents[left] = 0;
                self.contents[right] = -1;
            } else if let Some(idx) = self.first_gte_10() {
                debug_assert!(self.contents[idx] != -1);
                debug_assert!(self.contents[idx + 1] == -1);

                let spaces = self.contents[(idx + 1)..]
                    .iter()
                    .take_while(|&&x| x == -1)
                    .count();
                let left = idx;
                let right = match spaces {
                    1 => left + 1,
                    3 => left + 2,
                    7 => left + 4,
                    15 => left + 8,
                    _ => unreachable!(),
                };

                let old_val = self.contents[left];
                let left_val = old_val / 2;
                let right_val = old_val - left_val;

                self.contents[left] = left_val;
                self.contents[right] = right_val;
            } else {
                break;
            }
        }
    }
}

impl std::fmt::Debug for SnailNum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn render_chunk(chunk: &[i8]) -> String {
            assert!(chunk.len() >= 2);
            assert!(chunk.len() % 2 == 0);
            match chunk {
                [-1, -1] => String::new(),
                [x, -1] => format!("{x}"),
                [x, y] => format!("[{x},{y}]"),
                _ => {
                    let split = chunk.len() / 2;
                    let x = render_chunk(&chunk[..split]);
                    let y = render_chunk(&chunk[split..]);

                    match (x.as_str(), y.as_str()) {
                        ("", "") => String::new(),
                        (_, "") => x,
                        ("", _) => y,
                        (_, _) => format!("[{x},{y}]"),
                    }
                }
            }
        }

        write!(f, "{}", render_chunk(&self.contents))
    }
}

impl std::fmt::Display for SnailNum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::ops::Add for SnailNum {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut new = SnailNum { contents: [-1; 32] };

        for i in 0..16 {
            new.contents[i] = self.contents[i * 2];
            new.contents[i + 16] = rhs.contents[i * 2];
        }

        new.reduce();
        new
    }
}

#[derive(Clone, Copy, Debug)]
pub enum SnailNumParseError {
    TooDeep,
    InvalidChar,
}

impl FromStr for SnailNum {
    type Err = SnailNumParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut num = SnailNum { contents: [-1; 32] };

        let mut idx = 0;
        let mut depth = 0;
        for c in s.chars() {
            match c {
                x if x.is_ascii_whitespace() => continue,
                '[' => depth += 1,
                ']' => depth -= 1,
                ',' => {
                    // Advance the index by some amount, depending on the current depth
                    // If at MAX_DEPTH, advance to the next multiple of 2^0
                    // if at MAX_DEPTH - 1, advance to the next multiple of 2^1
                    // if at MAX_DEPTH - 2, advance to the next multiple of 2^2
                    idx = idx >> (Self::MAX_DEPTH - depth);
                    idx += 1;
                    idx = idx << (Self::MAX_DEPTH - depth);
                }
                x if x.is_ascii_digit() => {
                    let x_num = x.to_digit(10).unwrap() as i8;
                    num.contents[idx] = x_num;
                }
                _ => Err(SnailNumParseError::InvalidChar)?,
            }
        }

        Ok(num)
    }
}

fn parse_input(raw: &str) -> Vec<SnailNum> {
    raw.lines()
        .map(|s| s.parse())
        .collect::<Result<Vec<_>, _>>()
        .expect("Expected puzzle input to parse")
}

fn part_1(input: &[SnailNum]) -> i32 {
    let mut sum = input[0];

    for num in &input[1..] {
        sum = sum + *num;
    }

    sum.magnitude()
}

fn part_2(input: &[SnailNum]) -> i32 {
    let mut max = -1;
    for a in 0..input.len() {
        for b in 0..input.len() {
            if a == b {
                continue;
            }

            max = std::cmp::max(max, (input[a] + input[b]).magnitude());
        }
    }

    max
}

impl_day!("2021", "18", "Snailfish", Vec<SnailNum>, i32, i32);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snailnum_render() {
        let mut num = SnailNum { contents: [-1; 32] };

        num.contents[0] = 1;
        num.contents[16] = 2;
        assert_eq!(format!("{}", num), "[1,2]");

        num.contents[24] = 3;
        assert_eq!(format!("{}", num), "[1,[2,3]]");

        num.contents[8] = 5;
        num.contents[24] = -1;
        assert_eq!(format!("{}", num), "[[1,5],2]");

        for i in 0..32 {
            if i % 4 == 0 {
                num.contents[i] = i as i8 / 4;
            } else {
                num.contents[i] = -1;
            }
        }
        assert_eq!(format!("{}", num), "[[[0,1],[2,3]],[[4,5],[6,7]]]");

        for i in 0..32 {
            if i % 2 == 0 {
                num.contents[i] = i as i8 / 2;
            } else {
                num.contents[i] = -1;
            }
        }
        assert_eq!(
            format!("{}", num),
            "[[[[0,1],[2,3]],[[4,5],[6,7]]],[[[8,9],[10,11]],[[12,13],[14,15]]]]"
        );
    }

    #[test]
    fn test_snailnum_roundtrips() {
        for num_str in include_str!("./input.txt").lines() {
            let num: SnailNum = num_str.parse().expect("expected input number to parse");

            assert_eq!(format!("{}", num), num_str);
        }
    }

    #[test]
    fn test_reduce() {
        let cases = vec![(
            "[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]",
            "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]",
        )];

        for (input, expected) in cases {
            let mut a: SnailNum = input.parse().expect("Expected test case input to parse");

            a.reduce();

            // Compare strings for easier debugging
            assert_eq!(format!("{}", a), expected);
        }
    }

    #[test]
    fn test_magnitude() {
        let cases = vec![
            ("[9, 1]", 29),
            ("[[9,1],[1,9]]", 129),
            ("[[1,2],[[3,4],5]]", 143),
            ("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", 1384),
            ("[[[[1,1],[2,2]],[3,3]],[4,4]]", 445),
            ("[[[[3,0],[5,3]],[4,4]],[5,5]]", 791),
            ("[[[[5,0],[7,4]],[5,5]],[6,6]]", 1137),
            (
                "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
                3488,
            ),
        ];

        for (num_str, expected_mag) in cases {
            let num: SnailNum = num_str
                .parse()
                .expect("expected test case to have valid snailnum str");
            dbg!(num);
            assert_eq!(num.magnitude(), expected_mag);
        }
    }

    #[test]
    fn test_add() {
        let cases = vec![
            (
                "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]",
                "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]",
                "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]",
            ),
            (
                "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]",
                "[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]",
                "[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]",
            ),
            (
                "[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]",
                "[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]",
                "[[[[7,0],[7,7]],[[7,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]]",
            ),
            (
                "[[[[7,0],[7,7]],[[7,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]]",
                "[7,[5,[[3,8],[1,4]]]]",
                "[[[[7,7],[7,8]],[[9,5],[8,7]]],[[[6,8],[0,8]],[[9,9],[9,0]]]]",
            ),
            (
                "[[[[7,7],[7,8]],[[9,5],[8,7]]],[[[6,8],[0,8]],[[9,9],[9,0]]]]",
                "[[2,[2,2]],[8,[8,1]]]",
                "[[[[6,6],[6,6]],[[6,0],[6,7]]],[[[7,7],[8,9]],[8,[8,1]]]]",
            ),
            (
                "[[[[6,6],[6,6]],[[6,0],[6,7]]],[[[7,7],[8,9]],[8,[8,1]]]]",
                "[2,9]",
                "[[[[6,6],[7,7]],[[0,7],[7,7]]],[[[5,5],[5,6]],9]]",
            ),
            (
                "[[[[6,6],[7,7]],[[0,7],[7,7]]],[[[5,5],[5,6]],9]]",
                "[1,[[[9,3],9],[[9,0],[0,7]]]]",
                "[[[[7,8],[6,7]],[[6,8],[0,8]]],[[[7,7],[5,0]],[[5,5],[5,6]]]]",
            ),
            (
                "[[[[7,8],[6,7]],[[6,8],[0,8]]],[[[7,7],[5,0]],[[5,5],[5,6]]]]",
                "[[[5,[7,4]],7],1]",
                "[[[[7,7],[7,7]],[[8,7],[8,7]]],[[[7,0],[7,7]],9]]",
            ),
            (
                "[[[[7,7],[7,7]],[[8,7],[8,7]]],[[[7,0],[7,7]],9]]",
                "[[[[4,2],2],6],[8,7]]  ",
                "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
            ),
        ];

        for (a, b, expected_sum) in cases {
            let a: SnailNum = a.parse().expect("Expected test case input to parse");
            let b: SnailNum = b.parse().expect("Expected test case input to parse");

            assert_eq!(format!("{}", a + b), expected_sum);
        }
    }
}
