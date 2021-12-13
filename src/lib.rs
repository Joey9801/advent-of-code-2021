use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct DayName {
    pub name: &'static str,
    pub year: &'static str,
    pub day: &'static str,
}

#[derive(Debug)]
pub struct RunResult {
    pub name: DayName,
    pub parse_time: Duration,
    pub p1_time: Duration,
    pub p2_time: Duration,
    pub p1_result: String,
    pub p2_result: String,
}

impl RunResult {
    pub fn total_time(&self) -> Duration {
        self.parse_time + self.p1_time + self.p2_time
    }
}

pub trait Day {
    type ParsedInput;
    type P1Result: std::fmt::Display;
    type P2Result: std::fmt::Display;

    const REAL_INPUT: &'static str;

    fn name() -> DayName;

    fn parse_input(raw: &str) -> Self::ParsedInput;
    fn part_1(input: &Self::ParsedInput) -> Self::P1Result;
    fn part_2(input: &Self::ParsedInput) -> Self::P2Result;

    fn run() -> RunResult {
        let sw = Instant::now();
        let input = Self::parse_input(Self::REAL_INPUT);
        let parse_time = sw.elapsed();

        let sw = Instant::now();
        let p1_result = Self::part_1(&input);
        let p1_time = sw.elapsed();
        let p1_result = format!("{}", p1_result);

        let sw = Instant::now();
        let p2_result = Self::part_2(&input);
        let p2_time = sw.elapsed();
        let p2_result = format!("{}", p2_result);

        RunResult {
            name: Self::name(),
            parse_time,
            p1_time,
            p2_time,
            p1_result,
            p2_result,
        }
    }

    fn erased() -> ErasedDay
    where
        Self: 'static,
    {
        ErasedDay {
            name: Self::name(),
            run: Box::new(Self::run),
        }
    } 
}

pub struct ErasedDay {
    pub name: DayName,
    pub run: Box<dyn Fn() -> RunResult>,
}

macro_rules! impl_day {
    ($year:literal, $day:literal, $name:literal, $input_ty:ty, $p1_ans_ty:ty, $p2_ans_ty:ty) => {
        pub struct Day {}

        impl crate::Day for Day {
            type ParsedInput = $input_ty;
            type P1Result = $p1_ans_ty;
            type P2Result = $p2_ans_ty;

            const REAL_INPUT: &'static str = include_str!("./input.txt");

            fn name() -> crate::DayName {
                crate::DayName {
                    name: $name,
                    year: $year,
                    day: $day,
                }
            }

            fn parse_input(raw: &str) -> Self::ParsedInput {
                parse_input(raw)
            }

            fn part_1(input: &Self::ParsedInput) -> Self::P1Result {
                part_1(input)
            }


            fn part_2(input: &Self::ParsedInput) -> Self::P2Result {
                part_2(input)
            }
        }
    }
}

macro_rules! days {
    ($($day:ident),+) => {
        paste::paste! {
            $(
                pub mod [<day $day>];
            )*
            
            pub fn all_days() -> Vec<ErasedDay> {
                vec![
                    $(
                        [<day $day>]::Day::erased(),
                    )*
                ]
            }
        }
    }
}

days!(_1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13);