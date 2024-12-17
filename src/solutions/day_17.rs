use itertools::Itertools;

pub fn solve_1(program: &[&str]) -> String {
    Computer::new(program).run().iter().join(",")
}

#[derive(Debug)]
struct Computer {
    register_a: u32,
    register_b: u32,
    register_c: u32,
    instructions: Vec<u32>,
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

    fn run(&mut self) -> Vec<u32> {
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
    fn from(opcode: u32) -> Self {
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

    fn execute(&self, operand: u32, computer: &mut Computer) -> Option<u32> {
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
            Instruction::Adv => computer.register_a /= 2u32.pow(combo(operand)),
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
                computer.register_b = computer.register_a / 2u32.pow(combo(operand))
            }
            Instruction::Cdv => {
                computer.register_c = computer.register_a / 2u32.pow(combo(operand))
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
}
