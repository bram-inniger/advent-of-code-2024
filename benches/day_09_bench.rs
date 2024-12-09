use advent_of_code_2024::solutions::*;
use criterion::{criterion_group, criterion_main, Criterion};
// use std::time::Duration;

/// Run this benchmark using
/// ```shell
/// $ cargo bench
/// ```
fn day09(c: &mut Criterion) {
    let input = include_str!("../inputs/day_09.txt").trim();
    c.benchmark_group("day09")
        // .measurement_time(Duration::from_secs(20))
        // .sample_size(500)
        .bench_function("part1", |b| {
            b.iter(|| day_09::solve_1(input));
        })
        .bench_function("part2", |b| {
            b.iter(|| day_09::solve_2(input));
        });
}

criterion_group!(benches, day09);
criterion_main!(benches);
