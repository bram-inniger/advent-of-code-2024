use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub fn solve_1(manual: &str) -> u32 {
    let [rules, updates] = manual.split("\n\n").collect_vec()[..] else {
        panic!("Expected two parts but got {}", manual);
    };

    let rules = parse_rules(rules);
    let updates = parse_updates(updates);

    updates
        .iter()
        .filter(|&update| is_valid(update, &rules))
        .map(|update| update[update.len() / 2])
        .sum()
}

fn parse_rules(rules: &str) -> HashMap<u32, Vec<u32>> {
    rules
        .split('\n')
        .map(|rule| {
            let [before, after] = rule
                .split('|')
                .map(|n| n.parse::<u32>().unwrap())
                .collect_vec()[..]
            else {
                panic!("Expected a rule but got {}", rule)
            };
            (before, after)
        })
        .sorted_by_key(|&(before, _)| before)
        .chunk_by(|&(before, _)| before)
        .into_iter()
        .map(|(before, afters)| (before, afters.map(|(_, after)| after).collect()))
        .collect()
}

fn parse_updates(updates: &str) -> Vec<Vec<u32>> {
    updates
        .split('\n')
        .map(|update| {
            update
                .split(',')
                .map(|n| n.parse::<u32>().unwrap())
                .collect()
        })
        .collect()
}

fn is_valid(update: &[u32], rules: &HashMap<u32, Vec<u32>>) -> bool {
    let mut seen: HashSet<u32> = HashSet::new();

    for page in update {
        if rules
            .get(page)
            .unwrap_or(&vec![])
            .iter()
            .any(|after| seen.contains(after))
        {
            return false;
        }

        seen.insert(*page);
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_05_part_01_sample() {
        let sample = "\
                47|53\n\
                97|13\n\
                97|61\n\
                97|47\n\
                75|29\n\
                61|13\n\
                75|53\n\
                29|13\n\
                97|29\n\
                53|29\n\
                61|53\n\
                97|53\n\
                61|29\n\
                47|13\n\
                75|47\n\
                97|75\n\
                47|61\n\
                75|61\n\
                47|29\n\
                75|13\n\
                53|13\n\
                \n\
                75,47,61,53,29\n\
                97,61,53,29,13\n\
                75,29,13\n\
                75,97,47,61,53\n\
                61,13,29\n\
                97,13,75,29,47\
            ";

        assert_eq!(143, solve_1(sample));
    }

    #[test]
    fn day_05_part_01_solution() {
        let input = include_str!("../../inputs/day_05.txt").trim();

        assert_eq!(6_034, solve_1(input));
    }
}
