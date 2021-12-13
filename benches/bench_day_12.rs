use criterion::{black_box, criterion_group, criterion_main, Criterion};

use aoc_2021::day_12::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    let input_str = include_str!("../src/day_12/input.txt");

    let input = parse_input(input_str);

    let mut group = c.benchmark_group("day_12");

    group.bench_function("parse", |b| {
        b.iter(|| black_box(parse_input(black_box(input_str))))
    });
    group.bench_function("part_1", |b| {
        b.iter(|| black_box(part_1(black_box(&input))))
    });
    group.bench_function("part_2", |b| {
        b.iter(|| black_box(part_2(black_box(&input))))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
