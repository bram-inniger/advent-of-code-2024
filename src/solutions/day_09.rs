pub fn solve_1(disk_map: &str) -> u64 {
    // Naive and slow implementation, should speed up by keeping
    // a running deque of both open and closed spaces and running them in opposing orders
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

    loop {
        let first_free = (0..disk.len()).find(|&idx| disk[idx] == u64::MAX).unwrap();
        let last_file = (0..disk.len())
            .rev()
            .find(|&idx| disk[idx] != u64::MAX)
            .unwrap();

        if last_file < first_free {
            return disk
                .iter()
                .take_while(|&&f| f != u64::MAX)
                .enumerate()
                .map(|(idx, &f)| f * idx as u64)
                .sum();
        }

        disk[first_free] = disk[last_file];
        disk[last_file] = u64::MAX;
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
}
