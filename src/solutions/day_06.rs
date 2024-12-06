use itertools::Itertools;
use std::collections::HashSet;

pub fn solve_1(map: &[&str]) -> usize {
    Map::new(map).walk()
}

#[derive(Debug)]
struct Map {
    obstructions: HashSet<Coordinate>,
    width: i32,
    height: i32,
    guard: Coordinate,
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
        let guard = map
            .iter()
            .find(|(_, c)| *c == '^')
            .map(|(coordinate, _)| *coordinate)
            .unwrap();

        Self {
            obstructions,
            width,
            height,
            guard,
        }
    }

    fn walk(&self) -> usize {
        let mut guard = self.guard;
        let mut direction = Direction::Up;
        let mut visited = HashSet::new();

        while (0..self.width).contains(&guard.x) && (0..self.height).contains(&guard.y) {
            visited.insert(guard);
            let next = direction.step(&guard);

            if self.obstructions.contains(&next) {
                direction = direction.turn();
            } else {
                guard = next;
            }
        }

        visited.len()
    }
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
}
