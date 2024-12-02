use itertools::Itertools;

pub fn solve_1(locations: &str) -> u32 {
    let (mut left, mut right) = parse_locations(locations);

    left.sort();
    right.sort();

    left.iter()
        .zip_eq(right.iter())
        .map(|(&a, &b)| a.abs_diff(b))
        .sum()
}

pub fn solve_2(locations: &str) -> usize {
    let (left, right) = parse_locations(locations);
    let frequencies = right.iter().counts();

    left.iter()
        .map(|num| (*num as usize) * frequencies.get(num).unwrap_or(&0usize))
        .sum()
}

fn parse_locations(locations: &str) -> (Vec<i32>, Vec<i32>) {
    let mut left = vec![];
    let mut right = vec![];

    for line in locations.lines() {
        let numbers = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        left.push(numbers[0]);
        right.push(numbers[1]);
    }

    (left, right)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_01_part_01_sample() {
        let sample = "\
                3   4
                4   3
                2   5
                1   3
                3   9
                3   3\
            ";

        assert_eq!(11, solve_1(sample));
    }

    #[test]
    fn day_01_part_01_solution() {
        let input = include_str!("../../inputs/day_01.txt").trim();

        assert_eq!(2_113_135, solve_1(input));
    }

    #[test]
    fn day_01_part_02_sample() {
        let sample = "\
                3   4
                4   3
                2   5
                1   3
                3   9
                3   3\
            ";

        assert_eq!(31, solve_2(sample));
    }

    #[test]
    fn day_01_part_02_solution() {
        let input = include_str!("../../inputs/day_01.txt").trim();

        assert_eq!(19_097_157, solve_2(input));
    }
}
