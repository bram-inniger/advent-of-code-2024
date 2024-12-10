use advent_of_code_2024::solutions::*;
use criterion::{criterion_group, criterion_main, Criterion};
use itertools::Itertools;
// use std::time::Duration;

/// Run this benchmark using
/// ```shell
/// $ cargo bench
/// ```
fn day10(c: &mut Criterion) {
    let input = include_str!("../inputs/day_10.txt").lines().collect_vec();
    c.benchmark_group("day10")
        // .measurement_time(Duration::from_secs(20))
        // .sample_size(500)
        .bench_function("part1", |b| {
            b.iter(|| day_10::solve_1(&input));
        })
        .bench_function("part2", |b| {
            b.iter(|| day_10::solve_2(&input));
        });
}

criterion_group!(benches, day10);
criterion_main!(benches);
