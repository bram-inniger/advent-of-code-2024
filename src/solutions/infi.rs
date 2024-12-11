use crate::solutions::infi::Instruction::{Add, Jmpos, PushNum, PushSym, Ret};
use itertools::Itertools;
use std::collections::VecDeque;

pub fn solve_1(program: &[&str]) -> i64 {
    let program = program
        .iter()
        .map(|instruction| Instruction::new(instruction))
        .collect_vec();

    (0..30)
        .flat_map(|x| (0..30).flat_map(move |y| (0..30).map(move |z| Cube { x, y, z })))
        .map(|cube| cube.snow(&program))
        .sum()
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Cube {
    x: i64,
    y: i64,
    z: i64,
}

impl Cube {
    fn snow(&self, program: &[Instruction]) -> i64 {
        let mut stack = VecDeque::<i64>::new();
        let mut pc = 0usize;

        loop {
            match program[pc] {
                PushNum { value } => stack.push_back(value),
                PushSym { symbol } => match symbol {
                    Symbol::X => stack.push_back(self.x),
                    Symbol::Y => stack.push_back(self.y),
                    Symbol::Z => stack.push_back(self.z),
                },
                Add => {
                    let n_1 = stack.pop_back().unwrap();
                    let n_2 = stack.pop_back().unwrap();
                    stack.push_back(n_1 + n_2);
                }
                Jmpos { offset } => {
                    let n = stack.pop_back().unwrap();
                    if n >= 0 {
                        pc += offset as usize;
                    }
                }
                Ret => break,
            }

            pc += 1;
        }

        stack.pop_back().unwrap()
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum Instruction {
    PushNum { value: i64 },
    PushSym { symbol: Symbol },
    Add,
    Jmpos { offset: i64 },
    Ret,
}

impl Instruction {
    fn new(instruction: &str) -> Self {
        match instruction {
            "push x" | "push X" => PushSym { symbol: Symbol::X },
            "push y" | "push Y" => PushSym { symbol: Symbol::Y },
            "push z" | "push Z" => PushSym { symbol: Symbol::Z },
            _ if instruction.starts_with("push") => PushNum {
                value: instruction[5..].parse::<i64>().unwrap(),
            },
            "add" => Add,
            _ if instruction.starts_with("jmpos") => Jmpos {
                offset: instruction[6..].parse::<i64>().unwrap(),
            },
            "ret" => Ret,
            _ => panic!("Invalid instruction: {}", instruction),
        }
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum Symbol {
    X,
    Y,
    Z,
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;

    #[test]
    fn day_01_part_01_sample() {
        #[rustfmt::skip]
        let sample = vec![
            "push 999",
            "push x",
            "push -3",
            "add",
            "jmpos 2",
            "ret",
            "ret",
            "push 123",
            "ret",
        ];

        assert_eq!(5_686_200, solve_1(&sample));
    }

    #[test]
    fn day_01_part_01_solution() {
        let input = include_str!("../../inputs/infi/infi.txt")
            .lines()
            .collect_vec();

        assert_eq!(4_375, solve_1(&input));
    }
}
