pub fn solve_1(secrets: &[&str]) -> u64 {
    secrets
        .iter()
        .map(|secret| Secret::new(secret))
        .map(|secret| secret.predict(2_000))
        .sum()
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Secret {
    number: u64,
}

impl Secret {
    fn new(secret: &str) -> Self {
        Self {
            number: secret.parse().unwrap(),
        }
    }

    fn predict(&self, time: u32) -> u64 {
        let mut number = self.number;

        for _ in 0..time {
            number = ((number * 64) ^ number) % 16_777_216;
            number = ((number / 32) ^ number) % 16_777_216;
            number = ((number * 2048) ^ number) % 16_777_216;
        }

        number
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
}
