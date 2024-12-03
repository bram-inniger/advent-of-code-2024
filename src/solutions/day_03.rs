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
}
