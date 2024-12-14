use advent_of_code_2024::solutions::day_14::Room;
use advent_of_code_2024::solutions::*;
use criterion::{criterion_group, criterion_main, Criterion};
use itertools::Itertools;
use std::time::Duration;

/// Run this benchmark using
/// ```shell
/// $ cargo bench
/// ```
fn day14(c: &mut Criterion) {
    let input = include_str!("../inputs/day_14.txt").lines().collect_vec();
    let room = Room {
        width: 101,
        height: 103,
    };
    c.benchmark_group("day14")
        .measurement_time(Duration::from_secs(120))
        .sample_size(100)
        .bench_function("part1", |b| {
            b.iter(|| day_14::solve_1(&input, &room));
        })
        .bench_function("part2", |b| {
            b.iter(|| day_14::solve_2(&input, &room));
        });
}

criterion_group!(benches, day14);
criterion_main!(benches);
