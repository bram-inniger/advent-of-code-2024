use lazy_static::lazy_static;
use std::collections::HashSet;

pub fn solve_1(reports: &[&str]) -> usize {
    solve(reports, false)
}

pub fn solve_2(reports: &[&str]) -> usize {
    solve(reports, true)
}

pub fn solve(reports: &[&str], problem_dampener: bool) -> usize {
    reports
        .iter()
        .map(|report| Report::new(report))
        .filter(|report| report.is_safe(problem_dampener))
        .count()
}

lazy_static! {
    static ref ALLOWED: HashSet<i32> = [1, 2, 3].into_iter().collect::<HashSet<i32>>();
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

    fn is_safe(&self, problem_dampener: bool) -> bool {
        fn is_safe_levels(levels: &[i32]) -> bool {
            let sign = i32::signum(levels[0] - levels[1]);
            (1..levels.len())
                .map(|idx| sign * (levels[idx - 1] - levels[idx]))
                .all(|delta| ALLOWED.contains(&delta))
        }

        if is_safe_levels(&self.levels) {
            return true;
        }

        if problem_dampener {
            for idx in 0..self.levels.len() {
                let mut levels = self.levels.clone();
                levels.remove(idx);
                let safe = is_safe_levels(&levels);

                if safe {
                    return true;
                }
            }
        }

        false
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

    #[test]
    fn day_02_part_02_sample() {
        let sample = vec![
            "7 6 4 2 1",
            "1 2 7 8 9",
            "9 7 6 2 1",
            "1 3 2 4 5",
            "8 6 4 4 1",
            "1 3 6 7 9",
        ];

        assert_eq!(4, solve_2(&sample));
    }

    #[test]
    fn day_02_part_02_solution() {
        let input = include_str!("../../inputs/day_02.txt")
            .lines()
            .collect_vec();

        assert_eq!(439, solve_2(&input));
    }
}
