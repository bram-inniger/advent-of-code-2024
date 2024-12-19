use itertools::Itertools;

pub fn solve_1(towels: &[&str]) -> usize {
    let patterns = towels[0].split(", ").collect_vec();
    let designs = towels.iter().skip(2).copied().collect_vec();

    designs
        .iter()
        .filter(|design| is_valid(design, &patterns))
        .count()
}

fn is_valid(design: &str, patterns: &[&str]) -> bool {
    design.is_empty()
        || patterns
            .iter()
            .filter(|&pattern| design.starts_with(pattern))
            .any(|&pattern| is_valid(&design[pattern.len()..], patterns))
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn day_19_part_01_sample() {
        let sample = vec![
            "r, wr, b, g, bwu, rb, gb, br",
            "",
            "brwrr",
            "bggr",
            "gbbr",
            "rrbgbr",
            "ubwu",
            "bwurrg",
            "brgr",
            "bbrgwb",
        ];

        assert_eq!(6, solve_1(&sample));
    }

    #[test]
    fn day_19_part_01_solution() {
        let input = include_str!("../../inputs/day_19.txt")
            .lines()
            .collect_vec();

        assert_eq!(220, solve_1(&input));
    }
}
