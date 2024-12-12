use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::ops::Not;

pub fn solve_1(garden: &[&str]) -> u32 {
    solve(garden, false)
}

pub fn solve_2(garden: &[&str]) -> u32 {
    solve(garden, true)
}

fn solve(garden: &[&str], discount: bool) -> u32 {
    Garden::new(garden)
        .regions()
        .iter()
        .map(|r| r.price(discount))
        .sum()
}

#[derive(Debug)]
struct Garden {
    plants: Vec<char>,
    width: usize,
    height: usize,
}

impl Garden {
    fn new(garden: &[&str]) -> Self {
        let width = garden[0].len();
        let height = garden.len();
        let plants = garden.iter().flat_map(|line| line.chars()).collect();

        Self {
            plants,
            width,
            height,
        }
    }

    fn regions(&self) -> Vec<Region> {
        let mut uf = UnionFind::new(self.plants.len());

        for y in 0..self.height {
            for x in 0..self.width {
                let current = y * self.width + x;
                let right = y * self.width + (x + 1);
                let under = (y + 1) * self.width + x;

                if x < self.width - 1 && self.plants[current] == self.plants[right] {
                    uf.union(current, right);
                }
                if y < self.height - 1 && self.plants[current] == self.plants[under] {
                    uf.union(current, under);
                }
            }
        }

        uf.sets()
            .iter()
            .map(|set| {
                let _plant_name = self.plants[set[0]];
                let plants = set
                    .iter()
                    .map(|&plant| Coordinate::from_idx(plant, self.width))
                    .collect();

                Region {
                    _plant_name,
                    plants,
                }
            })
            .collect()
    }
}

#[derive(Debug)]
struct Region {
    _plant_name: char,
    plants: HashSet<Coordinate>,
}

impl Region {
    fn price(&self, discount: bool) -> u32 {
        let area = self.plants.len() as u32;
        let fencing = if discount {
            self.sides()
        } else {
            self.perimeter()
        };

        area * fencing
    }

    fn perimeter(&self) -> u32 {
        self.plants
            .iter()
            .map(|plant| {
                4 - plant
                    .neighbours()
                    .iter()
                    .map(|(neighbour, _)| neighbour)
                    .filter(|neighbour| self.plants.contains(neighbour))
                    .count()
            })
            .sum::<usize>() as u32
    }

