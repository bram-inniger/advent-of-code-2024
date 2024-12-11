use std::cmp::Ordering;
use std::collections::{HashMap, HashSet, VecDeque};

pub fn solve_1(program: &[&str]) -> i64 {
    let program = program
        .iter()
        .map(|instruction| Instruction::new(instruction))
        .collect::<Vec<_>>();

    (0..30)
        .flat_map(|x| (0..30).flat_map(move |y| (0..30).map(move |z| Cube { x, y, z })))
        .map(|cube| cube.snow(&program))
        .sum()
}

pub fn solve_2(program: &[&str]) -> usize {
    let program = program
        .iter()
        .map(|instruction| Instruction::new(instruction))
        .collect::<Vec<_>>();

    let cubes = (0..30)
        .flat_map(|x| (0..30).flat_map(move |y| (0..30).map(move |z| Cube { x, y, z })))
        .filter(|cube| cube.snow(&program) > 0)
        .collect::<Vec<_>>();
    let cubes_set = cubes.iter().copied().collect::<HashSet<_>>();
    let mut clouds = UnionFind::new(&cubes);

    for cube in cubes.iter() {
        for neighbour in cube.in_cloud(&cubes_set) {
            clouds.union(cube, &neighbour)
        }
    }

    clouds.set_count()
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
                Instruction::PushNum { value } => stack.push_back(value),
                Instruction::PushSym { symbol } => match symbol {
                    Symbol::X => stack.push_back(self.x),
                    Symbol::Y => stack.push_back(self.y),
                    Symbol::Z => stack.push_back(self.z),
                },
                Instruction::Add => {
                    let n_1 = stack.pop_back().unwrap();
                    let n_2 = stack.pop_back().unwrap();
                    stack.push_back(n_1 + n_2);
                }
                Instruction::Jmpos { offset } => {
                    let n = stack.pop_back().unwrap();
                    if n >= 0 {
                        pc += offset as usize;
                    }
                }
                Instruction::Ret => break,
            }

            pc += 1;
        }

        stack.pop_back().unwrap()
    }

    fn in_cloud(&self, clouds: &HashSet<Cube>) -> Vec<Cube> {
        #[rustfmt::skip]
        let neighbours = [
            Cube { x: self.x + 1, y: self.y, z: self.z },
            Cube { x: self.x, y: self.y + 1, z: self.z },
            Cube { x: self.x, y: self.y, z: self.z +1 },
        ];

        neighbours
            .iter()
            .filter(|neighbour| clouds.contains(neighbour))
            .copied()
            .collect()
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
            "push x" | "push X" => Instruction::PushSym { symbol: Symbol::X },
            "push y" | "push Y" => Instruction::PushSym { symbol: Symbol::Y },
            "push z" | "push Z" => Instruction::PushSym { symbol: Symbol::Z },
            _ if instruction.starts_with("push") => Instruction::PushNum {
                value: instruction[5..].parse::<i64>().unwrap(),
            },
            "add" => Instruction::Add,
            _ if instruction.starts_with("jmpos") => Instruction::Jmpos {
                offset: instruction[6..].parse::<i64>().unwrap(),
            },
            "ret" => Instruction::Ret,
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

#[derive(Debug)]
struct UnionFind {
    parents: Vec<usize>,
    ranks: Vec<usize>,
    cube_indices: HashMap<Cube, usize>,
}

impl UnionFind {
    fn new(cubes: &[Cube]) -> Self {
        let n = cubes.len();

        let cube_indices = cubes
            .iter()
            .enumerate()
            .map(|(idx, cube)| (*cube, idx))
            .collect::<HashMap<_, _>>();

        UnionFind {
            parents: (0..n).collect(),
            ranks: vec![0; n],
            cube_indices,
        }
    }

    fn find(&mut self, cube: &Cube) -> usize {
        let idx = self.cube_indices[cube];

        self.find_helper(self.parents[idx])
    }

    fn find_helper(&mut self, idx: usize) -> usize {
        // Path compression
        if self.parents[idx] != idx {
            self.parents[idx] = self.find_helper(self.parents[idx]);
        }
        self.parents[idx]
    }

    fn union(&mut self, cube_1: &Cube, cube_2: &Cube) {
        let root_1 = self.find(cube_1);
        let root_2 = self.find(cube_2);

        if root_1 != root_2 {
            // Union by rank
            match self.ranks[root_1].cmp(&self.ranks[root_2]) {
                Ordering::Less => self.parents[root_1] = root_2,
                Ordering::Greater => self.parents[root_2] = root_1,
                Ordering::Equal => {
                    self.parents[root_2] = root_1;
                    self.ranks[root_1] += 1;
                }
            }
        }
    }

    pub fn set_count(&mut self) -> usize {
        (0..self.parents.len())
            .map(|idx| self.find_helper(idx))
            .collect::<HashSet<_>>()
            .len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;

    #[test]
    fn infi_part_01_sample() {
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
    fn infi_part_01_solution() {
        let input = include_str!("../../inputs/infi/infi.txt")
            .lines()
            .collect_vec();

        assert_eq!(4_375, solve_1(&input));
    }

    #[test]
    fn infi_part_02_sample() {
        // No sample input provided
    }

    #[test]
    fn infi_part_02_solution() {
        let input = include_str!("../../inputs/infi/infi.txt")
            .lines()
            .collect_vec();

        assert_eq!(16, solve_2(&input));
    }
}
