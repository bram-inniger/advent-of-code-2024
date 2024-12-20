use itertools::Itertools;

pub fn solve_1(word_search: &[&str]) -> usize {
    let word_search = WordSearch::new(word_search);

    (0..word_search.height)
        .flat_map(|y| (0..word_search.width).map(move |x| (x, y)))
        .map(|(x, y)| word_search.nr_xmas_at(x, y))
        .sum()
}

pub fn solve_2(word_search: &[&str]) -> usize {
    let word_search = WordSearch::new(word_search);

    (1..word_search.height - 1)
        .flat_map(|y| (1..word_search.width - 1).map(move |x| (x, y)))
        .filter(|&(x, y)| word_search.is_x_mas_at(x, y))
        .count()
}

#[derive(Debug)]
struct WordSearch {
    grid: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

impl WordSearch {
    fn new(word_search: &[&str]) -> Self {
        let grid = word_search
            .iter()
            .map(|line| line.chars().collect_vec())
            .collect_vec();
        let width = grid[0].len();
        let height = grid.len();

        Self {
            grid,
            width,
            height,
        }
    }

    fn nr_xmas_at(&self, x: usize, y: usize) -> usize {
        let is_xmas = |delta: &dyn Fn(i32) -> Delta| {
            (0..4)
                .map(|idx| {
                    self.grid[(y as i32 + delta(idx).delta_y) as usize]
                        [(x as i32 + delta(idx).delta_x) as usize]
                })
                .collect::<String>()
                == "XMAS"
        };

        [
            x < self.width - 3 && y >= 3 && is_xmas(&(|idx| Delta::new(idx, -idx))),
            x < self.width - 3 && is_xmas(&(|idx| Delta::new(idx, 0))),
            x < self.width - 3 && y < self.height - 3 && is_xmas(&(|idx| Delta::new(idx, idx))),
            y >= 3 && is_xmas(&(|idx| Delta::new(0, -idx))),
            y < self.height - 3 && is_xmas(&(|idx| Delta::new(0, idx))),
            x >= 3 && y < self.height - 3 && is_xmas(&(|idx| Delta::new(-idx, idx))),
            x >= 3 && is_xmas(&(|idx| Delta::new(-idx, 0))),
            x >= 3 && y >= 3 && is_xmas(&(|idx| Delta::new(-idx, -idx))),
        ]
        .iter()
        .filter(|&&b| b)
        .count()
    }

    fn is_x_mas_at(&self, x: usize, y: usize) -> bool {
        self.grid[y][x] == 'A'
            && ((self.grid[y - 1][x - 1] == 'M' && self.grid[y + 1][x + 1] == 'S')
                || (self.grid[y - 1][x - 1] == 'S' && self.grid[y + 1][x + 1] == 'M'))
            && ((self.grid[y - 1][x + 1] == 'M' && self.grid[y + 1][x - 1] == 'S')
                || (self.grid[y - 1][x + 1] == 'S' && self.grid[y + 1][x - 1] == 'M'))
    }
}

#[derive(Debug)]
struct Delta {
    delta_x: i32,
    delta_y: i32,
}

impl Delta {
    fn new(delta_x: i32, delta_y: i32) -> Self {
        Self { delta_x, delta_y }
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn day_04_part_01_sample() {
        let sample = vec![
            "MMMSXXMASM",
            "MSAMXMSMSA",
            "AMXSXMAAMM",
            "MSAMASMSMX",
            "XMASAMXAMM",
            "XXAMMXXAMA",
            "SMSMSASXSS",
            "SAXAMASAAA",
            "MAMMMXMMMM",
            "MXMXAXMASX",
        ];

        assert_eq!(18, solve_1(&sample));
    }

    #[test]
    fn day_04_part_01_solution() {
        let input = include_str!("../../inputs/day_04.txt")
            .lines()
            .collect_vec();

        assert_eq!(2_458, solve_1(&input));
    }

    #[test]
    fn day_04_part_02_sample() {
        let sample = vec![
            "MMMSXXMASM",
            "MSAMXMSMSA",
            "AMXSXMAAMM",
            "MSAMASMSMX",
            "XMASAMXAMM",
            "XXAMMXXAMA",
            "SMSMSASXSS",
            "SAXAMASAAA",
            "MAMMMXMMMM",
            "MXMXAXMASX",
        ];

        assert_eq!(9, solve_2(&sample));
    }

    #[test]
    fn day_04_part_02_solution() {
        let input = include_str!("../../inputs/day_04.txt")
            .lines()
            .collect_vec();

        assert_eq!(1_945, solve_2(&input));
    }
}
