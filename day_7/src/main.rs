fn input() -> Vec<i32> {
    include_str!("../input.txt")
        .split(",")
        .map(str::parse)
        .map(Result::unwrap)
        .collect()
}

fn part_1(input: &[i32]) -> i32 {
    let min = *input.iter().min().unwrap();
    let max = *input.iter().max().unwrap();

    (min..max)
        .map(|a| input.iter().map(|b| (a - b).abs()).sum())
        .min()
        .unwrap()
}

fn part_2(input: &[i32]) -> i32 {
    let min = *input.iter().min().unwrap();
    let max = *input.iter().max().unwrap();

    fn fuel_req(a: i32, b: i32) -> i32 {
        let n = (a - b).abs();
        (n * (n + 1)) / 2
    }

    (min..max)
        .map(|a| input.iter().map(|b| fuel_req(a, *b)).sum())
        .min()
        .unwrap()
}

fn main() {
    let input = input();
    dbg!(part_1(&input));
    dbg!(part_2(&input));
}
