use crate::util::clique::Clique;
use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};

pub fn solve_1(connections: &[&str]) -> usize {
    let network = Network::new(connections);

    Clique::new(&network.connections)
        .cliques(Some(3))
        .iter()
        .filter(|set| {
            set.iter()
                .any(|computer| network.translate(computer).starts_with("t"))
        })
        .count()
}

pub fn solve_2(connections: &[&str]) -> String {
    let network = Network::new(connections);

    Clique::new(&network.connections)
        .cliques(None)
        .iter()
        .max_by_key(|set| set.len())
        .map(|set| set.iter().map(|c| network.translate(c)).sorted().join(","))
        .unwrap()
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

    fn translate(&self, id: &usize) -> String {
        self.id_to_name[id].to_owned()
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
