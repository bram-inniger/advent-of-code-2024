pub fn solve_1(disk_map: &str) -> u64 {
    let mut disk: Vec<u64> = vec![];
    let mut file = true;
    let mut id = 0;

    for c in disk_map.chars().map(|c| c.to_digit(10).unwrap()) {
        if file {
            (0..c).for_each(|_| disk.push(id));
            id += 1;
        } else {
            (0..c).for_each(|_| disk.push(u64::MAX));
        }

        file = !file;
    }

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

pub fn solve_2(disk_map: &str) -> u64 {
    let mut files: Vec<Block> = Vec::new();
    let mut spaces: Vec<Block> = Vec::new();
    let mut file = true;
    let mut id = 0;
    let mut idx = 0;

    for d in disk_map.chars().map(|c| c.to_digit(10).unwrap() as usize) {
        if file {
            let file = Block { idx, len: d, id };

            files.push(file);
            id += 1;
        } else {
            let space = Block { idx, len: d, id: 0 };

            spaces.push(space);
        }

        idx += d;
        file = !file;
    }

    for file in files.iter_mut().rev() {
        if let Some(space) = spaces
            .iter_mut()
            .take_while(|s| s.idx < file.idx)
            .find(|s| s.len >= file.len)
        {
            file.idx = space.idx;
            space.idx += file.len;
            space.len -= file.len;
        }
    }

    files
        .iter()
        .flat_map(|f| (f.idx..f.idx + f.len).map(|idx| f.id * idx as u64))
        .sum()
}

// improvement idea, make enum for "File" and "Empty" ?
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Block {
    idx: usize,
    len: usize,
    id: u64,
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
