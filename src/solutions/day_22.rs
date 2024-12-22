use itertools::Itertools;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use rustc_hash::FxHashMap;

pub fn solve_1(secrets: &[&str]) -> i64 {
    secrets
        .iter()
        .map(|secret| Secret::new(secret))
        .map(|secret| secret.predict(2_000))
        .sum()
}

pub fn solve_2(secrets: &[&str]) -> i64 {
    let all_sequences = secrets
        .par_iter()
        .map(|secret| Secret::new(secret))
        .map(|secret| secret.sequences(2_000))
        .collect::<Vec<_>>();

    let mut combined_sequences = FxHashMap::default();

    for sequences in all_sequences {
        for (sequence, bananas) in sequences {
            *combined_sequences.entry(sequence).or_insert(0) += bananas;
        }
    }

    combined_sequences.values().max().copied().unwrap()
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Secret {
    number: i64,
}

impl Secret {
    pub fn new(secret: &str) -> Self {
        Self {
            number: secret.parse().unwrap(),
        }
    }

    fn predict(&self, time: i64) -> i64 {
        self.numbers(time)[time as usize]
    }

    #[allow(clippy::identity_op)]
    pub fn sequences(&self, time: i64) -> FxHashMap<i64, i64> {
        #[rustfmt::skip]
        fn sequence_compact(secrets: &[i64], idx: usize) -> i64 {
                  (secrets[idx - 3] % 10 - secrets[idx - 4] % 10) * 1_000_000
                + (secrets[idx - 2] % 10 - secrets[idx - 3] % 10) * 10_000
                + (secrets[idx - 1] % 10 - secrets[idx - 2] % 10) * 100
                + (secrets[idx - 0] % 10 - secrets[idx - 1] % 10) * 1
        }

        let secrets = self.numbers(time);

        (4..secrets.len())
            .map(|idx| (sequence_compact(&secrets, idx), secrets[idx] % 10))
            .unique_by(|(sequence, _)| *sequence)
            .collect()
    }

    fn numbers(&self, time: i64) -> Vec<i64> {
        let mut secrets = vec![self.number];
        let mut number = self.number;

        for _ in 0..time {
            number = ((number * 64) ^ number) % 16_777_216;
            number = ((number / 32) ^ number) % 16_777_216;
            number = ((number * 2048) ^ number) % 16_777_216;

            secrets.push(number);
        }

        secrets
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn day_22_part_01_sample() {
        #[rustfmt::skip]
        let sample = vec![
            "1",
            "10",
            "100",
            "2024",
        ];

        assert_eq!(37_327_623, solve_1(&sample));
    }

    #[test]
    fn day_22_part_01_solution() {
        let input = include_str!("../../inputs/day_22.txt")
            .lines()
            .collect_vec();

        assert_eq!(14_726_157_693, solve_1(&input));
    }

    #[test]
    fn day_22_part_02_sample() {
        #[rustfmt::skip]
        let sample = vec![
            "1",
            "2",
            "3",
            "2024",
        ];

        assert_eq!(23, solve_2(&sample));
    }

    #[test]
    fn day_22_part_02_solution() {
        let input = include_str!("../../inputs/day_22.txt")
            .lines()
            .collect_vec();

        assert_eq!(1_614, solve_2(&input));
    }
}
