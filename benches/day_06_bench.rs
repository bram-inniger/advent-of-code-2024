use advent_of_code_2024::solutions::*;
use criterion::{criterion_group, criterion_main, Criterion};
use itertools::Itertools;

/// Run this benchmark using
/// ```shell
/// $ cargo bench
/// ```
fn day06(c: &mut Criterion) {
    let input = include_str!("../inputs/day_06.txt").lines().collect_vec();
    let mut group = c.benchmark_group("day06");

    group.bench_function("part1", |b| {
        b.iter(|| day_06::solve_1(&input));
    });

    group.bench_function("part2", |b| {
        b.iter(|| day_06::solve_2(&input));
    });
}

criterion_group!(benches, day06);
criterion_main!(benches);
