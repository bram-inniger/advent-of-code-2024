use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};
use std::fmt;
use std::fmt::Display;
use std::ops::Not;

pub fn solve_1(codes: &[&str]) -> usize {
    codes.iter().map(|code| complexity(code)).sum()
}

fn complexity(code: &str) -> usize {
    let numeric = Keypad::numeric();
    let directional = Keypad::directional();

    let mut paths = vec![code.chars().map(Button::from).collect_vec()];
    for keypad in [&numeric, &directional, &directional] {
        paths = paths
            .iter()
            .flat_map(|path| shortest_len(path, keypad).paths)
            .collect_vec();
    }

    let shortest = paths.iter().map(|path| path.len()).min().unwrap();
    let numeric_part = code[..code.len() - 1].parse::<usize>().unwrap();

    shortest * numeric_part
}

fn shortest_len(buttons: &[Button], keypad: &Keypad) -> Paths {
    let mut paths = vec![vec![]];
    let mut from = Button::Activate;

    for to in buttons {
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

    Paths { paths }
}

#[derive(Debug)]
struct Keypad {
    shortest_paths: FxHashMap<Button, FxHashMap<Button, Vec<Vec<Button>>>>,
}

impl Keypad {
    pub fn numeric() -> Self {
        // TODO write a method that generates these, using a 2D grid
        #[rustfmt::skip]
        let graph: FxHashMap<Button, FxHashMap<Button, Button>> = [
            (Button::Seven, [
                    (Button::Eight, Button::Right),
                    (Button::Four, Button::Down),
                ].into_iter().collect(),
            ),
            (Button::Eight, [
                    (Button::Seven, Button::Left),
                    (Button::Nine, Button::Right),
                    (Button::Five, Button::Down),
                ].into_iter().collect(),
            ),
            (Button::Nine, [
                    (Button::Eight, Button::Left),
                    (Button::Six, Button::Down),
                ].into_iter().collect(),
            ),
            (Button::Four, [
                    (Button::Five, Button::Right),
                    (Button::Seven, Button::Up),
                    (Button::One, Button::Down),
                ].into_iter().collect(),
            ),
            (Button::Five, [
                    (Button::Four, Button::Left),
                    (Button::Six, Button::Right),
                    (Button::Eight, Button::Up),
                    (Button::Two, Button::Down),
                ].into_iter().collect(),
            ),
            (Button::Six, [
                    (Button::Five, Button::Left),
                    (Button::Nine, Button::Up),
                    (Button::Three, Button::Down),
                ].into_iter().collect(),
            ),
            (Button::One, [
                    (Button::Two, Button::Right),
                    (Button::Four, Button::Up),
                ].into_iter().collect(),
            ),
            (Button::Two, [
                    (Button::One, Button::Left),
                    (Button::Three, Button::Right),
                    (Button::Five, Button::Up),
                    (Button::Zero, Button::Down),
                ].into_iter().collect(),
            ),
            (Button::Three, [
                    (Button::Two, Button::Left),
                    (Button::Six, Button::Up),
                    (Button::Activate, Button::Down),
                ].into_iter().collect(),
            ),
            (Button::Zero, [
                    (Button::Activate, Button::Right),
                    (Button::Two, Button::Up),
                ].into_iter().collect(),
            ),
            (Button::Activate, [
                    (Button::Zero, Button::Left),
                    (Button::Three, Button::Up),
                ].into_iter().collect(),
            ),
        ]
        .into_iter()
        .collect();

        Self::new(graph)
    }

    pub fn directional() -> Self {
        // TODO write a method that generates these, using a 2D grid
        #[rustfmt::skip]
        let graph: FxHashMap<Button, FxHashMap<Button, Button>> = [
            (Button::Up, [
                    (Button::Activate, Button::Right),
                    (Button::Down, Button::Down),
                ].into_iter().collect(),
            ),
            (Button::Activate, [
                    (Button::Up, Button::Left),
                    (Button::Right, Button::Down),
                ].into_iter().collect(),
            ),
            (Button::Left, [
                    (Button::Down, Button::Right),
                ].into_iter().collect(),
            ),
            (Button::Down, [
                    (Button::Left, Button::Left),
                    (Button::Right, Button::Right),
                    (Button::Up, Button::Up),
                ].into_iter().collect(),
            ),
            (Button::Right, [
                    (Button::Down, Button::Left),
                    (Button::Activate, Button::Up),
                ].into_iter().collect(),
            ),
        ]
        .into_iter()
        .collect();

        Self::new(graph)
    }

    fn new(graph: FxHashMap<Button, FxHashMap<Button, Button>>) -> Self {
        let buttons = graph.keys().copied().collect_vec();

        let shortest_paths = buttons
            .iter()
            .map(|&from| {
                (
                    from,
                    buttons
                        .iter()
                        .map(|&to| (to, Paths::shortest_paths(from, to, &graph).paths))
                        .collect(),
                )
            })
            .collect();

        Self { shortest_paths }
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
    fn from(button: char) -> Self {
        match button {
            '0' => Button::Zero,
            '1' => Button::One,
            '2' => Button::Two,
            '3' => Button::Three,
            '4' => Button::Four,
            '5' => Button::Five,
            '6' => Button::Six,
            '7' => Button::Seven,
            '8' => Button::Eight,
            '9' => Button::Nine,
            'A' => Button::Activate,
            '^' => Button::Up,
            'v' => Button::Down,
            '<' => Button::Left,
            '>' => Button::Right,
            _ => panic!("Invalid button: {}", button),
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

#[derive(Debug)]
struct Paths {
    paths: Vec<Vec<Button>>,
}

impl Paths {
    pub fn shortest_paths(
        from: Button,
        to: Button,
        graph: &FxHashMap<Button, FxHashMap<Button, Button>>,
    ) -> Self {
        Self::all_paths(from, to, FxHashSet::default(), vec![], graph)
            .keep_shortest()
            .keep_shortest_compressed()
    }

    // TODO use dijkstra instead
    fn all_paths(
        from: Button,
        to: Button,
        visited: FxHashSet<Button>,
        path: Vec<Button>,
        graph: &FxHashMap<Button, FxHashMap<Button, Button>>,
    ) -> Self {
        if from == to {
            let mut path = path.clone();
            path.push(Button::Activate);
            Self { paths: vec![path] }
        } else {
            let mut new_visited = visited.clone();
            new_visited.insert(from);

            let paths = graph[&from]
                .iter()
                .filter(|(new_from, _)| visited.contains(new_from).not())
                .flat_map(|(&new_from, &button)| {
                    let mut new_path = path.clone();
                    new_path.push(button);

                    Self::all_paths(new_from, to, new_visited.clone(), new_path, graph).paths
                })
                .collect();
            Self { paths }
        }
    }

    fn keep_shortest(&self) -> Self {
        self.paths
            .clone()
            .into_iter()
            .sorted_by_key(|path| path.len())
            .chunk_by(|path| path.len())
            .into_iter()
            .map(|(len, group)| (len, group.into_iter().collect_vec()))
            .min_by_key(|&(len, _)| len)
            .map(|(_len, paths)| Self { paths })
            .unwrap()
    }

    fn keep_shortest_compressed(&self) -> Self {
        fn compressed_len(path: &[Button]) -> usize {
            if path.len() <= 1 {
                return path.len();
            }

            let mut len = 1;
            let mut prev = path[0];

            for &button in path[1..].iter() {
                if button != prev {
                    len += 1;
                    prev = button;
                }
            }

            len
        }

        self.paths
            .clone()
            .into_iter()
            .sorted_by_key(|path| compressed_len(path))
            .chunk_by(|path| compressed_len(path))
            .into_iter()
            .map(|(len, group)| (len, group.into_iter().collect_vec()))
            .min_by_key(|&(len, _)| len)
            .map(|(_len, paths)| Self { paths })
            .unwrap()
    }
}

impl Display for Paths {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let print = self
            .paths
            .iter()
            .map(|path| path.iter().map(ToString::to_string).join(""))
            .join("\n");

        write!(f, "{}", print)
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
}
