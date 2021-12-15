use std::time::Duration;

use aoc_2021::{all_days, RunResult};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]
struct Opt {
    /// Single day: Only run the solution for a single day
    #[structopt(name = "DAY_NUM", long = "single-day")]
    single_day: Option<String>,
}

fn print_results(results: &[RunResult]) {
    if results.len() == 0 {
        return;
    }

    fn col_width(results: &[RunResult], title: &str, len: impl Fn(&RunResult) -> usize) -> usize {
        std::cmp::max(title.len(), results.iter().map(len).max().unwrap())
    }

    let name_width = col_width(results, "Name", |r| r.name.name.len());
    let p1_result_width = col_width(results, "P1 result", |r| r.p1_result.len());
    let p2_result_width = col_width(results, "P2 result", |r| r.p2_result.len());
    let parse_time_width = col_width(results, "Parse time", |r| {
        format!("{:?}", r.parse_time).len()
    });
    let p1_time_width = col_width(results, "P1 time", |r| format!("{:?}", r.p1_time).len());
    let p2_time_width = col_width(results, "P2 time", |r| format!("{:?}", r.p2_time).len());

    let total_parse_time: Duration = results.iter().map(|r| r.parse_time).sum();
    let total_p1_time: Duration = results.iter().map(|r| r.p1_time).sum();
    let total_p2_time: Duration = results.iter().map(|r| r.p2_time).sum();
    let total_time: Duration = results.iter().map(|r| r.total_time()).sum();

    let parse_time_width = std::cmp::max(parse_time_width, format!("{:?}", total_parse_time).len());
    let p1_time_width = std::cmp::max(p1_time_width, format!("{:?}", total_p1_time).len());
    let p2_time_width = std::cmp::max(p2_time_width, format!("{:?}", total_p2_time).len());

    let total_offset = 10 // "year/day |"
        + name_width + 3
        + p1_result_width + 3
        + p2_result_width + 1;


    let header = format!(
        "Year/Day | {:name_width$} | {:p1_result_width$} | {:p2_result_width$} | {:parse_time_width$} | {:p1_time_width$} | {:p2_time_width$} | Total time",
        "Name",
        "P1 result",
        "P2 result",
        "Parse time",
        "P1 time",
        "P2 time",
        name_width = name_width,
        p1_result_width = p1_result_width,
        p2_result_width = p2_result_width,
        parse_time_width = parse_time_width,
        p1_time_width = p1_time_width,
        p2_time_width = p2_time_width,
    );

    println!("{}", header);
    for _ in 0..header.len() {
        print!("-");
    }
    println!();

    let print = |result: &RunResult| {
        println!("{}/{:03} | {:name_width$} | {:p1_result_width$} | {:p2_result_width$} | {:parse_time_width$?} | {:p1_time_width$?} | {:p2_time_width$?} | {:?}",
            result.name.year,
            result.name.day,
            result.name.name,
            result.p1_result,
            result.p2_result,
            result.parse_time,
            result.p1_time,
            result.p2_time,
            result.total_time(),
        )
    };

    for result in results {
        print(result);
    }

    for _ in 0..header.len() {
        print!("-");
    }
    println!();

    println!(
        "{:>total_offset$} | {:parse_time_width$?} | {:p1_time_width$?} | {:p2_time_width$?} | {:?}",
        "Total",
        total_parse_time,
        total_p1_time,
        total_p2_time,
        total_time,
    );
}

fn main() {
    let opt = Opt::from_args();
    let mut days = all_days();

    if let Some(day_num) = opt.single_day {
        days = days.drain(..).filter(|d| d.name.day == &day_num).collect();
    }

    let results = days.iter().map(|d| (d.run)()).collect::<Vec<_>>();

    print_results(&results);
}
