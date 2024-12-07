use itertools::Itertools;

pub fn solve_1(calibrations: &[&str]) -> u64 {
    solve(calibrations, false)
}

pub fn solve_2(calibrations: &[&str]) -> u64 {
    solve(calibrations, true)
}

fn solve(calibrations: &[&str], concat: bool) -> u64 {
    calibrations
        .iter()
        .map(|c| Calibration::new(c, concat))
        .filter(|c| c.can_solve())
        .map(|c| c.test_value)
        .sum()
}

struct Calibration {
    test_value: u64,
    equation_values: Vec<u64>,
    equation_lengths: Vec<u64>,
    concat: bool,
}

impl Calibration {
    fn new(calibration: &str, concat: bool) -> Self {
        let [test_value, equation_values] = calibration.split(": ").collect_vec()[..] else {
            panic!("Expected two parts but got {}", calibration);
        };

        let test_value = test_value.parse::<u64>().unwrap();
        let equation_values_str = equation_values.split(' ').collect_vec();

        let equation_values = equation_values_str
            .iter()
            .map(|s| s.parse::<u64>().unwrap())
            .collect();
        let equation_lengths = equation_values_str
            .iter()
            .map(|s| s.len() as u32)
            .map(|nr_digits| 10u64.pow(nr_digits))
            .collect();

        Self {
            test_value,
            equation_values,
            equation_lengths,
            concat,
        }
    }

    fn can_solve(&self) -> bool {
        self.can_solve_helper(1, self.equation_values[0])
    }

    fn can_solve_helper(&self, idx: usize, acc: u64) -> bool {
        if idx == self.equation_values.len() {
            acc == self.test_value
        } else {
            self.can_solve_helper(idx + 1, acc + self.equation_values[idx])
                || self.can_solve_helper(idx + 1, acc * self.equation_values[idx])
                || (self.concat
                    && self.can_solve_helper(
                        idx + 1,
                        acc * self.equation_lengths[idx] + self.equation_values[idx],
                    ))
        }
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn day_07_part_01_sample() {
        let sample = vec![
            "190: 10 19",
            "3267: 81 40 27",
            "83: 17 5",
            "156: 15 6",
            "7290: 6 8 6 15",
            "161011: 16 10 13",
            "192: 17 8 14",
            "21037: 9 7 18 13",
            "292: 11 6 16 20",
        ];

        assert_eq!(3_749, solve_1(&sample));
    }

    #[test]
    fn day_07_part_01_solution() {
        let input = include_str!("../../inputs/day_07.txt")
            .lines()
            .collect_vec();

        assert_eq!(975_671_981_569, solve_1(&input));
    }

    #[test]
    fn day_07_part_02_sample() {
        let sample = vec![
            "190: 10 19",
            "3267: 81 40 27",
            "83: 17 5",
            "156: 15 6",
            "7290: 6 8 6 15",
            "161011: 16 10 13",
            "192: 17 8 14",
            "21037: 9 7 18 13",
            "292: 11 6 16 20",
        ];

        assert_eq!(11_387, solve_2(&sample));
    }

    #[test]
    fn day_07_part_02_solution() {
        let input = include_str!("../../inputs/day_07.txt")
            .lines()
            .collect_vec();

        assert_eq!(223_472_064_194_845, solve_2(&input));
    }
}
