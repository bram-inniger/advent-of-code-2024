use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};
use std::fs::File;
use std::io;
use std::io::Write;
use std::ops::Not;

pub fn solve_1(system: &str) -> u64 {
    let System {
        mut wires,
        mut gates,
    } = System::new(system);

    while gates.is_empty().not() {
        let ready = gates
            .iter()
            .filter(|gate| wires.contains_key(&gate.in_1) && wires.contains_key(&gate.in_2))
            .cloned()
            .collect_vec();

        for gate in ready {
            wires.insert(gate.out.clone(), gate.value(&wires));
            gates.remove(&gate);
        }
    }

    let output = wires
        .iter()
        .filter(|(wire, _)| wire.starts_with("z"))
        .sorted_by_key(|(wire, _)| *wire)
        .rev()
        .map(|(_, value)| value.to_string())
        .join("");

    u64::from_str_radix(&output, 2).unwrap()
}

pub fn solve_2(_system: &str) -> String {
    "".to_owned()
}

#[derive(Debug)]
struct System {
    wires: FxHashMap<String, u32>,
    gates: FxHashSet<Gate>,
}

impl System {
    fn new(system: &str) -> Self {
        let (wires, gates) = system.split_once("\n\n").unwrap();

        let wires: FxHashMap<String, u32> = wires
            .split('\n')
            .map(|wire| {
                let (wire, value) = wire.split_once(": ").unwrap();
                (wire.to_owned(), value.parse::<u32>().unwrap())
            })
            .collect();
        let gates: FxHashSet<Gate> = gates.split('\n').map(Gate::new).collect();

        Self { wires, gates }
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Gate {
    in_1: String,
    in_2: String,
    out: String,
    gate_type: GateType,
}

impl Gate {
    fn new(gate: &str) -> Self {
        let split = gate.split(" ").collect_vec();

        let in_1 = split[0].to_owned();
        let in_2 = split[2].to_owned();
        let out = split[4].to_owned();
        let gate_type = match split[1] {
            "AND" => GateType::And,
            "OR" => GateType::Or,
            "XOR" => GateType::Xor,
            _ => panic!("Invalid gate: {}", gate),
        };

        Self {
            in_1,
            in_2,
            out,
            gate_type,
        }
    }

    fn value(&self, wires: &FxHashMap<String, u32>) -> u32 {
        let in_1 = wires[&self.in_1];
        let in_2 = wires[&self.in_2];

        match self.gate_type {
            GateType::And => in_1 & in_2,
            GateType::Or => in_1 | in_2,
            GateType::Xor => in_1 ^ in_2,
        }
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum GateType {
    And,
    Or,
    Xor,
}

#[allow(dead_code)]
fn generate_graphviz_file(system: &str) -> Result<(), io::Error> {
    let mut file = File::create("graphviz/day_24.dot")?;

    let gates = System::new(system).gates;

    file.write_all("// Create a .png from this file using either: \n".as_bytes())?;
    file.write_all("// `$ dot -Tpng day_24.dot -o day_24-dot.png`\n".as_bytes())?;
    file.write_all("// `$ neato -Tpng day_24.dot -o day_24-neato.png`\n\n".as_bytes())?;
    file.write_all("Digraph G {\n".as_bytes())?;

    for gate in &gates {
        let shape = match gate.gate_type {
            GateType::And => "square",
            GateType::Or => "oval",
            GateType::Xor => "diamond",
        };
        let node = format!("    {} [shape={}]\n", gate.out, shape);
        file.write_all(node.as_bytes())?;
    }

    file.write_all("\n".as_bytes())?;

    for gate in &gates {
        let edge_1 = format!("    {} -> {}\n", gate.in_1, gate.out);
        let edge_2 = format!("    {} -> {}\n", gate.in_2, gate.out);
        file.write_all(edge_1.as_bytes())?;
        file.write_all(edge_2.as_bytes())?;
    }

    file.write_all("}".as_bytes())?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_24_part_01_sample() {
        let sample_1 = "\
                x00: 1\n\
                x01: 1\n\
                x02: 1\n\
                y00: 0\n\
                y01: 1\n\
                y02: 0\n\
                \n\
                x00 AND y00 -> z00\n\
                x01 XOR y01 -> z01\n\
                x02 OR y02 -> z02\
            ";

        let sample_2 = "\
                x00: 1\n\
                x01: 0\n\
                x02: 1\n\
                x03: 1\n\
                x04: 0\n\
                y00: 1\n\
                y01: 1\n\
                y02: 1\n\
                y03: 1\n\
                y04: 1\n\
                \n\
                ntg XOR fgs -> mjb\n\
                y02 OR x01 -> tnw\n\
                kwq OR kpj -> z05\n\
                x00 OR x03 -> fst\n\
                tgd XOR rvg -> z01\n\
                vdt OR tnw -> bfw\n\
                bfw AND frj -> z10\n\
                ffh OR nrd -> bqk\n\
                y00 AND y03 -> djm\n\
                y03 OR y00 -> psh\n\
                bqk OR frj -> z08\n\
                tnw OR fst -> frj\n\
                gnj AND tgd -> z11\n\
                bfw XOR mjb -> z00\n\
                x03 OR x00 -> vdt\n\
                gnj AND wpb -> z02\n\
                x04 AND y00 -> kjc\n\
                djm OR pbm -> qhw\n\
                nrd AND vdt -> hwm\n\
                kjc AND fst -> rvg\n\
                y04 OR y02 -> fgs\n\
                y01 AND x02 -> pbm\n\
                ntg OR kjc -> kwq\n\
                psh XOR fgs -> tgd\n\
                qhw XOR tgd -> z09\n\
                pbm OR djm -> kpj\n\
                x03 XOR y03 -> ffh\n\
                x00 XOR y04 -> ntg\n\
                bfw OR bqk -> z06\n\
                nrd XOR fgs -> wpb\n\
                frj XOR qhw -> z04\n\
                bqk OR frj -> z07\n\
                y03 OR x01 -> nrd\n\
                hwm AND bqk -> z03\n\
                tgd XOR rvg -> z12\n\
                tnw OR pbm -> gnj\
            ";

        assert_eq!(4, solve_1(sample_1));
        assert_eq!(2_024, solve_1(sample_2));
    }

    #[test]
    fn day_24_part_01_solution() {
        let input = include_str!("../../inputs/day_24.txt").trim();

        assert_eq!(36_035_961_805_936, solve_1(input));
    }

    #[ignore]
    #[test]
    fn day_24_part_02_sample() {
        let sample = "\
                x00: 1\n\
                x01: 1\n\
                x02: 1\n\
                y00: 0\n\
                y01: 1\n\
                y02: 0\n\
                \n\
                x00 AND y00 -> z00\n\
                x01 XOR y01 -> z01\n\
                x02 OR y02 -> z02\
            ";

        assert_eq!("z00,z01,z02,z05", solve_2(sample));
    }

    #[ignore]
    #[test]
    fn day_24_part_02_solution() {
        let input = include_str!("../../inputs/day_24.txt").trim();

        assert_eq!("", solve_2(input));
    }
}
