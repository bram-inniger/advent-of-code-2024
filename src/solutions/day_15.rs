use itertools::Itertools;
use std::collections::VecDeque;
use std::ops::{Index, IndexMut};

pub fn solve_1(description: &str) -> usize {
    let (warehouse, moves) = description.split_once("\n\n").unwrap();

    let mut robot = Robot::new(moves, warehouse);
    let mut warehouse = Warehouse::new(warehouse);

    while let Some(direction) = robot.moves.pop_front() {
        let next_coordinate = robot.coordinate.next(&direction);
        let next_type = warehouse[next_coordinate];

        match next_type {
            Type::Wall => continue,
            Type::Empty => robot.coordinate = next_coordinate,
            Type::Box => {
                let mut box_end = next_coordinate;

                while warehouse[box_end] == Type::Box {
                    box_end = box_end.next(&direction);
                }

                match warehouse[box_end] {
                    Type::Wall => continue,
                    Type::Empty => {
                        warehouse[box_end] = Type::Box;
                        warehouse[next_coordinate] = Type::Empty;
                        robot.coordinate = next_coordinate;
                    }
                    Type::Box => panic!("End of the line of boxes is still a box: {:?}", box_end),
                }
            }
        }
    }

    warehouse.gps_sum()
}

#[derive(Debug)]
struct Warehouse {
    map: Vec<Vec<Type>>,
}

impl Warehouse {
    fn new(warehouse: &str) -> Self {
        let map = warehouse
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '#' => Type::Wall,
                        '.' | '@' => Type::Empty,
                        'O' => Type::Box,
                        _ => panic!("Invalid type in warehouse: {}", c),
                    })
                    .collect()
            })
            .collect();

        Self { map }
    }

    fn gps_sum(&self) -> usize {
        self.map
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .map(move |(x, t)| (Coordinate { x, y }, t))
            })
            .filter(|(_, t)| **t == Type::Box)
            .map(|(Coordinate { x, y }, _)| 100 * y + x)
            .sum()
    }

    #[allow(dead_code)]
    fn pretty_print(&self, robot: &Robot) -> String {
        (0..self.map.len())
            .map(|y| {
                self.map[y]
                    .iter()
                    .enumerate()
                    .map(|(x, t)| match t {
                        Type::Wall => '#',
                        Type::Empty if Coordinate { x, y } == robot.coordinate => '@',
                        Type::Empty => '.',
                        Type::Box => 'O',
                    })
                    .collect::<String>()
            })
            .join("\n")
    }
}

impl Index<Coordinate> for Warehouse {
    type Output = Type;

    fn index(&self, index: Coordinate) -> &Self::Output {
        &self.map[index.y][index.x]
    }
}

impl IndexMut<Coordinate> for Warehouse {
    fn index_mut(&mut self, index: Coordinate) -> &mut Self::Output {
        &mut self.map[index.y][index.x]
    }
}

#[derive(Debug)]
struct Robot {
    coordinate: Coordinate,
    moves: VecDeque<Direction>,
}

impl Robot {
    fn new(moves: &str, warehouse: &str) -> Self {
        let coordinate = warehouse
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(move |(x, c)| (Coordinate { x, y }, c))
            })
            .find(|(_, c)| *c == '@')
            .map(|(coordinate, _)| coordinate)
            .unwrap();
        let moves = moves
            .replace('\n', "")
            .chars()
            .map(|c| match c {
                '^' => Direction::Up,
                '>' => Direction::Right,
                'v' => Direction::Down,
                '<' => Direction::Left,
                _ => panic!("Invalid direction in moves: {}", c),
            })
            .collect();

        Self { coordinate, moves }
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum Type {
    Wall,
    Empty,
    Box,
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    fn next(&self, direction: &Direction) -> Self {
        match direction {
            Direction::Up => Self {
                x: self.x,
                y: self.y - 1,
            },
            Direction::Right => Self {
                x: self.x + 1,
                y: self.y,
            },
            Direction::Down => Self {
                x: self.x,
                y: self.y + 1,
            },
            Direction::Left => Self {
                x: self.x - 1,
                y: self.y,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_15_part_01_sample() {
        let sample_1 = "\
                ########\n\
                #..O.O.#\n\
                ##@.O..#\n\
                #...O..#\n\
                #.#.O..#\n\
                #...O..#\n\
                #......#\n\
                ########\n\
                \n\
                <^^>>>vv<v>>v<<\
            ";
        let sample_2 = "\
                ##########\n\
                #..O..O.O#\n\
                #......O.#\n\
                #.OO..O.O#\n\
                #..O@..O.#\n\
                #O#..O...#\n\
                #O..O..O.#\n\
                #.OO.O.OO#\n\
                #....O...#\n\
                ##########\n\
                \n\
                <vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^\n\
                vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v\n\
                ><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<\n\
                <<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^\n\
                ^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><\n\
                ^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^\n\
                >^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^\n\
                <><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>\n\
                ^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>\n\
                v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^\
            ";

        assert_eq!(2_028, solve_1(sample_1));
        assert_eq!(10_092, solve_1(sample_2));
    }

    #[test]
    fn day_15_part_01_solution() {
        let input = include_str!("../../inputs/day_15.txt").trim();

        assert_eq!(1_568_399, solve_1(input));
    }
}
