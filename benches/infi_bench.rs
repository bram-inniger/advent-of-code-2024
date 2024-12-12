use advent_of_code_2024::solutions::*;
use criterion::{criterion_group, criterion_main, Criterion};
use itertools::Itertools;

/// Run this benchmark using
/// ```shell
/// $ cargo bench
/// ```
fn infi(c: &mut Criterion) {
    let input = include_str!("../inputs/infi/infi.txt")
        .lines()
        .collect_vec();
    c.benchmark_group("infi")
        .bench_function("part1", |b| {
            b.iter(|| infi::solve_1(&input));
        })
        .bench_function("part2", |b| {
            b.iter(|| infi::solve_2(&input));
        });
}

criterion_group!(benches, infi);
criterion_main!(benches);
