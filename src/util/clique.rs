use rustc_hash::{FxHashMap, FxHashSet};

#[derive(Debug)]
pub struct Clique {
    edges: FxHashMap<usize, FxHashSet<usize>>,
}

impl Clique {
    pub fn new(edges: &FxHashMap<usize, FxHashSet<usize>>) -> Self {
        Self {
            edges: edges.clone(),
        }
    }

    pub fn cliques(&self, clique_len: Option<usize>) -> Vec<FxHashSet<usize>> {
        let mut cliques = Vec::new();

        let r = FxHashSet::default();
        let p: FxHashSet<usize> = self.edges.keys().cloned().collect();
        let x = FxHashSet::default();

        if let Some(clique_len) = clique_len {
            self.bron_kerbosch_no_pivot(r, p, x, &mut cliques, clique_len);
        } else {
            self.bron_kerbosch(r, p, x, &mut cliques);
        }

        cliques
    }

    fn bron_kerbosch(
        &self,
        r: FxHashSet<usize>,
        p: FxHashSet<usize>,
        x: FxHashSet<usize>,
        max_cliques: &mut Vec<FxHashSet<usize>>,
    ) {
        if p.is_empty() && x.is_empty() {
            max_cliques.push(r);
            return;
        }

        let u = *p.union(&x).next().unwrap();

        for &v in p.difference(self.edges.get(&u).unwrap_or(&FxHashSet::default())) {
            let mut next_r = r.clone();
            next_r.insert(v);

            let next_p = p
                .intersection(self.edges.get(&v).unwrap_or(&FxHashSet::default()))
                .copied()
                .collect();
            let next_x = x
                .intersection(self.edges.get(&v).unwrap_or(&FxHashSet::default()))
                .copied()
                .collect();

            self.bron_kerbosch(next_r, next_p, next_x, max_cliques);
        }
    }

    fn bron_kerbosch_no_pivot(
        &self,
        r: FxHashSet<usize>,
        p: FxHashSet<usize>,
        x: FxHashSet<usize>,
        cliques: &mut Vec<FxHashSet<usize>>,
        clique_len: usize,
    ) {
        if r.len() == clique_len {
            cliques.push(r);
            return;
        }

        let mut p = p.clone();

        while let Some(&v) = p.iter().next() {
            p.remove(&v);

            let mut next_r = r.clone();
            next_r.insert(v);

            let next_p = p
                .intersection(self.edges.get(&v).unwrap_or(&FxHashSet::default()))
                .copied()
                .collect();
            let next_x = x
                .intersection(self.edges.get(&v).unwrap_or(&FxHashSet::default()))
                .copied()
                .collect();

            self.bron_kerbosch_no_pivot(next_r, next_p, next_x, cliques, clique_len);
        }
    }
}
