use itertools::Itertools;
use rustc_hash::FxHashMap;

pub fn solve_1(stones: &str) -> u64 {
    solve(stones, 25)
}

pub fn solve_2(stones: &str) -> u64 {
    solve(stones, 75)
}

fn solve(stones: &str, iterations: u32) -> u64 {
    fn solve_helper(stones: FxHashMap<u64, u64>, iterations: u32) -> FxHashMap<u64, u64> {
        if iterations == 0 {
            return stones;
        }

        let new_stones = stones
            .into_iter()
            .flat_map(|(stone, count)| match stone {
                0 => vec![(1, count)],
                _ if nr_digits(stone) % 2 == 0 => {
                    let (left, right) = split_stone(stone);
                    vec![(left, count), (right, count)]
                }
                _ => vec![(stone * 2024, count)],
            })
            .sorted_by_key(|&(stone, _)| stone)
            .chunk_by(|&(stone, _)| stone)
            .into_iter()
            .map(|(stone, counts)| {
                (
                    stone,
                    counts.into_iter().map(|(_, count)| count).sum::<u64>(),
                )
            })
            .collect();

        solve_helper(new_stones, iterations - 1)
    }

    let stones = stones
        .split(" ")
        .map(|s| s.parse::<u64>().unwrap())
        .sorted()
        .chunk_by(|&stone| stone)
        .into_iter()
        .map(|(stone, grouped)| (stone, grouped.count() as u64))
        .collect();

    solve_helper(stones, iterations).values().sum()
}

fn nr_digits(n: u64) -> u32 {
    match n {
        0..=9 => 1,
        10..=99 => 2,
        100..=999 => 3,
        1_000..=9_999 => 4,
        10_000..=99_999 => 5,
        100_000..=999_999 => 6,
        1_000_000..=9_999_999 => 7,
        10_000_000..=99_999_999 => 8,
        100_000_000..=999_999_999 => 9,
        1_000_000_000..=9_999_999_999 => 10,
        10_000_000_000..=99_999_999_999 => 11,
        100_000_000_000..=999_999_999_999 => 12,
        1_000_000_000_000..=9_999_999_999_999 => 13,
        10_000_000_000_000..=99_999_999_999_999 => 14,
        100_000_000_000_000..=999_999_999_999_999 => 15,
        1_000_000_000_000_000..=9_999_999_999_999_999 => 16,
        10_000_000_000_000_000..=99_999_999_999_999_999 => 17,
        100_000_000_000_000_000..=999_999_999_999_999_999 => 18,
        1_000_000_000_000_000_000..=9_999_999_999_999_999_999 => 19,
        10_000_000_000_000_000_000..=u64::MAX => 20,
    }
}

fn split_stone(stone: u64) -> (u64, u64) {
    let nr_digits = nr_digits(stone);
    let zeroes = 10u64.pow(nr_digits / 2);

    (stone / zeroes, stone % zeroes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_11_part_01_sample() {
        let sample = "125 17";

        assert_eq!(55_312, solve_1(sample));
    }

    #[test]
    fn day_11_part_01_solution() {
        let input = include_str!("../../inputs/day_11.txt").trim();

        assert_eq!(183_248, solve_1(input));
    }

    #[test]
    fn day_11_part_02_sample() {
        // No sample input provided
    }

    #[test]
    fn day_11_part_02_solution() {
        let input = include_str!("../../inputs/day_11.txt").trim();

        assert_eq!(218_811_774_248_729, solve_2(input));
    }
}
