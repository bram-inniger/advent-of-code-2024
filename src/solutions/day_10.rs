use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::VecDeque;
use std::ops::Add;

pub fn solve_1(map: &[&str]) -> u32 {
    Map::new(map).score(Scoring::SinglePath)
}

pub fn solve_2(map: &[&str]) -> u32 {
    Map::new(map).score(Scoring::Rating)
}

#[derive(Debug)]
struct Map {
    tiles: FxHashMap<Position, u32>,
    trailheads: Vec<Position>,
}

impl Map {
    fn new(map: &[&str]) -> Self {
        let tiles: FxHashMap<_, _> = map
            .iter()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().map(move |(x, c)| {
                    let position = Position {
                        x: x as i32,
                        y: y as i32,
                    };
                    let height = c.to_digit(10).unwrap();
                    (position, height)
                })
            })
            .collect();

        let trailheads = tiles
            .iter()
            .filter(|(_, height)| **height == 0)
            .map(|(trailhead, _)| *trailhead)
            .collect();

        Map { tiles, trailheads }
    }

    fn score(&self, scoring: Scoring) -> u32 {
        self.trailheads
            .iter()
            .map(|trailhead| self.score_trailhead(trailhead, scoring))
            .sum()
    }

    fn score_trailhead(&self, trailhead: &Position, scoring: Scoring) -> u32 {
        let mut to_visit: VecDeque<Position> = VecDeque::new();
        let mut seen: FxHashSet<Position> = FxHashSet::default();
        let mut trails = 0;

        to_visit.push_back(*trailhead);

        while let Some(position) = to_visit.pop_front() {
            if scoring == Scoring::SinglePath {
                if seen.contains(&position) {
                    continue;
                }

                seen.insert(position);
            }

            let height = *self.tiles.get(&position).unwrap();

            if height == 9 {
                trails += 1;
                continue;
            }

            [
                Position { x: 0, y: 1 },
                Position { x: 0, y: -1 },
                Position { x: 1, y: 0 },
                Position { x: -1, y: 0 },
            ]
            .into_iter()
            .map(|n_delta| n_delta + position)
            .filter(|n| {
                self.tiles
                    .get(n)
                    .iter()
                    .any(|&&n_height| n_height == height + 1)
            })
            .for_each(|n| to_visit.push_back(n));
        }

        trails
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum Scoring {
    SinglePath,
    Rating,
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

impl Add for Position {
    type Output = Position;

    fn add(self, rhs: Self) -> Self::Output {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn day_10_part_01_sample() {
        #[rustfmt::skip]
        let sample = vec![
            "89010123",
            "78121874",
            "87430965",
            "96549874",
            "45678903",
            "32019012",
            "01329801",
            "10456732",
        ];

        assert_eq!(36, solve_1(&sample));
    }

    #[test]
    fn day_10_part_01_solution() {
        let input = include_str!("../../inputs/day_10.txt")
            .lines()
            .collect_vec();

        assert_eq!(754, solve_1(&input));
    }

    #[test]
    fn day_10_part_02_sample() {
        #[rustfmt::skip]
        let sample = vec![
            "89010123",
            "78121874",
            "87430965",
            "96549874",
            "45678903",
            "32019012",
            "01329801",
            "10456732",
        ];

        assert_eq!(81, solve_2(&sample));
    }

    #[test]
    fn day_10_part_02_solution() {
        let input = include_str!("../../inputs/day_10.txt")
            .lines()
            .collect_vec();

        assert_eq!(1_609, solve_2(&input));
    }
}
