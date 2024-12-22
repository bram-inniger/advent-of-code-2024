use crate::util::graph::{Dijkstra, Graph};
use itertools::Itertools;
use rustc_hash::FxHashMap;

pub fn solve_1(maze: &[&str]) -> u32 {
    solve(maze).shortest_distance
}

pub fn solve_2(maze: &[&str]) -> usize {
    solve(maze).shortest_tiles
}

fn solve(maze: &[&str]) -> SolvedMaze {
    let maze = Maze::new(maze);
    let dijkstra = maze.dijkstra(&maze.start);

    let shortest_distance = maze
        .ends
        .iter()
        .map(|end| dijkstra.distance(end))
        .min()
        .unwrap();
    let shortest_tiles = maze
        .ends
        .iter()
        .filter(|end| dijkstra.distance(end) == shortest_distance)
        .map(|end| dijkstra.shortest_paths(end))
        .flat_map(|paths| paths.into_iter())
        .flat_map(|path| path.into_iter())
        .map(|node| node.coordinate)
        .unique()
        .count();

    SolvedMaze {
        shortest_distance,
        shortest_tiles,
    }
}

#[derive(Debug)]
struct Maze {
    pub start: Node,
    pub ends: Vec<Node>,
    graph: Graph<Node, u32>,
}

impl Maze {
    pub fn new(maze: &[&str]) -> Self {
        let tiles: FxHashMap<Coordinate, char> = maze
            .iter()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(move |(x, c)| (Coordinate { x, y }, c))
            })
            .filter(|(_, c)| ['.', 'S', 'E'].contains(c))
            .collect();

        let start = tiles
            .iter()
            .find(|(_, c)| **c == 'S')
            .map(|(coord, _)| *coord)
            .unwrap();
        let start = Node {
            coordinate: start,
            orientation: Orientation::East,
        };

        let end = tiles
            .iter()
            .find(|(_, c)| **c == 'E')
            .map(|(coord, _)| *coord)
            .unwrap();
        let ends = [
            Orientation::North,
            Orientation::East,
            Orientation::South,
            Orientation::West,
        ]
        .map(|orientation| Node {
            coordinate: end,
            orientation,
        })
        .to_vec();

        let mut graph = Graph::default();
        tiles
            .keys()
            .flat_map(|&coordinate| {
                [
                    Orientation::North,
                    Orientation::East,
                    Orientation::South,
                    Orientation::West,
                ]
                .map(move |orientation| Node {
                    coordinate,
                    orientation,
                })
            })
            .flat_map(|node| {
                node.neighbours()
                    .into_iter()
                    .map(move |(neighbour, weight)| (node, neighbour, weight))
            })
            .for_each(|(node, neighbour, weight)| graph.add_edge(&node, &neighbour, &weight));

        Self { start, ends, graph }
    }

    pub fn dijkstra(&self, start: &Node) -> Dijkstra<Node, u32> {
        self.graph.dijkstra(start)
    }
}

