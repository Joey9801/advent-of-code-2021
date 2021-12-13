use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day_12::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    let input_str = include_str!("../input.txt");

    let input = parse_input(input_str);
    c.bench_function("d12_parse", |b| {
        b.iter(|| black_box(parse_input(black_box(input_str))))
    });
    c.bench_function("d12_p1", |b| {
        b.iter(|| black_box(part_1(black_box(&input))))
    });
    c.bench_function("d12_p2", |b| {
        b.iter(|| black_box(part_2_memo(black_box(&input))))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
