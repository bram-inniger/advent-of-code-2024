use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::VecDeque;
use std::ops::Not;

pub fn solve_1(track: &[&str], min_save: u32) -> usize {
    let track = Track::new(track);
    let times = track.run();

    (1..track.width)
        .flat_map(|x| (1..track.height).map(move |y| Coordinate { x, y }))
        .flat_map(|cheat| {
            [(1, 0), (0, 1), (-1, 0), (0, -1)]
                .into_iter()
                .map(move |(dx, dy)| (cheat, Coordinate { x: dx, y: dy }))
        })
        .map(|(cheat, delta)| {
            (
                Coordinate {
                    x: cheat.x - delta.x,
                    y: cheat.y - delta.y,
                },
                Coordinate {
                    x: cheat.x,
                    y: cheat.y,
                },
                Coordinate {
                    x: cheat.x + delta.x,
                    y: cheat.y + delta.y,
                },
            )
        })
        .filter(|(zero, one, two)| {
            track.road.contains(zero) && track.road.contains(one).not() && track.road.contains(two)
        })
        .map(|(zero, _, two)| (times[&zero], times[&two]))
        .filter(|(from, to)| from > to)
        .map(|(from, to)| from - to - 2)
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

    fn run(&self) -> FxHashMap<Coordinate, u32> {
        let mut to_visit = VecDeque::<(Coordinate, u32)>::new();
        let mut visited = FxHashMap::<Coordinate, u32>::default();

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
            solve_1(&sample, 0)
        );
    }

    #[test]
    fn day_20_part_01_solution() {
        let input = include_str!("../../inputs/day_20.txt")
            .lines()
            .collect_vec();

        assert_eq!(1_289, solve_1(&input, 100));
    }
}
