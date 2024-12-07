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
    concat: bool,
}

impl Calibration {
    fn new(calibration: &str, concat: bool) -> Self {
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
            let results = if self.concat {
                vec![
                    acc + self.equation_values[idx],
                    acc * self.equation_values[idx],
                    Self::concat(acc, self.equation_values[idx]),
                ]
            } else {
                vec![
                    acc + self.equation_values[idx],
                    acc * self.equation_values[idx],
                ]
            };

            results
                .iter()
                .any(|new_acc| self.can_solve_helper(idx + 1, *new_acc))
        }
    }

    fn concat(l: u64, r: u64) -> u64 {
        let mut l_cpy = l;
        let mut r_cpy = r;

        while r_cpy > 0 {
            r_cpy /= 10;
            l_cpy *= 10;
        }

        l_cpy + r
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
