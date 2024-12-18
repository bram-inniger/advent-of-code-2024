use itertools::Itertools;
use rustc_hash::FxHashSet;
use std::collections::VecDeque;
use std::ops::Not;

pub fn solve_1(bytes: &[&str], nr_bytes: usize, max_dim: i32) -> u32 {
    distance_to_exit(bytes, nr_bytes, max_dim).unwrap()
}

pub fn solve_2(bytes: &[&str], max_dim: i32) -> String {
    let nr_bytes = (0..bytes.len())
        .collect_vec()
        .partition_point(|&nr_bytes| distance_to_exit(bytes, nr_bytes, max_dim).is_some());
    bytes[nr_bytes - 1].to_owned()
}

fn distance_to_exit(bytes: &[&str], nr_bytes: usize, max_dim: i32) -> Option<u32> {
    let corruption: FxHashSet<_> = bytes
        .iter()
        .take(nr_bytes)
        .map(|line| Position::new(line))
        .collect();

    let start = Position { x: 0, y: 0 };
    let end = Position {
        x: max_dim,
        y: max_dim,
    };

    let mut to_visit = VecDeque::<(Position, u32)>::new();
    to_visit.push_back((start, 0));
    let mut visited = FxHashSet::<Position>::default();

    while let Some((position, distance)) = to_visit.pop_front() {
        if position == end {
            return Some(distance);
        }

        if visited.contains(&position)
            || corruption.contains(&position)
            || (0..=max_dim).contains(&position.x).not()
            || (0..=max_dim).contains(&position.y).not()
        {
            continue;
        }

        visited.insert(position);
        [(1, 0), (-1, 0), (0, 1), (0, -1)]
            .map(|(dx, dy)| Position {
                x: position.x + dx,
                y: position.y + dy,
            })
            .into_iter()
            .for_each(|neighbour| to_visit.push_back((neighbour, distance + 1)));
    }

    None
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(position: &str) -> Self {
        let (x, y) = position
            .split_once(",")
            .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
            .unwrap();
        Self { x, y }
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn day_18_part_01_sample() {
        #[rustfmt::skip]
        let sample = vec![
            "5,4",
            "4,2",
            "4,5",
            "3,0",
            "2,1",
            "6,3",
            "2,4",
            "1,5",
            "0,6",
            "3,3",
            "2,6",
            "5,1",
            "1,2",
            "5,5",
            "2,5",
            "6,5",
            "1,4",
            "0,4",
            "6,4",
            "1,1",
            "6,1",
            "1,0",
            "0,5",
            "1,6",
            "2,0",
        ];

        assert_eq!(22, solve_1(&sample, 12, 6));
    }

    #[test]
    fn day_18_part_01_solution() {
        let input = include_str!("../../inputs/day_18.txt")
            .lines()
            .collect_vec();

        assert_eq!(306, solve_1(&input, 1024, 70));
    }

    #[test]
    fn day_18_part_02_sample() {
        #[rustfmt::skip]
        let sample = vec![
            "5,4",
            "4,2",
            "4,5",
            "3,0",
            "2,1",
            "6,3",
            "2,4",
            "1,5",
            "0,6",
            "3,3",
            "2,6",
            "5,1",
            "1,2",
            "5,5",
            "2,5",
            "6,5",
            "1,4",
            "0,4",
            "6,4",
            "1,1",
            "6,1",
            "1,0",
            "0,5",
            "1,6",
            "2,0",
        ];

        assert_eq!("6,1", solve_2(&sample, 6));
    }

    #[test]
    fn day_18_part_02_solution() {
        let input = include_str!("../../inputs/day_18.txt")
            .lines()
            .collect_vec();

        assert_eq!("38,63", solve_2(&input, 70));
    }
}
