pub fn solve_1(stones: &str) -> usize {
    let mut stones: Vec<_> = stones
        .split(" ")
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

    for _ in 0..25 {
        let mut new_stones: Vec<u64> = Vec::new();

        for stone in stones {
            if stone == 0 {
                new_stones.push(1);
            } else if nr_digits(stone) % 2 == 0 {
                let (left, right) = split_stone(stone);
                new_stones.push(left);
                new_stones.push(right);
            } else {
                new_stones.push(stone * 2024)
            }
        }

        stones = new_stones;
    }

    stones.len()
}

fn nr_digits(n: u64) -> u32 {
    match n {
        0..=9 => 1,
        10..=99 => 2,
        100..=999 => 3,
        1_000..=9_999 => 4,
        10_000..=99_999 => 5,
        100_000..=999_999 => 6,
        1_000_000..=9_999_999 => 7,
        10_000_000..=99_999_999 => 8,
        100_000_000..=999_999_999 => 9,
        1_000_000_000..=9_999_999_999 => 10,
        10_000_000_000..=99_999_999_999 => 11,
        100_000_000_000..=999_999_999_999 => 12,
        1_000_000_000_000..=9_999_999_999_999 => 13,
        10_000_000_000_000..=99_999_999_999_999 => 14,
        100_000_000_000_000..=999_999_999_999_999 => 15,
        1_000_000_000_000_000..=9_999_999_999_999_999 => 16,
        10_000_000_000_000_000..=99_999_999_999_999_999 => 17,
        100_000_000_000_000_000..=999_999_999_999_999_999 => 18,
        1_000_000_000_000_000_000..=9_999_999_999_999_999_999 => 19,
        10_000_000_000_000_000_000..=u64::MAX => 20,
    }
}

fn split_stone(stone: u64) -> (u64, u64) {
    let nr_digits = nr_digits(stone);
    let zeroes = 10u64.pow(nr_digits / 2);

    (stone / zeroes, stone % zeroes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_11_part_01_sample() {
        let sample = "125 17";

        assert_eq!(55_312, solve_1(sample));
    }

    #[test]
    fn day_11_part_01_solution() {
        let input = include_str!("../../inputs/day_11.txt").trim();

        assert_eq!(183_248, solve_1(input));
    }
}
