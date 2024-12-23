use itertools::Itertools;
use std::cmp::Ordering;

#[derive(Debug)]
pub struct UnionFind {
    parents: Vec<usize>,
    ranks: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        UnionFind {
            parents: (0..n).collect(),
            ranks: vec![0; n],
        }
    }

    pub fn find(&mut self, idx: usize) -> usize {
        // Path compression
        if self.parents[idx] != idx {
            self.parents[idx] = self.find(self.parents[idx]);
        }
        self.parents[idx]
    }

    pub fn union(&mut self, idx_1: usize, idx_2: usize) {
        let root_1 = self.find(idx_1);
        let root_2 = self.find(idx_2);

        if root_1 != root_2 {
            // Union by rank
            match self.ranks[root_1].cmp(&self.ranks[root_2]) {
                Ordering::Less => self.parents[root_1] = root_2,
                Ordering::Greater => self.parents[root_2] = root_1,
                Ordering::Equal => {
                    self.parents[root_2] = root_1;
                    self.ranks[root_1] += 1;
                }
            }
        }
    }

    pub fn sets(&mut self) -> Vec<Vec<usize>> {
        (0..self.parents.len())
            .map(|idx| (self.find(idx), idx))
            .sorted_by_key(|&(root, _)| root)
            .chunk_by(|&(root, _)| root)
            .into_iter()
            .map(|(_, set)| set.into_iter().map(move |(_, idx)| idx).collect())
            .collect()
    }
}
