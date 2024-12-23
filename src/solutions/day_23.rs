use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};
use std::fmt::{Display, Formatter};
use std::iter;
use std::ops::Not;

pub fn solve_1(connections: &[&str]) -> usize {
    let sets = Network::new(connections).sets();
    let threes = &sets[3];

    threes.iter().filter(|&set| set.starts_with_t()).count()
}

pub fn solve_2(connections: &[&str]) -> String {
    Network::new(connections)
        .sets()
        .last()
        .filter(|sets| sets.len() == 1)
        .unwrap()
        .first()
        .unwrap()
        .to_string()
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

    fn sets(&self) -> Vec<Vec<Set>> {
        let mut connected_sets = vec![vec![]];

        let mut computers = self
            .connections
            .keys()
            .map(|computer| Set::new(computer))
            .collect_vec();

        connected_sets.push(computers.clone());

        // TODO, re-use iter::successors from Day22
        loop {
            computers = computers
                .iter()
                .flat_map(|set| set.grow(self))
                .unique()
                .collect_vec();

            if computers.is_empty() {
                return connected_sets;
            }

            connected_sets.push(computers.clone());
        }
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Set {
    computers: Vec<String>,
}

impl Set {
    fn new(a: &str) -> Self {
        Self {
            computers: iter::once(a.to_owned()).collect(),
        }
    }

    fn grow(&self, network: &Network) -> Vec<Self> {
        let set: FxHashSet<_> = self.computers.iter().cloned().collect();

        set.iter()
            .flat_map(|set| network.connections[set].clone())
            .filter(|computer| set.contains(computer).not())
            .map(|candidate| (candidate.clone(), network.connections[&candidate].clone()))
            .filter(|(_, connections)| set.iter().all(|computer| connections.contains(computer)))
            .map(|(candidate, _)| Set {
                computers: self
                    .computers
                    .iter()
                    .chain(iter::once(&candidate))
                    .cloned()
                    .sorted()
                    .collect(),
            })
            .collect()
    }

    fn starts_with_t(&self) -> bool {
        self.computers
            .iter()
            .any(|computer| computer.starts_with("t"))
    }
}

impl Display for Set {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let print = self.computers.iter().join(",");
        write!(f, "{}", print)
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

    #[ignore] // Too slow (but correct), need to rework
    #[test]
    fn day_23_part_01_solution() {
        let input = include_str!("../../inputs/day_23.txt")
            .lines()
            .collect_vec();

        assert_eq!(1_194, solve_1(&input));
    }

    #[test]
    fn day_23_part_02_sample() {
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

        assert_eq!("co,de,ka,ta", solve_2(&sample));
    }

    #[ignore] // Too slow (but correct), need to rework
    #[test]
    fn day_23_part_02_solution() {
        let input = include_str!("../../inputs/day_23.txt")
            .lines()
            .collect_vec();

        assert_eq!("bd,bu,dv,gl,qc,rn,so,tm,wf,yl,ys,ze,zr", solve_2(&input));
    }
}
