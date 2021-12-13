pub type InputElem = ([u8; 10], [u8; 4]);

pub fn parse_input(raw: &str) -> Vec<InputElem> {
    fn parse_segment(s: &str) -> u8 {
        let mut out = 0;
        for c in s.bytes() {
            if c < b'a' || c > b'g' {
                panic!("Character out of range");
            }
            out |= 1 << (c - b'a');
        }
        out
    }

    fn parse_line(s: &str) -> ([u8; 10], [u8; 4]) {
        let mut map = [0; 10];
        let mut code = [0; 4];
        for (i, s) in s.split_ascii_whitespace().enumerate() {
            if i < 10 {
                map[i] = parse_segment(s);
            }
            if i >= 11 {
                code[i - 11] = parse_segment(s);
            }
        }
        (map, code)
    }

    raw
        .lines()
        .map(parse_line)
        .collect()
}

#[derive(Debug)]
pub struct SolvedMap {
    digits: [u8; 10],
}

impl SolvedMap {
    /// Returns the map values, but sorted such that the first element is the value for the digit zero
    pub fn from_input(map: [u8; 10]) -> Self {
        let mut out = Self { digits: [0; 10] };

        // Fill in the easy codes, (1, 4, 7, and 8)
        for val in &map {
            match val.count_ones() {
                2 => out.digits[1] = *val,
                3 => out.digits[7] = *val,
                4 => out.digits[4] = *val,
                7 => out.digits[8] = *val,
                _ => (),
            }
        }

        // Of the 5 segment numbers (2, 3, 5), only 3 has (digit1 & digit3) == digit1
        out.digits[3] = *map
            .iter()
            .filter(|v| v.count_ones() == 5)
            .filter(|v| *v & out.digits[1] == out.digits[1])
            .next()
            .unwrap();

        // Of the 6 segment numbers (0, 6, 9), only 6 has (digit1 & digit6) != digit1
        out.digits[6] = *map
            .iter()
            .filter(|v| v.count_ones() == 6)
            .filter(|v| *v & out.digits[1] != out.digits[1])
            .next()
            .unwrap();

        // Of the remaining 6 segment numbers (0, 9), only 9 has (digit9 & digit4) == digit4
        let six = out.digits[6];
        for val in map
            .iter()
            .filter(|v| v.count_ones() == 6)
            .filter(|v| **v != six)
        {
            if *val & out.digits[4] == out.digits[4] {
                out.digits[9] = *val;
            } else {
                out.digits[0] = *val;
            }
        }

        // Of the remaning 5 segment numbers (2, 5), only 5 has (digit5 & digit6) == digit5
        let three = out.digits[3];
        for val in map
            .iter()
            .filter(|v| v.count_ones() == 5)
            .filter(|v| **v != three)
        {
            if *val & six == *val {
                out.digits[5] = *val;
            } else {
                out.digits[2] = *val;
            }
        }

        out
    }

    pub fn value(&self, val: u8) -> u8 {
        self.digits.iter().position(|d| *d == val).unwrap() as u8
    }
}

pub fn part_1(input: &[([u8; 10], [u8; 4])]) -> i32 {
    fn easy_code(c: u8) -> bool {
        match c.count_ones() {
            2 | 3 | 4 | 7 => true,
            _ => false,
        }
    }

    input
        .iter()
        .map(|(_map, code)| code.iter().filter(|c| easy_code(**c)).count())
        .sum::<usize>() as i32
}

pub fn part_2(input: &[([u8; 10], [u8; 4])]) -> i32 {
    let mut ans = 0;
    for (map, code) in input {
        let map = SolvedMap::from_input(*map);

        let code = map.value(code[0]) as i32 * 1000
            + map.value(code[1]) as i32 * 100
            + map.value(code[2]) as i32 * 10
            + map.value(code[3]) as i32 * 1;
        ans += code;
    }

    ans
}

impl_day!("2021", "8", "Seven Segment Search", Vec<InputElem>, i32, i32);