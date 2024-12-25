use itertools::Itertools;

pub fn solve_1(schematics: &str) -> usize {
    let [locks, keys]: [Vec<_>; 2] = schematics
        .split("\n\n")
        .map(Schematic::new)
        .sorted_by_key(|schematic| schematic.schematic_type)
        .chunk_by(|schematic| schematic.schematic_type)
        .into_iter()
        .map(|(schema_type, group)| (schema_type, group.into_iter().collect_vec()))
        .map(|(_, schematics)| schematics)
        .collect_vec()
        .try_into()
        .unwrap();

    locks
        .iter()
        .flat_map(|lock| keys.iter().map(move |key| (lock, key)))
        .filter(|(lock, key)| (0..5).all(|idx| lock.pins[idx] < key.pins[idx]))
        .count()
}

pub fn solve_2() {
    // Deliver The Chronicle
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Schematic {
    pins: [usize; 5],
    schematic_type: SchematicType,
}

impl Schematic {
    fn new(schematic: &str) -> Self {
        let schematic_type = match schematic.split('\n').next().unwrap() {
            "....." => SchematicType::Key,
            "#####" => SchematicType::Lock,
            _ => panic!("Invalid schematic: {}", schematic),
        };

        let schematic = schematic
            .split('\n')
            .map(|line| line.chars().collect_vec())
            .collect_vec();
        let heights = match schematic_type {
            SchematicType::Lock => (0..schematic.len()).collect_vec(),
            SchematicType::Key => (0..schematic.len()).rev().collect_vec(),
        };

        let pins = (0..schematic[0].len())
            .map(|idx| {
                heights
                    .iter()
                    .take_while(|&&height| schematic[height][idx] == '#')
                    .last()
                    .unwrap()
            })
            .copied()
            .collect_vec()
            .try_into()
            .unwrap();

        Self {
            pins,
            schematic_type,
        }
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum SchematicType {
    Lock,
    Key,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_25_part_01_sample() {
        let sample = "\
                #####\n\
                .####\n\
                .####\n\
                .####\n\
                .#.#.\n\
                .#...\n\
                .....\n\
                \n\
                #####\n\
                ##.##\n\
                .#.##\n\
                ...##\n\
                ...#.\n\
                ...#.\n\
                .....\n\
                \n\
                .....\n\
                #....\n\
                #....\n\
                #...#\n\
                #.#.#\n\
                #.###\n\
                #####\n\
                \n\
                .....\n\
                .....\n\
                #.#..\n\
                ###..\n\
                ###.#\n\
                ###.#\n\
                #####\n\
                \n\
                .....\n\
                .....\n\
                .....\n\
                #....\n\
                #.#..\n\
                #.#.#\n\
                #####\
            ";

        assert_eq!(3, solve_1(sample));
    }

    #[test]
    fn day_25_part_01_solution() {
        let input = include_str!("../../inputs/day_25.txt").trim();

        assert_eq!(3_223, solve_1(input));
    }

    #[test]
    fn day_25_part_02_sample() {
        solve_2();
    }

    #[test]
    fn day_25_part_02_solution() {
        solve_2();
    }
}
