use itertools::Itertools;
use std::iter;

pub fn solve_1(calibrations: &[&str]) -> u64 {
    calibrations
        .iter()
        .map(|c| Calibration::new(c))
        .filter(|c| c.can_solve())
        .map(|c| c.test_value)
        .sum()
}

struct Calibration {
    test_value: u64,
    equation_values: Vec<u64>,
}

impl Calibration {
    fn new(calibration: &str) -> Self {
        //3267: 81 40 27
        let [test_value, equation_values] = calibration.split(": ").collect_vec()[..] else {
            panic!("Expected two parts but got {}", calibration);
        };

        let test_value = test_value.parse::<u64>().unwrap();
        let equation_values = equation_values
            .split(' ')
            .map(|s| s.parse::<u64>().unwrap())
            .collect();

        Self {
            test_value,
            equation_values,
        }
    }

    fn can_solve(&self) -> bool {
        Self::can_solve_helper(&self.equation_values)
            .iter()
            .any(|&equation_result| equation_result == self.test_value)
    }

    fn can_solve_helper(equation_values: &[u64]) -> Vec<u64> {
        if equation_values.len() == 1 {
            return equation_values.to_vec();
        }

        let add = {
            let new_equation_values = iter::once(equation_values[0] + equation_values[1])
                .chain(equation_values.iter().skip(2).copied())
                .collect_vec();
            Self::can_solve_helper(&new_equation_values)
        };

        let multiply = {
            let new_equation_values = iter::once(equation_values[0] * equation_values[1])
                .chain(equation_values.iter().skip(2).copied())
                .collect_vec();
            Self::can_solve_helper(&new_equation_values)
        };

        [add, multiply].concat()
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
}
