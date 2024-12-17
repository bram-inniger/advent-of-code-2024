use itertools::Itertools;

pub fn solve_1(program: &[&str]) -> String {
    Computer::new(program).run().iter().join(",")
}

pub fn solve_2(program: &[&str]) -> u64 {
    let instructions = Computer::new(program).instructions;
    let mut guesses = vec![0u64];

    for idx in 1..=instructions.len() {
        guesses = guesses
            .into_iter()
            .flat_map(|valid_guess| (0..8).map(move |guess| valid_guess * 8 + guess))
            .filter(|&guess| {
                let mut computer = Computer::new(program);
                computer.register_a = guess;
                let out = computer.run();

                out.len() >= idx && out[out.len() - idx] == instructions[instructions.len() - idx]
            })
            .collect_vec();
    }

    *guesses.iter().min().unwrap()
}

#[derive(Debug)]
struct Computer {
    register_a: u64,
    register_b: u64,
    register_c: u64,
    instructions: Vec<u64>,
    ipr: usize,
}

impl Computer {
    fn new(program: &[&str]) -> Self {
        let register_a = program[0][12..].parse().unwrap();
        let register_b = program[1][12..].parse().unwrap();
        let register_c = program[2][12..].parse().unwrap();
        let instructions = program[4][9..]
            .split(',')
            .map(|instruction| instruction.parse().unwrap())
            .collect();

        Self {
            register_a,
            register_b,
            register_c,
            instructions,
            ipr: 0,
        }
    }

    fn run(&mut self) -> Vec<u64> {
        let mut output = vec![];

        loop {
            let Some(&opcode) = self.instructions.get(self.ipr) else {
                return output;
            };
            let operand = self.instructions[self.ipr + 1];

            if let Some(out) = Instruction::from(opcode).execute(operand, self) {
                output.push(out);
            };
        }
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum Instruction {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl Instruction {
    fn from(opcode: u64) -> Self {
        match opcode {
            0 => Self::Adv,
            1 => Self::Bxl,
            2 => Self::Bst,
            3 => Self::Jnz,
            4 => Self::Bxc,
            5 => Self::Out,
            6 => Self::Bdv,
            7 => Self::Cdv,
            _ => unreachable!(),
        }
    }

    fn execute(&self, operand: u64, computer: &mut Computer) -> Option<u64> {
        let combo = |combo_operand| match combo_operand {
            (0..=3) => combo_operand,
            4 => computer.register_a,
            5 => computer.register_b,
            6 => computer.register_c,
            _ => unreachable!(),
        };

        let mut jump = false;
        let mut output = None;

        match self {
            Instruction::Adv => computer.register_a /= 2u64.pow(combo(operand) as u32),
            Instruction::Bxl => computer.register_b ^= operand,
            Instruction::Bst => computer.register_b = combo(operand) % 8,
            Instruction::Jnz => {
                if computer.register_a != 0 {
                    computer.ipr = operand as usize;
                    jump = true;
                }
            }
            Instruction::Bxc => computer.register_b ^= computer.register_c,
            Instruction::Out => output = Some(combo(operand) % 8),
            Instruction::Bdv => {
                computer.register_b = computer.register_a / 2u64.pow(combo(operand) as u32)
            }
            Instruction::Cdv => {
                computer.register_c = computer.register_a / 2u64.pow(combo(operand) as u32)
            }
        }

        if !jump {
            computer.ipr += 2;
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;

    #[test]
    fn day_17_part_01_sample() {
        let sample = vec![
            "Register A: 729",
            "Register B: 0",
            "Register C: 0",
            "",
            "Program: 0,1,5,4,3,0",
        ];

        assert_eq!("4,6,3,5,6,3,5,2,1,0", solve_1(&sample));
    }

    #[test]
    fn day_17_part_01_solution() {
        let input = include_str!("../../inputs/day_17.txt")
            .lines()
            .collect_vec();

        assert_eq!("1,6,3,6,5,6,5,1,7", solve_1(&input));
    }

    #[test]
    fn day_17_part_02_sample() {
        let sample = vec![
            "Register A: 2024",
            "Register B: 0",
            "Register C: 0",
            "",
            "Program: 0,3,5,4,3,0",
        ];

        assert_eq!(117_440, solve_2(&sample));
    }

    #[test]
    fn day_17_part_02_solution() {
        let input = include_str!("../../inputs/day_17.txt")
            .lines()
            .collect_vec();

        assert_eq!(247_839_653_009_594, solve_2(&input));
    }
}
