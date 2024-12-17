use advent_of_code_2024::solutions::*;
use criterion::{criterion_group, criterion_main, Criterion};
use itertools::Itertools;

/// Run this benchmark using
/// ```shell
/// $ cargo bench
/// ```
fn day17(c: &mut Criterion) {
    let input = include_str!("../inputs/day_17.txt").lines().collect_vec();

    c.benchmark_group("day17")
        .bench_function("part1", |b| {
            b.iter(|| day_17::solve_1(&input));
        })
        .bench_function("part2", |b| {
            b.iter(|| day_17::solve_2(&input));
        });
}

criterion_group!(benches, day17);
criterion_main!(benches);
