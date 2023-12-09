use std::collections::HashMap;

struct Instruction {
    left: String,
    right: String,
}

fn parse_map(map_str: &str) -> HashMap<String, Instruction> {
    map_str
        .lines()
        .map(|l| {
            let (pos_str, dsts_str) = l.split_once('=').unwrap();
            let pos_str = pos_str.trim();
            let (l, r) = dsts_str
                .trim_matches(|c| "() ".contains(c))
                .split_once(',')
                .unwrap();

            (
                pos_str.to_string(),
                Instruction {
                    left: l.trim().to_string(),
                    right: r.trim().to_string(),
                },
            )
        })
        .collect()
}

fn part1(instructions: &str, map: &HashMap<String, Instruction>) -> u64 {
    let mut pos = "AAA";
    for (steps, c) in instructions.chars().cycle().enumerate() {
        let Instruction { left: l, right: r } = map.get(pos).unwrap();
        match c {
            'L' => pos = l,
            'R' => pos = r,
            _ => unreachable!(),
        }
        if pos == "ZZZ" {
            return steps as u64 + 1;
        }
    }
    unreachable!()
}

fn greater_common_divsior(a: u64, b: u64) -> u64 {
    if a == 0 {
        b
    } else {
        greater_common_divsior(b % a, a)
    }
}

fn least_common_multiple(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 {
        0
    } else {
        (a * b) / greater_common_divsior(a, b)
    }
}

fn part2(instructions: &str, map: &HashMap<String, Instruction>) -> u64 {
    let mut poses: Vec<_> = map.keys().filter(|p| p.ends_with('A')).cloned().collect();
    let mut cycles = vec![];

    for p in &mut poses {
        for (steps, c) in instructions.chars().cycle().enumerate() {
            let Instruction { left: l, right: r } = map.get(p).unwrap();

            match c {
                'L' => *p = l.to_string(),
                'R' => *p = r.to_string(),
                _ => unreachable!(),
            }

            if p.ends_with('Z') {
                cycles.push(steps as u64 + 1);
                break;
            }
        }
    }

    cycles
        .iter()
        .fold(1, |acc, c| least_common_multiple(*c, acc))
}

fn main() {
    let input = include_str!("input.txt");
    let (instructions, map_str) = input.split_once("\n\n").unwrap();
    let map = parse_map(map_str);

    println!("Part 1: {}", part1(instructions, &map));
    println!("Part 2: {}", part2(instructions, &map));
}
