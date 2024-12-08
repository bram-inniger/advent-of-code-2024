use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub fn solve_1(map: &[&str]) -> usize {
    Map::new(map).antinodes(false).len()
}

pub fn solve_2(map: &[&str]) -> usize {
    Map::new(map).antinodes(true).len()
}

#[derive(Debug)]
struct Map {
    antennas: HashMap<char, Vec<Location>>,
    width: i32,
    height: i32,
}

impl Map {
    fn new(map: &[&str]) -> Self {
        let antennas = map
            .iter()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().map(move |(x, c)| {
                    let x = x as i32;
                    let y = y as i32;
                    (c, Location { x, y })
                })
            })
            .filter(|(c, _)| *c != '.')
            .sorted_by_key(|(c, _)| *c)
            .chunk_by(|(c, _)| *c)
            .into_iter()
            .map(|(c, locations)| (c, locations.into_iter().map(|(_, l)| l).collect()))
            .collect();
        let width = map[0].len() as i32;
        let height = map.len() as i32;

        Self {
            antennas,
            width,
            height,
        }
    }

    fn antinodes(&self, harmonics: bool) -> HashSet<Location> {
        self.antennas
            .values()
            .flat_map(|antenna| {
                (0..antenna.len())
                    .flat_map(move |i| {
                        ((i + 1)..antenna.len()).map(move |j| (antenna[i], antenna[j]))
                    })
                    .flat_map(|(l1, l2)| self.antinodes_per_pair(l1, l2, harmonics))
            })
            .collect()
    }

    fn antinodes_per_pair(&self, l1: Location, l2: Location, harmonics: bool) -> Vec<Location> {
        [
            (l1, l1.x - l2.x, l1.y - l2.y),
            (l2, l2.x - l1.x, l2.y - l1.y),
        ]
        .iter()
        .flat_map(|(antinode, dx, dy)| {
            (if harmonics { 0..i32::MAX } else { 1..2 })
                .map(move |i| Location {
                    x: antinode.x + i * dx,
                    y: antinode.y + i * dy,
                })
                .take_while(|l| l.is_in_bounds(self))
        })
        .collect()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
struct Location {
    x: i32,
    y: i32,
}

impl Location {
    fn is_in_bounds(&self, map: &Map) -> bool {
        self.x >= 0 && self.x < map.width && self.y >= 0 && self.y < map.height
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn day_08_part_01_sample() {
        let sample = vec![
            "............",
            "........0...",
            ".....0......",
            ".......0....",
            "....0.......",
            "......A.....",
            "............",
            "............",
            "........A...",
            ".........A..",
            "............",
            "............",
        ];

        assert_eq!(14, solve_1(&sample));
    }

    #[test]
    fn day_08_part_01_solution() {
        let input = include_str!("../../inputs/day_08.txt")
            .lines()
            .collect_vec();

        assert_eq!(269, solve_1(&input));
    }

    #[test]
    fn day_08_part_02_sample() {
        let sample = vec![
            "............",
            "........0...",
            ".....0......",
            ".......0....",
            "....0.......",
            "......A.....",
            "............",
            "............",
            "........A...",
            ".........A..",
            "............",
            "............",
        ];

        assert_eq!(34, solve_2(&sample));
    }

    #[test]
    fn day_08_part_02_solution() {
        let input = include_str!("../../inputs/day_08.txt")
            .lines()
            .collect_vec();

        assert_eq!(949, solve_2(&input));
    }
}
