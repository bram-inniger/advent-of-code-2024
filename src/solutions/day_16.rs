use rustc_hash::{FxHashMap, FxHashSet};

pub fn solve_1(maze: &[&str]) -> u32 {
    let maze = Maze::new(maze);
    let start = Node {
        coordinate: maze.start,
        orientation: Orientation::East,
    };
    let end = [
        Orientation::North,
        Orientation::East,
        Orientation::South,
        Orientation::West,
    ]
    .map(|orientation| Node {
        coordinate: maze.end,
        orientation,
    });

    let distances = dijkstra(&maze, &start);
    end.iter().map(|e| distances[e]).min().unwrap()
}

fn dijkstra(maze: &Maze, start: &Node) -> FxHashMap<Node, u32> {
    let mut unvisited: FxHashSet<Node> = maze
        .tiles
        .iter()
        .flat_map(|&coordinate| {
            [
                Orientation::North,
                Orientation::East,
                Orientation::South,
                Orientation::West,
            ]
            .map(|orientation| Node {
                coordinate,
                orientation,
            })
        })
        .collect();

    // todo, maybe, make this a BTreeMap instead, popping the smallest element?
    let mut distances: FxHashMap<Node, u32> = unvisited
        .iter()
        .map(|node| {
            let distance = if node == start { 0 } else { u32::MAX };
            (*node, distance)
        })
        .collect();

    loop {
        let Some((&current, &current_distance)) = distances
            .iter()
            .filter(|&(_, &distance)| distance != u32::MAX)
            .filter(|(node, _)| unvisited.contains(node))
            .min_by_key(|&(_, distance)| distance)
        else {
            return distances;
        };

        for (neighbour, neighbour_distance) in current
            .neighbours()
            .into_iter()
            .filter(|(node, _)| unvisited.contains(node))
        {
            distances.insert(
                neighbour,
                u32::min(distances[&neighbour], current_distance + neighbour_distance),
            );
        }

        unvisited.remove(&current);
    }
}

#[derive(Debug)]
struct Maze {
    tiles: FxHashSet<Coordinate>,
    start: Coordinate,
    end: Coordinate,
}

impl Maze {
    fn new(maze: &[&str]) -> Self {
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
        let end = tiles
            .iter()
            .find(|(_, c)| **c == 'E')
            .map(|(coord, _)| *coord)
            .unwrap();
        let tiles = tiles.keys().copied().collect();

        Self { tiles, start, end }
    }
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
                    coordinate: self.coordinate.next(&self.orientation.clockwise()),
                    orientation: self.orientation.clockwise(),
                },
                1_001,
            ),
            (
                Node {
                    coordinate: self.coordinate.next(&self.orientation.counter_clockwise()),
                    orientation: self.orientation.counter_clockwise(),
                },
                1_001,
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
}
