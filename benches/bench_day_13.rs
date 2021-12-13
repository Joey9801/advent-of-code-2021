use criterion::{black_box, criterion_group, criterion_main, Criterion};

use aoc_2021::day_13::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    let silly_input_str = include_str!("../src/day_13/silly_input.txt");

    let silly_input = parse_input(silly_input_str);

    let mut group = c.benchmark_group("day_13");

    group.bench_function("parse_silly", |b| {
        b.iter(|| black_box(parse_input(black_box(silly_input_str))))
    });
    group.bench_function("part_1_silly", |b| {
        b.iter(|| black_box(part_1(black_box(&silly_input))))
    });
    group.bench_function("part_2_silly", |b| {
        b.iter(|| black_box(part_2(black_box(&silly_input))))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
