use lazy_static::lazy_static;
use std::collections::{HashMap, HashSet, VecDeque};
use std::ops::Add;

pub fn solve_1(map: &[&str]) -> u32 {
    let map: HashMap<Position, u32> = map
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

    map.iter()
        .filter(|(_, height)| **height == 0)
        .map(|(trailhead, _)| trailhead)
        .map(|trailhead| trails_count(trailhead, &map))
        .sum()
}

fn trails_count(trailhead: &Position, map: &HashMap<Position, u32>) -> u32 {
    let mut to_visit: VecDeque<Position> = VecDeque::new();
    let mut seen: HashSet<Position> = HashSet::new();
    let mut trails = 0;

    to_visit.push_back(*trailhead);

    while let Some(position) = to_visit.pop_front() {
        if seen.contains(&position) {
            continue;
        }

        seen.insert(position);
        let height = *map.get(&position).unwrap();

        if height == 9 {
            trails += 1;
            continue;
        }

        NEIGHBOURS
            .iter()
            .map(|n_delta| *n_delta + position)
            .filter(|n| map.get(n).iter().any(|&&n_height| n_height == height + 1))
            .for_each(|n| to_visit.push_back(n));
    }

    trails
}

lazy_static! {
    static ref NEIGHBOURS: Vec<Position> = vec![
        Position { x: 0, y: 1 },
        Position { x: 0, y: -1 },
        Position { x: 1, y: 0 },
        Position { x: -1, y: 0 },
    ];
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
        let sample = vec![
            "89010123", "78121874", "87430965", "96549874", "45678903", "32019012", "01329801",
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
}
