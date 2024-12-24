use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};
use std::iter;
use std::ops::Not;

pub fn solve_1(connections: &[&str]) -> usize {
    let network = Network::new(connections);
    network.sets()[3 - 1]
        .iter()
        .filter(|&set| set.starts_with_t(&network))
        .count()
}

pub fn solve_2(connections: &[&str]) -> String {
    let network = Network::new(connections);
    network
        .sets()
        .last()
        .filter(|sets| sets.len() == 1)
        .unwrap()
        .first()
        .unwrap()
        .pretty_print(&network)
}

#[derive(Debug)]
struct Network {
    connections: FxHashMap<usize, FxHashSet<usize>>,
    id_to_name: FxHashMap<usize, String>,
}

impl Network {
    fn new(connections: &[&str]) -> Self {
        let id_to_name: FxHashMap<usize, String> = connections
            .iter()
            .flat_map(|connection| connection.split('-'))
            .unique()
            .enumerate()
            .map(|(id, name)| (id, name.to_owned()))
            .collect();
        let name_to_id: FxHashMap<String, usize> = id_to_name
            .iter()
            .map(|(id, name)| (name.to_owned(), *id))
            .collect();

        let connections = connections
            .iter()
            .flat_map(|connection| {
                let (from, to) = connection
                    .split_once('-')
                    .map(|(from, to)| (name_to_id[from], name_to_id[to]))
                    .unwrap();
                [(from, to), (to, from)]
            })
            .sorted_by_key(|&(from, _)| from)
            .chunk_by(|&(from, _)| from)
            .into_iter()
            .map(|(from, group)| (from, group.into_iter().map(|(_, to)| to).collect()))
            .collect();

        Self {
            connections,
            id_to_name,
        }
    }

    fn sets(&self) -> Vec<Vec<Set>> {
        let computers = self
            .connections
            .keys()
            .map(|computer| Set::new(*computer))
            .collect_vec();

        iter::successors(Some(computers), |computers| {
            let new_computers = computers
                .iter()
                .flat_map(|set| set.grow(self))
                .unique()
                .collect_vec();
            if new_computers.is_empty() {
                None
            } else {
                Some(new_computers)
            }
        })
        .collect()
    }

    fn translate(&self, id: &usize) -> String {
        self.id_to_name[id].to_owned()
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Set {
    computers: Vec<usize>,
}

impl Set {
    fn new(single: usize) -> Self {
        Self {
            computers: iter::once(single).collect(),
        }
    }

    fn grow(&self, network: &Network) -> Vec<Self> {
        let set: FxHashSet<_> = self.computers.iter().copied().collect();

        set.iter()
            .flat_map(|set| &network.connections[set])
            .filter(|computer| set.contains(computer).not())
            .map(|candidate| (candidate, &network.connections[candidate]))
            .filter(|(_, connections)| set.iter().all(|computer| connections.contains(computer)))
            .map(|(candidate, _)| Set {
                computers: self
                    .computers
                    .iter()
                    .chain(iter::once(candidate))
                    .copied()
                    .sorted()
                    .collect(),
            })
            .collect()
    }

    fn starts_with_t(&self, network: &Network) -> bool {
        self.computers
            .iter()
            .any(|computer| network.translate(computer).starts_with("t"))
    }

    fn pretty_print(&self, network: &Network) -> String {
        self.computers
            .iter()
            .map(|c| network.translate(c))
            .sorted()
            .join(",")
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

    #[test]
    fn day_23_part_02_solution() {
        let input = include_str!("../../inputs/day_23.txt")
            .lines()
            .collect_vec();

        assert_eq!("bd,bu,dv,gl,qc,rn,so,tm,wf,yl,ys,ze,zr", solve_2(&input));
    }
}
