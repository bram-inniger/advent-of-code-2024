use itertools::Itertools;
use rustc_hash::FxHashMap;

pub fn solve_1(towels: &[&str]) -> usize {
    let patterns = towels[0].split(", ").collect_vec();
    let designs = towels.iter().skip(2).copied().collect_vec();

    designs
        .iter()
        .filter(|design| ways_count(design, &patterns, &mut FxHashMap::default()) > 0)
        .count()
}

pub fn solve_2(towels: &[&str]) -> u64 {
    let patterns = towels[0].split(", ").collect_vec();
    let designs = towels.iter().skip(2).copied().collect_vec();

    designs
        .iter()
        .map(|design| ways_count(design, &patterns, &mut FxHashMap::default()))
        .sum()
}

fn ways_count(design: &str, patterns: &[&str], cache: &mut FxHashMap<usize, u64>) -> u64 {
    if let Some(count) = cache.get(&design.len()) {
        return *count;
    }

    let count = if design.is_empty() {
        1
    } else {
        patterns
            .iter()
            .filter(|&pattern| design.starts_with(pattern))
            .map(|&pattern| ways_count(&design[pattern.len()..], patterns, cache))
            .sum()
    };

    cache.insert(design.len(), count);

    count
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

    #[test]
    fn day_19_part_02_sample() {
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

        assert_eq!(16, solve_2(&sample));
    }

    #[test]
    fn day_19_part_02_solution() {
        let input = include_str!("../../inputs/day_19.txt")
            .lines()
            .collect_vec();

        assert_eq!(565_600_047_715_343, solve_2(&input));
    }
}
