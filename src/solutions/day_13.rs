use lazy_static::lazy_static;
use regex::Regex;

pub fn solve_1(machines: &str) -> i32 {
    machines
        .split("\n\n")
        .map(Machine::new)
        .map(|machine| machine.min_tokens())
        .sum()
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Machine {
    button_a: Position,
    button_b: Position,
    prize: Position,
}

impl Machine {
    fn new(machine: &str) -> Self {
        let lines = machine.lines().collect::<Vec<_>>();

        let button_a = BUTTON_RE.captures(lines[0]).unwrap();
        let x_a = button_a.name("x").unwrap().as_str().parse::<i32>().unwrap();
        let y_a = button_a.name("y").unwrap().as_str().parse::<i32>().unwrap();
        let button_a = Position { x: x_a, y: y_a };

        let button_b = BUTTON_RE.captures(lines[1]).unwrap();
        let x_b = button_b.name("x").unwrap().as_str().parse::<i32>().unwrap();
        let y_b = button_b.name("y").unwrap().as_str().parse::<i32>().unwrap();
        let button_b = Position { x: x_b, y: y_b };

        let prize = PRIZE_RE.captures(lines[2]).unwrap();
        let x_prize = prize.name("x").unwrap().as_str().parse::<i32>().unwrap();
        let y_prize = prize.name("y").unwrap().as_str().parse::<i32>().unwrap();
        let prize = Position {
            x: x_prize,
            y: y_prize,
        };

        Self {
            button_a,
            button_b,
            prize,
        }
    }

    fn min_tokens(&self) -> i32 {
        (0..=100)
            .map(|a_times| {
                let x_rem = self.prize.x - self.button_a.x * a_times;
                let b_times = x_rem / self.button_b.x;

                (a_times, b_times)
            })
            .filter(|&(a_times, b_times)| {
                b_times >= 0
                    && a_times * self.button_a.x + b_times * self.button_b.x == self.prize.x
                    && a_times * self.button_a.y + b_times * self.button_b.y == self.prize.y
            })
            .map(|(a_times, b_times)| a_times * 3 + b_times)
            .min()
            .unwrap_or(0)
    }
}

lazy_static! {
    static ref BUTTON_RE: Regex =
        Regex::new(r"^Button (A|B): X\+(?<x>\d+), Y\+(?<y>\d+)$").unwrap();
    static ref PRIZE_RE: Regex = Regex::new(r"^Prize: X=(?<x>\d+), Y=(?<y>\d+)$").unwrap();
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_13_part_01_sample() {
        let sample = "\
                Button A: X+94, Y+34\n\
                Button B: X+22, Y+67\n\
                Prize: X=8400, Y=5400\n\
                \n\
                Button A: X+26, Y+66\n\
                Button B: X+67, Y+21\n\
                Prize: X=12748, Y=12176\n\
                \n\
                Button A: X+17, Y+86\n\
                Button B: X+84, Y+37\n\
                Prize: X=7870, Y=6450\n\
                \n\
                Button A: X+69, Y+23\n\
                Button B: X+27, Y+71\n\
                Prize: X=18641, Y=10279\
            ";

        assert_eq!(480, solve_1(sample));
    }

    #[test]
    fn day_13_part_01_solution() {
        let input = include_str!("../../inputs/day_13.txt").trim();

        assert_eq!(33_209, solve_1(input));
    }
}
