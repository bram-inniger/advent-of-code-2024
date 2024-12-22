use crate::util::graph::Graph;
use itertools::Itertools;
use rustc_hash::FxHashMap;
use std::fmt::Display;
use std::{fmt, iter};

pub fn solve_1(codes: &[&str]) -> u64 {
    solve(codes, 2 + 1)
}

pub fn solve_2(codes: &[&str]) -> u64 {
    solve(codes, 25 + 1)
}

fn solve(codes: &[&str], nr_robots: u32) -> u64 {
    let numeric = Keypad::numeric();
    let directional = Keypad::directional();
    let mut cache = FxHashMap::default();

    codes
        .iter()
        .map(|code| complexity(code, nr_robots, &numeric, &directional, &mut cache))
        .sum()
}

fn complexity(
    code: &str,
    nr_robots: u32,
    numeric: &Keypad,
    directional: &Keypad,
    cache: &mut FxHashMap<(Vec<Button>, u32), u64>,
) -> u64 {
    let buttons = code
        .chars()
        .map(|button| Button::from(button).unwrap())
        .collect_vec();

    let shortest = shortest_len(&buttons, nr_robots, numeric, directional, cache);
    let numeric_part = code[..code.len() - 1].parse::<u64>().unwrap();

    shortest * numeric_part
}

fn shortest_len(
    path: &[Button],
    level: u32,
    keypad: &Keypad,
    directional: &Keypad,
    cache: &mut FxHashMap<(Vec<Button>, u32), u64>,
) -> u64 {
    if cache.contains_key(&(path.to_vec(), level)) {
        return cache[&(path.to_vec(), level)];
    }

    let len = if level == 0 {
        path.len() as u64
    } else {
        to_next_layer(path, keypad)
            .iter()
            .map(|next_path| split_sub_paths(next_path))
            .map(|sub_paths| {
                sub_paths
                    .iter()
                    .map(|sub_path| {
                        shortest_len(sub_path, level - 1, directional, directional, cache)
                    })
                    .sum::<u64>()
            })
            .min()
            .unwrap()
    };

    cache.insert((path.to_vec(), level), len);

    len
}

fn split_sub_paths(path: &[Button]) -> Vec<Vec<Button>> {
    let a_indices = path
        .iter()
        .enumerate()
        .filter(|(_, button)| **button == Button::Activate)
        .map(|(idx, _)| idx)
        .collect_vec();

    (0..a_indices.len())
        .map(|idx| {
            (
                if idx == 0 { 0 } else { a_indices[idx - 1] + 1 },
                a_indices[idx],
            )
        })
        .map(|(from, to)| path[from..=to].to_vec())
        .collect_vec()
}

fn to_next_layer(path: &[Button], keypad: &Keypad) -> Vec<Vec<Button>> {
    let mut paths = vec![vec![]];
    let mut from = Button::Activate;

    for to in path {
        paths = paths
            .iter()
            .flat_map(|first| {
                keypad.shortest_paths[&from][to]
                    .iter()
                    .map(|last| [first.clone(), last.clone()].concat())
            })
            .collect();

        from = *to;
    }

    paths
}

#[derive(Debug)]
struct Keypad {
    shortest_paths: FxHashMap<Button, FxHashMap<Button, Vec<Vec<Button>>>>,
}

impl Keypad {
    pub fn numeric() -> Keypad {
        #[rustfmt::skip]
        let keypad = vec![
            vec!['7', '8', '9'],
            vec!['4', '5', '6'],
            vec!['1', '2', '3'],
            vec![' ', '0', 'A'],
        ];

        Self::new(&keypad)
    }

