use advent_of_code_2024::solutions::*;
use criterion::{criterion_group, criterion_main, Criterion};
use itertools::Itertools;

/// Run this benchmark using
/// ```shell
/// $ cargo bench
/// ```
fn day22(c: &mut Criterion) {
    let input = include_str!("../inputs/day_22.txt").lines().collect_vec();
    c.benchmark_group("day22")
        .bench_function("part1", |b| {
            b.iter(|| day_22::solve_1(&input));
        })
        .bench_function("part2", |b| {
            b.iter(|| day_22::solve_2(&input));
        });
}

criterion_group!(benches, day22);
criterion_main!(benches);
