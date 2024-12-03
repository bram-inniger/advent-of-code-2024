use itertools::Itertools;
use regex::Regex;

pub fn solve_1(memory: &str) -> u32 {
    Regex::new(r"mul\((?<a>\d{1,3}),(?<b>\d{1,3})\)")
        .unwrap()
        .captures_iter(memory)
        .map(|caps| {
            let a = caps.name("a").unwrap().as_str().parse::<u32>().unwrap();
            let b = caps.name("b").unwrap().as_str().parse::<u32>().unwrap();
            a * b
        })
        .sum()
}

pub fn solve_2(memory: &str) -> u32 {
    let re_mul = Regex::new(r"mul\((?<a>\d{1,3}),(?<b>\d{1,3})\)").unwrap();
    let instructions = Regex::new(r"do\(\)|don't\(\)|mul\(\d{1,3},\d{1,3}\)")
        .unwrap()
        .find_iter(memory)
        .map(|m| m.as_str())
        .collect_vec();

    let mut sum = 0;
    let mut enabled = true;

    for instruction in instructions {
        match instruction {
            "do()" => enabled = true,
            "don't()" => enabled = false,
            _ => {
                if enabled {
                    let caps = re_mul.captures(instruction).unwrap();
                    let a = caps.name("a").unwrap().as_str().parse::<u32>().unwrap();
                    let b = caps.name("b").unwrap().as_str().parse::<u32>().unwrap();

                    sum += a * b;
                }
            }
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_03_part_01_sample() {
        let sample = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

        assert_eq!(161, solve_1(sample));
    }

    #[test]
    fn day_03_part_01_solution() {
        let input = include_str!("../../inputs/day_03.txt").trim();

        assert_eq!(183_380_722, solve_1(input));
    }

    #[test]
    fn day_03_part_02_sample() {
        let sample = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

        assert_eq!(48, solve_2(sample));
    }

    #[test]
    fn day_03_part_02_solution() {
        let input = include_str!("../../inputs/day_03.txt").trim();

        assert_eq!(82_733_683, solve_2(input));
    }
}