    pub fn directional() -> Keypad {
        #[rustfmt::skip]
        let keypad = vec![
            vec![' ', '^', 'A'],
            vec!['<', 'v', '>']
        ];

        Self::new(&keypad)
    }
    fn new(keypad: &[Vec<char>]) -> Keypad {
        fn translate(paths: Vec<Vec<Position>>) -> Vec<Vec<Button>> {
            paths
                .iter()
                .map(|path| {
                    (1..path.len())
                        .map(|idx| Button::from_positions(&path[idx - 1], &path[idx]))
                        .chain(iter::once(Button::Activate))
                        .collect_vec()
                })
                .collect()
        }

        let keypad: FxHashMap<Position, Button> = (0..keypad.len())
            .flat_map(|y| {
                (0..keypad[0].len()).map(move |x| (x as i32, y as i32, Button::from(keypad[y][x])))
            })
            .flat_map(|(x, y, btn)| btn.map(|btn| (Position { x, y }, btn)))
            .collect();

        let mut graph = Graph::<Position, u32>::default();
        keypad
            .keys()
            .flat_map(|&node| {
                node.neighbours()
                    .iter()
                    .map(move |&neighbour| (node, neighbour))
                    .collect_vec()
            })
            .filter(|(_, neighbour)| keypad.contains_key(neighbour))
            .for_each(|(node, neighbour)| graph.add_edge(&node, &neighbour, &1));

        let shortest_paths = keypad
            .iter()
            .map(|(from_position, &from_button)| {
                let paths = keypad
                    .iter()
                    .map(|(to_position, &to_button)| {
                        let paths =
                            translate(graph.dijkstra(from_position).shortest_paths(to_position));
                        (to_button, paths)
                    })
                    .collect();
                (from_button, paths)
            })
            .collect();

        Keypad { shortest_paths }
    }
}

impl Display for Keypad {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let print = self
            .shortest_paths
            .iter()
            .sorted_by_key(|(from, _)| *from)
            .flat_map(|(from, tos)| {
                tos.iter()
                    .sorted_by_key(|(to, _)| *to)
                    .map(move |(to, paths)| format!("{:?} to {:?} via: {:?}", from, to, paths))
            })
            .join("\n");

        write!(f, "{}", print)
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum Button {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Activate,
    Up,
    Down,
    Left,
    Right,
}

impl Button {
    fn from(button: char) -> Option<Self> {
        match button {
            '0' => Some(Button::Zero),
            '1' => Some(Button::One),
            '2' => Some(Button::Two),
            '3' => Some(Button::Three),
            '4' => Some(Button::Four),
            '5' => Some(Button::Five),
            '6' => Some(Button::Six),
            '7' => Some(Button::Seven),
            '8' => Some(Button::Eight),
            '9' => Some(Button::Nine),
            'A' => Some(Button::Activate),
            '^' => Some(Button::Up),
            'v' => Some(Button::Down),
            '<' => Some(Button::Left),
            '>' => Some(Button::Right),
            _ => None,
        }
    }

    fn from_positions(from: &Position, to: &Position) -> Self {
        let delta = (from.x - to.x, from.y - to.y);
        match delta {
            (1, 0) => Button::Left,
            (-1, 0) => Button::Right,
            (0, 1) => Button::Up,
            (0, -1) => Button::Down,
            _ => panic!("Invalid move between positions {:?} and {:?}", from, to),
        }
    }
}

impl Display for Button {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let print = match self {
            Button::Zero => "0",
            Button::One => "1",
            Button::Two => "2",
            Button::Three => "3",
            Button::Four => "4",
            Button::Five => "5",
            Button::Six => "6",
            Button::Seven => "7",
            Button::Eight => "8",
            Button::Nine => "9",
            Button::Activate => "A",
            Button::Up => "^",
            Button::Down => "v",
            Button::Left => "<",
            Button::Right => ">",
        };

        write!(f, "{}", print)
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn neighbours(&self) -> Vec<Self> {
        [(1, 0), (-1, 0), (0, 1), (0, -1)]
            .map(|(dx, dy)| Position {
                x: self.x + dx,
                y: self.y + dy,
            })
            .to_vec()
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn day_21_part_01_sample() {
        #[rustfmt::skip]
        let sample = vec![
            "029A",
            "980A",
            "179A",
            "456A",
            "379A",
        ];

        assert_eq!(126_384, solve_1(&sample));
    }

    #[test]
    fn day_21_part_01_solution() {
        let input = include_str!("../../inputs/day_21.txt")
            .lines()
            .collect_vec();

        assert_eq!(211_930, solve_1(&input));
    }

    #[test]
    fn day_21_part_02_sample() {
        // No sample input provided
    }

    #[test]
    fn day_21_part_02_solution() {
        let input = include_str!("../../inputs/day_21.txt")
            .lines()
            .collect_vec();

        assert_eq!(263_492_840_501_566, solve_2(&input));
    }
}
