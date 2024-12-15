use itertools::Itertools;
use std::collections::VecDeque;
use std::fmt::{Display, Formatter};
use std::ops::{Index, IndexMut};

pub fn solve_1(description: &str) -> usize {
    let (warehouse, moves) = description.split_once("\n\n").unwrap();

    let mut robot = Robot::new(moves, warehouse, false);
    let mut warehouse = Warehouse::new(warehouse, false);

    while let Some(direction) = robot.moves.pop_front() {
        let next_coordinate = robot.coordinate.next(&direction);
        let next_type = warehouse[next_coordinate];

        match next_type {
            Type::Wall => continue,
            Type::Empty => robot.coordinate = next_coordinate,
            Type::BoxLeft => {
                let mut box_end = next_coordinate;

                while warehouse[box_end] == Type::BoxLeft {
                    box_end = box_end.next(&direction);
                }

                match warehouse[box_end] {
                    Type::Wall => continue,
                    Type::Empty => {
                        warehouse[box_end] = Type::BoxLeft;
                        warehouse[next_coordinate] = Type::Empty;
                        robot.coordinate = next_coordinate;
                    }
                    Type::BoxLeft => {
                        panic!("End of the line of boxes is still a box: {:?}", box_end)
                    }
                    Type::BoxRight => unreachable!(),
                }
            }
            Type::BoxRight => unreachable!(),
        }
    }

    warehouse.gps_sum()
}

pub fn solve_2(description: &str) -> usize {
    let (warehouse, moves) = description.split_once("\n\n").unwrap();

    let mut robot = Robot::new(moves, warehouse, true);
    let mut warehouse = Warehouse::new(warehouse, true);

    while let Some(direction) = robot.moves.pop_front() {
        let next_coordinate = robot.coordinate.next(&direction);
        let next_type = warehouse[next_coordinate];

        match next_type {
            Type::Wall => continue,
            Type::Empty => robot.coordinate = next_coordinate,
            Type::BoxLeft | Type::BoxRight
                if [Direction::Up, Direction::Down].contains(&direction) =>
            {
                let adjacent = match next_type {
                    Type::BoxLeft => next_coordinate.next(&Direction::Right),
                    Type::BoxRight => next_coordinate.next(&Direction::Left),
                    _ => unreachable!(),
                };
                let coordinates = match can_move(&next_coordinate, &direction, &warehouse) {
                    Some(coordinates) => coordinates,
                    None => continue,
                };
                let adjacent_coordinates = match can_move(&adjacent, &direction, &warehouse) {
                    Some(coordinates) => coordinates,
                    None => continue,
                };

                do_move(
                    &[coordinates, adjacent_coordinates].concat(),
                    &direction,
                    &mut warehouse,
                );
                robot.coordinate = next_coordinate;
            }
            Type::BoxLeft | Type::BoxRight => {
                let coordinates = match can_move(&next_coordinate, &direction, &warehouse) {
                    Some(coordinates) => coordinates,
                    None => continue,
                };
                do_move(&coordinates, &direction, &mut warehouse);
                robot.coordinate = next_coordinate;
            }
        }
    }

    warehouse.gps_sum()
}

fn can_move(
    coordinate: &Coordinate,
    direction: &Direction,
    warehouse: &Warehouse,
) -> Option<Vec<Coordinate>> {
    let next_coordinate = coordinate.next(direction);
    let next_type = warehouse[next_coordinate];

    match next_type {
        Type::Wall => None,
        Type::Empty => Some(vec![*coordinate]),
        Type::BoxLeft | Type::BoxRight => match direction {
            Direction::Up | Direction::Down => {
                let adjacent_direction = match next_type {
                    Type::BoxLeft => Direction::Right,
                    Type::BoxRight => Direction::Left,
                    _ => unreachable!(),
                };
                let adjacent_coordinate = next_coordinate.next(&adjacent_direction);
                let next = match can_move(&next_coordinate, direction, warehouse) {
                    Some(to_move) => to_move,
                    None => {
                        return None;
                    }
                };
                let adjacent = match can_move(&adjacent_coordinate, direction, warehouse) {
                    Some(to_move) => to_move,
                    None => {
                        return None;
                    }
                };

                Some([next, adjacent, vec![*coordinate, adjacent_coordinate]].concat())
            }
            Direction::Right | Direction::Left => {
                let adjacent = match can_move(&next_coordinate, direction, warehouse) {
                    Some(to_move) => to_move,
                    None => {
                        return None;
                    }
                };

                Some([adjacent, vec![*coordinate, next_coordinate]].concat())
            }
        },
    }
}

fn do_move(coordinates: &[Coordinate], direction: &Direction, warehouse: &mut Warehouse) {
    let boxes = coordinates
        .iter()
        .map(|&coordinate| (coordinate, warehouse[coordinate]))
        .collect_vec();

    boxes.iter().for_each(|&(coordinate, _)| {
        warehouse[coordinate] = Type::Empty;
    });
    boxes
        .into_iter()
        .map(|(coordinate, t)| (coordinate.next(direction), t))
        .for_each(|(coordinate, t)| warehouse[coordinate] = t);
}

#[derive(Debug)]
struct Warehouse {
    map: Vec<Vec<Type>>,
}

impl Warehouse {
    fn new(warehouse: &str, wide: bool) -> Self {
        let map = warehouse
            .lines()
            .map(|line| {
                line.chars()
                    .flat_map(|c| {
                        if wide {
                            match c {
                                '#' => vec![Type::Wall, Type::Wall],
                                '.' | '@' => vec![Type::Empty, Type::Empty],
                                'O' => vec![Type::BoxLeft, Type::BoxRight],
                                _ => panic!("Invalid type in warehouse: {}", c),
                            }
                        } else {
                            match c {
                                '#' => vec![Type::Wall],
                                '.' | '@' => vec![Type::Empty],
                                'O' => vec![Type::BoxLeft],
                                _ => panic!("Invalid type in warehouse: {}", c),
                            }
                        }
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
            .filter(|&(_, &t)| t == Type::BoxLeft)
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
                        Type::BoxLeft => '[',
                        Type::BoxRight => ']',
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
    fn new(moves: &str, warehouse: &str, wide: bool) -> Self {
        let coordinate = warehouse
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().map(move |(x, c)| {
                    let x = if wide { x * 2 } else { x };
                    (Coordinate { x, y }, c)
                })
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
    BoxLeft,
    BoxRight,
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Direction::Up => "^",
                Direction::Right => ">",
                Direction::Down => "v",
                Direction::Left => "<",
            }
        )
    }
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

    #[test]
    fn day_15_part_02_sample() {
        let sample = "\
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

        assert_eq!(9_021, solve_2(sample));
    }

    #[test]
    fn day_15_part_02_solution() {
        let input = include_str!("../../inputs/day_15.txt").trim();

        assert_eq!(1_575_877, solve_2(input));
    }
}
