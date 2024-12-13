use lazy_static::lazy_static;
use regex::Regex;

pub fn solve_1(machines: &str) -> i64 {
    solve(machines, None, Some(100))
}

pub fn solve_2(machines: &str) -> i64 {
    solve(machines, Some(10_000_000_000_000), None)
}

fn solve(machines: &str, addition: Option<i64>, max_nr_pushes: Option<i64>) -> i64 {
    machines
        .split("\n\n")
        .map(|machine| Machine::new(machine, addition))
        .filter_map(|machine| machine.min_tokens(max_nr_pushes))
        .sum()
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Machine {
    button_a: Position,
    button_b: Position,
    prize: Position,
}

impl Machine {
    fn new(machine: &str, addition: Option<i64>) -> Self {
        let lines = machine.lines().collect::<Vec<_>>();

        let button_a = Position::from_str(lines[0], &BUTTON_RE);
        let button_b = Position::from_str(lines[1], &BUTTON_RE);
        let mut prize = Position::from_str(lines[2], &PRIZE_RE);

        if let Some(addition) = addition {
            prize.x += addition;
            prize.y += addition;
        }

        Self {
            button_a,
            button_b,
            prize,
        }
    }

    fn min_tokens(&self, max_nr_pushes: Option<i64>) -> Option<i64> {
        // The system is modeled by the following set of equations:
        //
        // 1. button_a.x * pushes_a + button_b.x * pushes_b = price.x
        // 2. button_a.y * pushes_a + button_b.y * pushes_b = price.y
        // 3. pushes_a, pushes_b ∈ ℕ
        // 4. cost = 3 * pushes_a + 1 * pushes_b
        //
        // Solving for both 1 and 2 will yield "pushes_a" and "pushes_b"
        // These should then be verified to exist in the set of Natural number
        // If they do, return the result of the cost function, otherwise return 0

        let max_nr_pushes = max_nr_pushes.unwrap_or(i64::MAX);

        let Position { x: a_x, y: a_y } = self.button_a;
        let Position { x: b_x, y: b_y } = self.button_b;
        let Position { x: p_x, y: p_y } = self.prize;

        let pushes_b = (a_x * p_y - a_y * p_x) / (a_x * b_y - a_y * b_x);
        let pushes_a = (p_x - b_x * pushes_b) / a_x;

        if (0..=max_nr_pushes).contains(&pushes_a)
            && (0..=max_nr_pushes).contains(&pushes_b)
            && a_x * pushes_a + b_x * pushes_b == self.prize.x
            && a_y * pushes_a + b_y * pushes_b == self.prize.y
        {
            Some(3 * pushes_a + pushes_b)
        } else {
            None
        }
    }
}

lazy_static! {
    static ref BUTTON_RE: Regex =
        Regex::new(r"^Button (A|B): X\+(?<x>\d+), Y\+(?<y>\d+)$").unwrap();
    static ref PRIZE_RE: Regex = Regex::new(r"^Prize: X=(?<x>\d+), Y=(?<y>\d+)$").unwrap();
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Position {
    x: i64,
    y: i64,
}

impl Position {
    fn from_str(position: &str, regex: &Regex) -> Self {
        let position = regex.captures(position).unwrap();
        let x = position.name("x").unwrap().as_str().parse::<i64>().unwrap();
        let y = position.name("y").unwrap().as_str().parse::<i64>().unwrap();
        Self { x, y }
    }
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

    #[test]
    fn day_13_part_02_sample() {
        // No sample solution provided
    }

    #[test]
    fn day_13_part_02_solution() {
        let input = include_str!("../../inputs/day_13.txt").trim();

        assert_eq!(83_102_355_665_474, solve_2(input));
    }
}
