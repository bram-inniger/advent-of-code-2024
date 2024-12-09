use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

pub fn solve_1(disk: &str) -> u64 {
    let mut disk: Vec<_> = disk
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .enumerate()
        .map(|(idx, d)| (idx as u64, d))
        .flat_map(|(idx, d)| (0..d).map(move |_| if idx % 2 == 0 { idx / 2 } else { u64::MAX }))
        .collect();
    let mut space_idx = 0;
    let mut file_idx = disk.len() - 1;

    loop {
        while disk[space_idx] != u64::MAX {
            space_idx += 1;
        }
        while disk[file_idx] == u64::MAX {
            file_idx -= 1;
        }

        if space_idx > file_idx {
            return disk
                .iter()
                .take_while(|&&f| f != u64::MAX)
                .enumerate()
                .map(|(idx, &f)| f * idx as u64)
                .sum();
        }

        disk[space_idx] = disk[file_idx];
        disk[file_idx] = u64::MAX;
    }
}

pub fn solve_2(disk: &str) -> u64 {
    let mut files: Vec<Block> = Vec::new();
    let mut file = true;
    let mut idx = 0;
    let mut spaces = (0..10).map(|_| BinaryHeap::<Block>::new()).collect_vec();

    for (id, d) in disk
        .chars()
        .enumerate()
        .map(|(id, c)| (id as u64 / 2, c.to_digit(10).unwrap() as usize))
    {
        if file {
            let file = Block { idx, len: d, id };
            files.push(file);
        } else {
            let space = Block { idx, len: d, id: 0 };
            spaces[space.len].push(space);
        }

        idx += d;
        file = !file;
    }

    for file in files.iter_mut().rev() {
        let first_space = (file.len..10)
            .flat_map(|idx| spaces[idx].peek().map(|x| (idx, *x)))
            .sorted_by_key(|&(_, b)| b)
            .next_back()
            .filter(|(_, space)| space.idx < file.idx);

        if let Some((idx, _)) = first_space {
            let mut space = spaces[idx].pop().unwrap();

            file.idx = space.idx;
            space.idx += file.len;
            space.len -= file.len;

            spaces[space.len].push(space);
        }
    }

    files
        .iter()
        .flat_map(|f| (f.idx..f.idx + f.len).map(|idx| f.id * idx as u64))
        .sum()
}

// improvement idea, make enum for "File" and "Empty" ?
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Block {
    idx: usize,
    len: usize,
    id: u64,
}

impl Ord for Block {
    fn cmp(&self, other: &Self) -> Ordering {
        self.idx.cmp(&other.idx).reverse()
    }
}

impl PartialOrd for Block {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_09_part_01_sample() {
        let sample = "2333133121414131402";

        assert_eq!(1_928, solve_1(sample));
    }

    #[test]
    fn day_09_part_01_solution() {
        let input = include_str!("../../inputs/day_09.txt").trim();

        assert_eq!(6_201_130_364_722, solve_1(input));
    }

    #[test]
    fn day_09_part_02_sample() {
        let sample = "2333133121414131402";

        assert_eq!(2_858, solve_2(sample));
    }

    #[test]
    fn day_09_part_02_solution() {
        let input = include_str!("../../inputs/day_09.txt").trim();

        assert_eq!(6_221_662_795_602, solve_2(input));
    }
}
