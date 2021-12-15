use aoc_2021::day_13::*;

fn main() {
    let Input { mut points, folds } = parse_input(include_str!("./input.txt"));
    part_2_impl(&mut points, &folds);

    let max_x = points.iter().map(|p| p.x).max().unwrap();
    let max_y = points.iter().map(|p| p.y).max().unwrap();

    for y in 0..=max_y {
        for x in 0..=max_x {
            let c = if points.contains(&Point { x, y }) {
                '#'
            } else {
                ' '
            };
            print!("{}", c);
        }
        println!();
    }
}
