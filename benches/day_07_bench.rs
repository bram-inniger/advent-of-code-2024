use advent_of_code_2024::solutions::*;
use criterion::{criterion_group, criterion_main, Criterion};
use itertools::Itertools;

/// Run this benchmark using
/// ```shell
/// $ cargo bench
/// ```
fn day07(c: &mut Criterion) {
    let input = include_str!("../inputs/day_07.txt").lines().collect_vec();
    let mut group = c.benchmark_group("day07");

    group.bench_function("part1", |b| {
        b.iter(|| day_07::solve_1(&input));
    });

    group.bench_function("part2", |b| {
        b.iter(|| day_07::solve_2(&input));
    });
}

criterion_group!(benches, day07);
criterion_main!(benches);
