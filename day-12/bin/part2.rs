pub mod share;

use crate::share::{count_arrangements, parse_input};
use crate::share::SpringCondition;
use std::{collections::HashMap, iter::repeat};

pub fn part2(input: &SpringCondition) -> usize {
    let mut visited = HashMap::new();

    input
        .iter()
        .map(|(record, groups)| {
            let unfolded_record = repeat((*record).to_string())
                .take(5)
                .collect::<Vec<_>>()
                .join("?");

            let unfolded_groups: Vec<_> = repeat(groups).take(5).flatten().copied().collect();

            visited.clear();
            count_arrangements(&unfolded_record, &unfolded_groups, 0, &mut visited)
        })
        .sum()
}

fn main() {
    let input = include_str!("input.txt");
    let parsed_input = parse_input(input);

    println!("{}", part2(&parsed_input));
}
