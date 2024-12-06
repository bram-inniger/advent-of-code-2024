use itertools::Itertools;
use std::collections::HashSet;
use std::ops::Not;

pub fn solve_1(map: &[&str]) -> usize {
    let (guards, _) = Map::new(map).walk();
    guards.iter().map(|guard| guard.coordinate).unique().count()
}

pub fn solve_2(map: &[&str]) -> usize {
    let map = Map::new(map);

    (0..map.height)
        .flat_map(|y| (0..map.width).map(move |x| Coordinate { x, y }))
        .filter(|c| map.obstructions.contains(c).not() && &map.guard.coordinate != c)
        .map(|c| {
            let mut map = map.clone();
            map.obstructions.insert(c);
            map
        })
        .map(|map| map.walk().1)
        .filter(|&s| s == Status::Looping)
        .count()
}

#[derive(Debug, Clone)]
struct Map {
    obstructions: HashSet<Coordinate>,
    width: i32,
    height: i32,
    guard: Guard,
}

impl Map {
    fn new(map: &[&str]) -> Map {
        let width = map[0].len() as i32;
        let height = map.len() as i32;
        let map = map
            .iter()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().map(move |(x, c)| {
                    let x = x as i32;
                    let y = y as i32;
                    (Coordinate { x, y }, c)
                })
            })
            .collect_vec();
        let obstructions = map
            .iter()
            .filter(|(_, c)| *c == '#')
            .map(|(coordinate, _)| *coordinate)
            .collect();
        let guard = Guard {
            coordinate: map
                .iter()
                .find(|(_, c)| *c == '^')
                .map(|(coordinate, _)| *coordinate)
                .unwrap(),
            direction: Direction::Up,
        };

        Self {
            obstructions,
            width,
            height,
            guard,
        }
    }

    fn walk(&self) -> (HashSet<Guard>, Status) {
        let mut guard = self.guard;
        let mut visited = HashSet::new();

        loop {
            if visited.contains(&guard) {
                return (visited, Status::Looping);
            }
            if (0..self.width).contains(&guard.coordinate.x).not()
                || (0..self.height).contains(&guard.coordinate.y).not()
            {
                return (visited, Status::Finished);
            }

            visited.insert(guard);
            let next_coordinate = guard.direction.step(&guard.coordinate);

            if self.obstructions.contains(&next_coordinate) {
                guard.direction = guard.direction.turn();
            } else {
                guard.coordinate = next_coordinate;
            }
        }
    }
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Guard {
    coordinate: Coordinate,
    direction: Direction,
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Coordinate {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn(&self) -> Direction {
        match self {
            Self::Up => Direction::Right,
            Self::Right => Direction::Down,
            Self::Down => Direction::Left,
            Self::Left => Direction::Up,
        }
    }

    fn step(&self, coordinate: &Coordinate) -> Coordinate {
        match self {
            Direction::Up => Coordinate {
                x: coordinate.x,
                y: coordinate.y - 1,
            },
            Direction::Right => Coordinate {
                x: coordinate.x + 1,
                y: coordinate.y,
            },
            Direction::Down => Coordinate {
                x: coordinate.x,
                y: coordinate.y + 1,
            },
            Direction::Left => Coordinate {
                x: coordinate.x - 1,
                y: coordinate.y,
            },
        }
    }
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum Status {
    Finished,
    Looping,
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn day_06_part_01_sample() {
        let sample = vec![
            "....#.....",
            ".........#",
            "..........",
            "..#.......",
            ".......#..",
            "..........",
            ".#..^.....",
            "........#.",
            "#.........",
            "......#...",
        ];

        assert_eq!(41, solve_1(&sample));
    }

    #[test]
    fn day_06_part_01_solution() {
        let input = include_str!("../../inputs/day_06.txt")
            .lines()
            .collect_vec();

        assert_eq!(4_515, solve_1(&input));
    }

    #[test]
    fn day_06_part_02_sample() {
        let sample = vec![
            "....#.....",
            ".........#",
            "..........",
            "..#.......",
            ".......#..",
            "..........",
            ".#..^.....",
            "........#.",
            "#.........",
            "......#...",
        ];

        assert_eq!(6, solve_2(&sample));
    }

    #[test]
    fn day_06_part_02_solution() {
        let input = include_str!("../../inputs/day_06.txt")
            .lines()
            .collect_vec();

        assert_eq!(1_309, solve_2(&input));
    }
}
