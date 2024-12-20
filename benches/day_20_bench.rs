use advent_of_code_2024::solutions::*;
use criterion::{criterion_group, criterion_main, Criterion};
use itertools::Itertools;

/// Run this benchmark using
/// ```shell
/// $ cargo bench
/// ```
fn day20(c: &mut Criterion) {
    let input = include_str!("../inputs/day_20.txt").lines().collect_vec();
    c.benchmark_group("day20")
        .bench_function("part1", |b| {
            b.iter(|| day_20::solve_1(&input, 100));
        })
        .bench_function("part2", |b| {
            b.iter(|| day_20::solve_2(&input, 100));
        });
}

criterion_group!(benches, day20);
criterion_main!(benches);
