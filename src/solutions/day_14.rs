use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

pub fn solve_1(robots: &[&str], room: &Room) -> u32 {
    robots
        .iter()
        .map(|robot| Robot::new(robot))
        .map(|robot| robot.walk(100, room))
        .flat_map(|robot| room.quadrant(robot.position))
        .sorted()
        .chunk_by(|&quadrant| quadrant)
        .into_iter()
        .map(|(_, group)| group.count() as u32)
        .reduce(|a, b| a * b)
        .unwrap()
}

lazy_static! {
    static ref ROBOT_RE: Regex =
        Regex::new(r"^p=(?<px>-?\d+),(?<py>-?\d+) v=(?<vx>-?\d+),(?<vy>-?\d+)$").unwrap();
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Robot {
    position: Pair,
    velocity: Pair,
}

impl Robot {
    fn new(robot: &str) -> Self {
        let captures = ROBOT_RE.captures(robot).unwrap();
        let px = captures.name("px").unwrap().as_str().parse().unwrap();
        let py = captures.name("py").unwrap().as_str().parse().unwrap();
        let vx = captures.name("vx").unwrap().as_str().parse().unwrap();
        let vy = captures.name("vy").unwrap().as_str().parse().unwrap();

        Self {
            position: Pair { x: px, y: py },
            velocity: Pair { x: vx, y: vy },
        }
    }

    fn walk(&self, time: i32, room: &Room) -> Self {
        let Pair { x: px, y: py } = self.position;
        let Pair { x: vx, y: vy } = self.velocity;

        Self {
            position: Pair {
                x: (px + (time * vx)).rem_euclid(room.width),
                y: (py + (time * vy)).rem_euclid(room.height),
            },
            velocity: self.velocity,
        }
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Pair {
    x: i32,
    y: i32,
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Room {
    width: i32,
    height: i32,
}

impl Room {
    fn quadrant(&self, position: Pair) -> Option<u32> {
        match self {
            _ if position.x < self.width / 2 && position.y < self.height / 2 => Some(1),
            _ if position.x > self.width / 2 && position.y < self.height / 2 => Some(2),
            _ if position.x < self.width / 2 && position.y > self.height / 2 => Some(3),
            _ if position.x > self.width / 2 && position.y > self.height / 2 => Some(4),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn day_14_part_01_sample() {
        let sample = vec![
            "p=0,4 v=3,-3",
            "p=6,3 v=-1,-3",
            "p=10,3 v=-1,2",
            "p=2,0 v=2,-1",
            "p=0,0 v=1,3",
            "p=3,0 v=-2,-2",
            "p=7,6 v=-1,-3",
            "p=3,0 v=-1,-2",
            "p=9,3 v=2,3",
            "p=7,3 v=-1,2",
            "p=2,4 v=2,-3",
            "p=9,5 v=-3,-3",
        ];
        let room = Room {
            width: 11,
            height: 7,
        };

        assert_eq!(12, solve_1(&sample, &room));
    }

    #[test]
    fn day_14_part_01_solution() {
        let input = include_str!("../../inputs/day_14.txt")
            .lines()
            .collect_vec();
        let room = Room {
            width: 101,
            height: 103,
        };

        assert_eq!(229_421_808, solve_1(&input, &room));
    }
}
