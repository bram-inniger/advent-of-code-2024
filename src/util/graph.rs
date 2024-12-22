use num_traits::bounds::UpperBounded;
use rustc_hash::FxHashMap;
use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;
use std::hash::Hash;
use std::ops::Add;

#[derive(Debug)]
pub struct Graph<N, W> {
    edges: FxHashMap<N, Vec<(N, W)>>,
}

impl<N, W> Graph<N, W>
where
    N: Ord + Eq + Hash + Clone,
    W: Copy + Ord + Default + Add<Output = W> + UpperBounded,
{
    fn new() -> Self {
        Graph {
            edges: FxHashMap::default(),
        }
    }

    pub fn add_edge(&mut self, from: &N, to: &N, weight: &W) {
        self.edges
            .entry(from.clone())
            .or_default()
            .push((to.clone(), *weight));
        self.edges.entry(to.clone()).or_default();
    }

    pub fn dijkstra(&self, start: &N) -> Dijkstra<N, W> {
        let mut distances: FxHashMap<N, W> = FxHashMap::default();
        let mut parents: FxHashMap<N, Vec<N>> = self
            .edges
            .keys()
            .map(|node| (node.clone(), vec![]))
            .collect();

        let mut pq: BinaryHeap<Reverse<(W, N)>> = BinaryHeap::new();

        distances.insert(start.clone(), W::default());
        pq.push(Reverse((W::default(), start.clone())));

        while let Some(Reverse((current_distance, current_node))) = pq.pop() {
            if let Some(&known_distance) = distances.get(&current_node) {
                if current_distance > known_distance {
                    continue;
                }
            }

            if let Some(neighbours) = self.edges.get(&current_node) {
                for (neighbour_node, weight) in neighbours {
                    let old_distance = *distances.get(neighbour_node).unwrap_or(&W::max_value());
                    let new_distance = current_distance + *weight;

                    match old_distance.cmp(&new_distance) {
                        Ordering::Less => {}
                        Ordering::Equal => {
                            parents
                                .get_mut(neighbour_node)
                                .unwrap()
                                .push(current_node.clone());
                        }
                        Ordering::Greater => {
                            distances.insert(neighbour_node.clone(), new_distance);
                            pq.push(Reverse((new_distance, neighbour_node.clone())));
                            parents.insert(neighbour_node.clone(), vec![current_node.clone()]);
                        }
                    }
                }
            }
        }

        Dijkstra { distances, parents }
    }
}

impl<N, W> Default for Graph<N, W>
where
    N: Ord + Eq + Hash + Clone,
    W: Copy + Ord + Default + Add<Output = W> + UpperBounded,
{
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct Dijkstra<N, W> {
    distances: FxHashMap<N, W>,
    parents: FxHashMap<N, Vec<N>>,
}

impl<N, W> Dijkstra<N, W>
where
    N: Ord + Eq + Hash + Clone,
    W: Copy + Ord + Default + Add<Output = W> + UpperBounded,
{
    pub fn distance(&self, node: &N) -> W {
        self.distances[node]
    }

    pub fn shortest_paths(&self, to: &N) -> Vec<Vec<N>> {
        self.shortest_paths_helper(to, &[])
    }

    fn shortest_paths_helper(&self, node: &N, path: &[N]) -> Vec<Vec<N>> {
        let mut new_path = path.to_vec();
        new_path.push(node.clone());

        if self.parents[node].is_empty() {
            return vec![new_path];
        }

        self.parents[node]
            .iter()
            .flat_map(|parent| self.shortest_paths_helper(parent, &new_path))
            .collect()
    }
}
