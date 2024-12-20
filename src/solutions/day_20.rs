use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::VecDeque;

pub fn solve_1(track: &[&str], min_save: i32) -> usize {
    solve(track, min_save, 2)
}

pub fn solve_2(track: &[&str], min_save: i32) -> usize {
    solve(track, min_save, 20)
}

fn solve(track: &[&str], min_save: i32, max_cheat_dist: i32) -> usize {
    let track = Track::new(track);
    let times = track.run();

    track
        .road
        .iter()
        .flat_map(|cheat| {
            (-max_cheat_dist..=max_cheat_dist).flat_map(move |dx| {
                (-max_cheat_dist..=max_cheat_dist).map(move |dy| (cheat, dx, dy))
            })
        })
        .map(|(cheat, dx, dy)| {
            (
                cheat,
                Coordinate {
                    x: cheat.x + dx,
                    y: cheat.y + dy,
                },
                dx.abs() + dy.abs(),
            )
        })
        .filter(|(_, to, cheat_dist)| track.road.contains(to) && *cheat_dist <= max_cheat_dist)
        .map(|(from, to, cheat_dist)| times[from] - times[&to] - cheat_dist)
        .filter(|&saved| saved >= min_save)
        .count()
}

#[derive(Debug)]
struct Track {
    road: FxHashSet<Coordinate>,
    _start: Coordinate,
    end: Coordinate,
    width: i32,
    height: i32,
}

impl Track {
    fn new(track: &[&str]) -> Self {
        let tiles: FxHashMap<_, _> = track
            .iter()
            .enumerate()
            .flat_map(|(y, line)| line.chars().enumerate().map(move |(x, c)| (y, x, c)))
            .filter(|&(_, _, c)| c != '#')
            .sorted_by_key(|&(_, _, c)| c)
            .chunk_by(|&(_, _, c)| c)
            .into_iter()
            .map(|(c, group)| {
                (
                    c,
                    group
                        .into_iter()
                        .map(|(y, x, _)| Coordinate {
                            x: x as i32,
                            y: y as i32,
                        })
                        .collect_vec(),
                )
            })
            .collect();

        let road = tiles.values().flatten().copied().collect();
        let _start = tiles[&'S'][0];
        let end = tiles[&'E'][0];
        let height = track.len() as i32;
        let width = track[0].len() as i32;

        Self {
            road,
            _start,
            end,
            width,
            height,
        }
    }

    fn run(&self) -> FxHashMap<Coordinate, i32> {
        let mut to_visit = VecDeque::<(Coordinate, i32)>::new();
        let mut visited = FxHashMap::<Coordinate, i32>::default();

        to_visit.push_back((self.end, 0));

        while let Some((node, distance)) = to_visit.pop_front() {
            if visited.contains_key(&node) {
                continue;
            }

            visited.insert(node, distance);

            [(1, 0), (-1, 0), (0, 1), (0, -1)]
                .map(|(dx, dy)| Coordinate {
                    x: node.x + dx,
                    y: node.y + dy,
                })
                .into_iter()
                .filter(|neighbour| {
                    self.road.contains(neighbour)
                        && (0..=self.width).contains(&neighbour.x)
                        && (0..=self.height).contains(&neighbour.y)
                })
                .for_each(|neighbour| {
                    to_visit.push_back((neighbour, distance + 1));
                });
        }

        visited
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Coordinate {
    x: i32,
    y: i32,
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn day_20_part_01_sample() {
        let sample = vec![
            "###############",
            "#...#...#.....#",
            "#.#.#.#.#.###.#",
            "#S#...#.#.#...#",
            "#######.#.#.###",
            "#######.#.#...#",
            "#######.#.###.#",
            "###..E#...#...#",
            "###.#######.###",
            "#...###...#...#",
            "#.#####.#.###.#",
            "#.#...#.#.#...#",
            "#.#.#.#.#.#.###",
            "#...#...#...###",
            "###############",
        ];

        assert_eq!(
            14 + 14 + 2 + 4 + 2 + 3 + 1 + 1 + 1 + 1 + 1,
            solve_1(&sample, 1)
        );
    }

    #[test]
    fn day_20_part_01_solution() {
        let input = include_str!("../../inputs/day_20.txt")
            .lines()
            .collect_vec();

        assert_eq!(1_289, solve_1(&input, 100));
    }

    #[test]
    fn day_20_part_02_sample() {
        let sample = vec![
            "###############",
            "#...#...#.....#",
            "#.#.#.#.#.###.#",
            "#S#...#.#.#...#",
            "#######.#.#.###",
            "#######.#.#...#",
            "#######.#.###.#",
            "###..E#...#...#",
            "###.#######.###",
            "#...###...#...#",
            "#.#####.#.###.#",
            "#.#...#.#.#...#",
            "#.#.#.#.#.#.###",
            "#...#...#...###",
            "###############",
        ];

        assert_eq!(
            32 + 31 + 29 + 39 + 25 + 23 + 20 + 19 + 12 + 14 + 12 + 22 + 4 + 3,
            solve_2(&sample, 50)
        );
    }

    #[test]
    fn day_20_part_02_solution() {
        let input = include_str!("../../inputs/day_20.txt")
            .lines()
            .collect_vec();

        assert_eq!(982_425, solve_2(&input, 100));
    }
}
