#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Delimiter {
    Paren,
    Bracket,
    Brace,
    Angle,
}

#[derive(Clone, Copy, Debug)]
pub enum Symbol {
    Open(Delimiter),
    Close(Delimiter),
}

impl Symbol {
    fn from_char(c: char) -> Self {
        match c {
            '(' => Self::Open(Delimiter::Paren),
            ')' => Self::Close(Delimiter::Paren),
            '[' => Self::Open(Delimiter::Bracket),
            ']' => Self::Close(Delimiter::Bracket),
            '{' => Self::Open(Delimiter::Brace),
            '}' => Self::Close(Delimiter::Brace),
            '<' => Self::Open(Delimiter::Angle),
            '>' => Self::Close(Delimiter::Angle),
            _ => panic!("illegal character"),
        }
    }
}

fn parse_input(raw: &str) -> Vec<Vec<Symbol>> {
    raw
        .lines()
        .map(|line| line.chars().map(Symbol::from_char).collect())
        .collect()
}

fn part_1(input: &[Vec<Symbol>]) -> i32 {
    fn score(d: Delimiter) -> i32 {
        match d {
            Delimiter::Paren => 3,
            Delimiter::Bracket => 57,
            Delimiter::Brace => 1197,
            Delimiter::Angle => 25137,
        }
    }

    let mut stack = Vec::new();
    let mut total_score = 0;
    for line in input {
        stack.clear();

        let mut this_score = 0;
        for sym in line {
            match *sym {
                Symbol::Open(open_d) => stack.push(open_d),
                Symbol::Close(close_d) => match stack.pop() {
                    Some(open_d) if open_d == close_d => continue,
                    Some(_open_d) => {
                        this_score = score(close_d);
                        break;
                    }
                    None => panic!("Problem statement didn't describe this case"),
                },
            }
        }

        total_score += this_score;
    }

    total_score
}

fn part_2(input: &[Vec<Symbol>]) -> u64 {
    fn score(d: Delimiter) -> u64 {
        match d {
            Delimiter::Paren => 1,
            Delimiter::Bracket => 2,
            Delimiter::Brace => 3,
            Delimiter::Angle => 4,
        }
    }

    let mut stack = Vec::new();
    let mut scores = Vec::new();
    'lines: for line in input {
        stack.clear();

        for sym in line {
            match *sym {
                Symbol::Open(open_d) => stack.push(open_d),
                Symbol::Close(close_d) => {
                    match stack.pop() {
                        // Skip the corrupted line
                        Some(open_d) if open_d != close_d => continue 'lines,
                        Some(_open_d) => continue,
                        None => panic!("Problem statement didn't describe this case"),
                    }
                }
            }
        }

        // The stack now contains the delimiters we need in reverse order
        scores.push(stack.iter().rev().fold(0, |s, d| s * 5 + score(*d)));
    }

    scores.sort();
    scores[scores.len() / 2]
}

impl_day!("2021", "10", "Syntax Scoring", Vec<Vec<Symbol>>, i32, u64);