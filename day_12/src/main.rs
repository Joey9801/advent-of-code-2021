use std::time::Instant;

use day_12::*;

fn main() {
    let input = input();

    let sw = Instant::now();
    let p1_ans = part_1(&input);
    let p1_time = sw.elapsed();
    dbg!(p1_ans);
    dbg!(p1_time);

    let sw = Instant::now();
    let p2_ans = part_2(&input);
    let p2_time = sw.elapsed();
    dbg!(p2_ans);
    dbg!(p2_time);

    let sw = Instant::now();
    let p2_memo_ans = part_2_memo(&input);
    let p2_memo_time = sw.elapsed();
    dbg!(p2_memo_ans);
    dbg!(p2_memo_time);
}