#[derive(Debug)]
struct SolvedMaze {
    shortest_distance: u32,
    shortest_tiles: usize,
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    fn next(&self, orientation: &Orientation) -> Self {
        match orientation {
            Orientation::North => Self {
                x: self.x,
                y: self.y - 1,
            },
            Orientation::East => Self {
                x: self.x + 1,
                y: self.y,
            },
            Orientation::South => Self {
                x: self.x,
                y: self.y + 1,
            },
            Orientation::West => Self {
                x: self.x - 1,
                y: self.y,
            },
        }
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum Orientation {
    North,
    East,
    South,
    West,
}

impl Orientation {
    fn clockwise(&self) -> Self {
        match self {
            Orientation::North => Orientation::East,
            Orientation::East => Orientation::South,
            Orientation::South => Orientation::West,
            Orientation::West => Orientation::North,
        }
    }

    fn counter_clockwise(&self) -> Self {
        match self {
            Orientation::North => Orientation::West,
            Orientation::East => Orientation::North,
            Orientation::South => Orientation::East,
            Orientation::West => Orientation::South,
        }
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Node {
    coordinate: Coordinate,
    orientation: Orientation,
}

impl Node {
    fn neighbours(&self) -> Vec<(Node, u32)> {
        vec![
            (
                Node {
                    coordinate: self.coordinate.next(&self.orientation),
                    orientation: self.orientation,
                },
                1,
            ),
            (
                Node {
                    coordinate: self.coordinate,
                    orientation: self.orientation.clockwise(),
                },
                1_000,
            ),
            (
                Node {
                    coordinate: self.coordinate,
                    orientation: self.orientation.counter_clockwise(),
                },
                1_000,
            ),
        ]
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn day_16_part_01_sample() {
        let sample_1 = vec![
            "###############",
            "#.......#....E#",
            "#.#.###.#.###.#",
            "#.....#.#...#.#",
            "#.###.#####.#.#",
            "#.#.#.......#.#",
            "#.#.#####.###.#",
            "#...........#.#",
            "###.#.#####.#.#",
            "#...#.....#.#.#",
            "#.#.#.###.#.#.#",
            "#.....#...#.#.#",
            "#.###.#.#.#.#.#",
            "#S..#.....#...#",
            "###############",
        ];
        let sample_2 = vec![
            "#################",
            "#...#...#...#..E#",
            "#.#.#.#.#.#.#.#.#",
            "#.#.#.#...#...#.#",
            "#.#.#.#.###.#.#.#",
            "#...#.#.#.....#.#",
            "#.#.#.#.#.#####.#",
            "#.#...#.#.#.....#",
            "#.#.#####.#.###.#",
            "#.#.#.......#...#",
            "#.#.###.#####.###",
            "#.#.#...#.....#.#",
            "#.#.#.#####.###.#",
            "#.#.#.........#.#",
            "#.#.#.#########.#",
            "#S#.............#",
            "#################",
        ];

        assert_eq!(7_036, solve_1(&sample_1));
        assert_eq!(11_048, solve_1(&sample_2));
    }

    #[test]
    fn day_16_part_01_solution() {
        let input = include_str!("../../inputs/day_16.txt")
            .lines()
            .collect_vec();

        assert_eq!(94_444, solve_1(&input));
    }

    #[test]
    fn day_16_part_02_sample() {
        let sample_1 = vec![
            "###############",
            "#.......#....E#",
            "#.#.###.#.###.#",
            "#.....#.#...#.#",
            "#.###.#####.#.#",
            "#.#.#.......#.#",
            "#.#.#####.###.#",
            "#...........#.#",
            "###.#.#####.#.#",
            "#...#.....#.#.#",
            "#.#.#.###.#.#.#",
            "#.....#...#.#.#",
            "#.###.#.#.#.#.#",
            "#S..#.....#...#",
            "###############",
        ];
        let sample_2 = vec![
            "#################",
            "#...#...#...#..E#",
            "#.#.#.#.#.#.#.#.#",
            "#.#.#.#...#...#.#",
            "#.#.#.#.###.#.#.#",
            "#...#.#.#.....#.#",
            "#.#.#.#.#.#####.#",
            "#.#...#.#.#.....#",
            "#.#.#####.#.###.#",
            "#.#.#.......#...#",
            "#.#.###.#####.###",
            "#.#.#...#.....#.#",
            "#.#.#.#####.###.#",
            "#.#.#.........#.#",
            "#.#.#.#########.#",
            "#S#.............#",
            "#################",
        ];

        assert_eq!(45, solve_2(&sample_1));
        assert_eq!(64, solve_2(&sample_2));
    }

    #[test]
    fn day_16_part_02_solution() {
        let input = include_str!("../../inputs/day_16.txt")
            .lines()
            .collect_vec();

        assert_eq!(502, solve_2(&input));
    }
}