    fn sides(&self) -> u32 {
        let sides: HashMap<_, _> = self
            .plants
            .iter()
            .flat_map(|plant| plant.sides(self))
            .enumerate()
            .map(|(idx, side)| (side, idx))
            .collect();

        let mut uf = UnionFind::new(sides.len());

        for (side, idx) in &sides {
            let neighbour = match side.orientation {
                Orientation::HorizontalUp | Orientation::HorizontalDown => side.right_neighbour(),
                Orientation::VerticalLeft | Orientation::VerticalRight => side.under_neighbour(),
            };
            if let Some(neighbour_idx) = sides.get(&neighbour) {
                uf.union(*idx, *neighbour_idx);
            }
        }

        uf.sets().len() as u32
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    fn neighbours(&self) -> Vec<(Self, Direction)> {
        #[rustfmt::skip]
        let neighbours = vec![
            (Self { x: self.x + 1, y: self.y, }, Direction::Right),
            (Self { x: self.x - 1, y: self.y, }, Direction::Left),
            (Self { x: self.x, y: self.y + 1, }, Direction::Down),
            (Self { x: self.x, y: self.y - 1, }, Direction::Up),
        ];

        neighbours
    }

    fn from_idx(idx: usize, width: usize) -> Self {
        Coordinate {
            x: (idx % width) as i32,
            y: (idx / width) as i32,
        }
    }

    fn sides(&self, region: &Region) -> Vec<Side> {
        self.neighbours()
            .iter()
            .filter(|(neighbour, _)| region.plants.contains(neighbour).not())
            .map(|(_, direction)| {
                let orientation = direction.as_orientation();
                let x = match orientation {
                    Orientation::VerticalRight => self.x + 1,
                    _ => self.x,
                };
                let y = match orientation {
                    Orientation::HorizontalDown => self.y + 1,
                    _ => self.y,
                };

                Side {
                    start: Coordinate { x, y },
                    orientation,
                }
            })
            .collect()
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Side {
    start: Coordinate,
    orientation: Orientation,
}

impl Side {
    fn right_neighbour(&self) -> Self {
        Self {
            start: Coordinate {
                x: self.start.x + 1,
                y: self.start.y,
            },
            orientation: self.orientation,
        }
    }

    fn under_neighbour(&self) -> Self {
        Self {
            start: Coordinate {
                x: self.start.x,
                y: self.start.y + 1,
            },
            orientation: self.orientation,
        }
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum Orientation {
    HorizontalUp,
    HorizontalDown,
    VerticalLeft,
    VerticalRight,
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn as_orientation(&self) -> Orientation {
        match self {
            Direction::Up => Orientation::HorizontalUp,
            Direction::Right => Orientation::VerticalRight,
            Direction::Down => Orientation::HorizontalDown,
            Direction::Left => Orientation::VerticalLeft,
        }
    }
}

#[derive(Debug)]
struct UnionFind {
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

    fn find(&mut self, idx: usize) -> usize {
        // Path compression
        if self.parents[idx] != idx {
            self.parents[idx] = self.find(self.parents[idx]);
        }
        self.parents[idx]
    }

    fn union(&mut self, idx_1: usize, idx_2: usize) {
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

    fn sets(&mut self) -> Vec<Vec<usize>> {
        (0..self.parents.len())
            .map(|idx| (self.find(idx), idx))
            .sorted_by_key(|&(root, _)| root)
            .chunk_by(|&(root, _)| root)
            .into_iter()
            .map(|(_, set)| set.into_iter().map(move |(_, idx)| idx).collect())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn day_12_part_01_sample() {
        #[rustfmt::skip]
        let sample_1 = vec![
            "AAAA",
            "BBCD",
            "BBCC",
            "EEEC",
        ];
        #[rustfmt::skip]
        let sample_2 = vec![
            "OOOOO",
            "OXOXO",
            "OOOOO",
            "OXOXO",
            "OOOOO",
        ];
        #[rustfmt::skip]
        let sample_3 = vec![
            "RRRRIICCFF",
            "RRRRIICCCF",
            "VVRRRCCFFF",
            "VVRCCCJFFF",
            "VVVVCJJCFE",
            "VVIVCCJJEE",
            "VVIIICJJEE",
            "MIIIIIJJEE",
            "MIIISIJEEE",
            "MMMISSJEEE",
        ];

        assert_eq!(140, solve_1(&sample_1));
        assert_eq!(772, solve_1(&sample_2));
        assert_eq!(1_930, solve_1(&sample_3));
    }

    #[test]
    fn day_12_part_01_solution() {
        let input = include_str!("../../inputs/day_12.txt")
            .lines()
            .collect_vec();

        assert_eq!(1_437_300, solve_1(&input));
    }

    #[test]
    fn day_12_part_02_sample() {
        #[rustfmt::skip]
        let sample_1 = vec![
            "AAAA",
            "BBCD",
            "BBCC",
            "EEEC",
        ];
        #[rustfmt::skip]
        let sample_2 = vec![
            "OOOOO",
            "OXOXO",
            "OOOOO",
            "OXOXO",
            "OOOOO",
        ];
        #[rustfmt::skip]
        let sample_3 = vec![
            "EEEEE",
            "EXXXX",
            "EEEEE",
            "EXXXX",
            "EEEEE",
        ];
        #[rustfmt::skip]
        let sample_4 = vec![
            "AAAAAA",
            "AAABBA",
            "AAABBA",
            "ABBAAA",
            "ABBAAA",
            "AAAAAA",
        ];
        #[rustfmt::skip]
        let sample_5 = vec![
            "RRRRIICCFF",
            "RRRRIICCCF",
            "VVRRRCCFFF",
            "VVRCCCJFFF",
            "VVVVCJJCFE",
            "VVIVCCJJEE",
            "VVIIICJJEE",
            "MIIIIIJJEE",
            "MIIISIJEEE",
            "MMMISSJEEE",
        ];

        assert_eq!(80, solve_2(&sample_1));
        assert_eq!(436, solve_2(&sample_2));
        assert_eq!(236, solve_2(&sample_3));
        assert_eq!(368, solve_2(&sample_4));
        assert_eq!(1_206, solve_2(&sample_5));
    }

    #[test]
    fn day_12_part_02_solution() {
        let input = include_str!("../../inputs/day_12.txt")
            .lines()
            .collect_vec();

        assert_eq!(849_332, solve_2(&input));
    }
}
