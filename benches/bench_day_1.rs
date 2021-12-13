use criterion::{black_box, criterion_group, criterion_main, Criterion};

use aoc_2021::day_1::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("day_1");

    let numbers = parse_input(include_str!("../src/day_1/input.txt"));

    group.bench_function("part_2", |b| b.iter(|| part_2(&numbers)));
    group.bench_function("part_2_unchecked", |b| {
        b.iter(|| black_box(part_2_unchecked(&numbers)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
