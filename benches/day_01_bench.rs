use criterion::{criterion_group, criterion_main, Criterion};

use advent_of_code_2024::solutions::*;

/// Run this benchmark using
/// ```shell
/// $ cargo bench
/// ```
fn day01(c: &mut Criterion) {
    let input = include_str!("../inputs/day_01.txt").trim();
    let mut group = c.benchmark_group("day01");

    group.bench_function("part1", |b| {
        b.iter(|| day_01::solve_1(input));
    });

    group.bench_function("part2", |b| {
        b.iter(|| day_01::solve_2(input));
    });
}

criterion_group!(benches, day01);
criterion_main!(benches);
