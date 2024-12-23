use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};

pub fn solve_1(connections: &[&str]) -> usize {
    Network::new(connections)
        .triplets()
        .iter()
        .filter(|set| set.starts_with_t())
        .unique()
        .count()
}

#[derive(Debug)]
struct Network {
    connections: FxHashMap<String, FxHashSet<String>>,
}

impl Network {
    fn new(connections: &[&str]) -> Self {
        let connections = connections
            .iter()
            .flat_map(|connection| {
                let (from, to) = connection
                    .split_once('-')
                    .map(|(from, to)| (from.to_string(), to.to_string()))
                    .unwrap();

                [(from.clone(), to.clone()), (to, from)]
            })
            .sorted_by_key(|(from, _)| from.clone())
            .chunk_by(|(from, _)| from.clone())
            .into_iter()
            .map(|(from, group)| (from, group.into_iter().map(|(_, to)| to).collect()))
            .collect();

        Self { connections }
    }

    fn triplets(&self) -> Vec<Set> {
        self.connections
            .keys()
            .flat_map(|a| self.connections[a].iter().map(move |b| (a, b)))
            .flat_map(|(a, b)| {
                self.connections[b]
                    .iter()
                    .filter(move |&c| c != a)
                    .filter(move |&c| self.connections[c].contains(a))
                    .map(move |c| Set::new(a, b, c))
            })
            .collect()
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Set {
    computers: [String; 3],
}

impl Set {
    fn new(a: &str, b: &str, c: &str) -> Self {
        let computers = [a, b, c]
            .iter()
            .map(|computer| computer.to_string())
            .sorted()
            .collect_vec()
            .try_into()
            .unwrap();
        Self { computers }
    }

    fn starts_with_t(&self) -> bool {
        self.computers
            .iter()
            .any(|computer| computer.starts_with("t"))
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn day_23_part_01_sample() {
        #[rustfmt::skip]
        let sample = vec![
            "kh-tc",
            "qp-kh",
            "de-cg",
            "ka-co",
            "yn-aq",
            "qp-ub",
            "cg-tb",
            "vc-aq",
            "tb-ka",
            "wh-tc",
            "yn-cg",
            "kh-ub",
            "ta-co",
            "de-co",
            "tc-td",
            "tb-wq",
            "wh-td",
            "ta-ka",
            "td-qp",
            "aq-cg",
            "wq-ub",
            "ub-vc",
            "de-ta",
            "wq-aq",
            "wq-vc",
            "wh-yn",
            "ka-de",
            "kh-ta",
            "co-tc",
            "wh-qp",
            "tb-vc",
            "td-yn",
        ];

        assert_eq!(7, solve_1(&sample));
    }

    #[test]
    fn day_23_part_01_solution() {
        let input = include_str!("../../inputs/day_23.txt")
            .lines()
            .collect_vec();

        assert_eq!(1_194, solve_1(&input));
    }
}
