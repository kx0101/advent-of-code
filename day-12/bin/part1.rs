pub mod share;

use crate::share::SpringCondition;
use crate::share::{count_arrangements, parse_input};
use std::collections::HashMap;

fn part1(input: &SpringCondition) -> usize {
    let mut visited = HashMap::new();

    input
        .iter()
        .map(|(record, groups)| {
            visited.clear();
            count_arrangements(record, groups, 0, &mut visited)
        })
        .sum()
}

fn main() {
    let input = include_str!("input.txt");
    let parsed_input = parse_input(input);

    println!("{}", part1(&parsed_input));
}
