use itertools::Itertools;
use std::collections::HashSet;

pub fn solve_1(reports: &[&str]) -> usize {
    reports
        .iter()
        .map(|report| Report::new(report))
        .filter(|report| report.is_safe())
        .count()
}

#[derive(Debug)]
struct Report {
    levels: Vec<i32>,
}

impl Report {
    fn new(report: &str) -> Self {
        let levels = report
            .split_whitespace()
            .map(|level| level.parse().unwrap())
            .collect();

        Self { levels }
    }

    fn is_safe(&self) -> bool {
        let allowed = [1, 2, 3].into_iter().collect::<HashSet<i32>>();
        let sign = i32::signum(self.levels[0] - self.levels[1]);
        let deltas = (1..self.levels.len())
            .map(|idx| sign * (self.levels[idx - 1] - self.levels[idx]))
            .collect_vec();

        deltas.iter().all(|delta| allowed.contains(delta))
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn day_02_part_01_sample() {
        let sample = vec![
            "7 6 4 2 1",
            "1 2 7 8 9",
            "9 7 6 2 1",
            "1 3 2 4 5",
            "8 6 4 4 1",
            "1 3 6 7 9",
        ];

        assert_eq!(2, solve_1(&sample));
    }

    #[test]
    fn day_02_part_01_solution() {
        let input = include_str!("../../inputs/day_02.txt")
            .lines()
            .collect_vec();

        assert_eq!(390, solve_1(&input));
    }
}
