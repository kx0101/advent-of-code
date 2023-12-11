pub mod share;

use crate::share::{parse, build_loop};

fn part1(input: &str) -> usize {
    let (map, start) = parse(input);
    let loop_coords = build_loop(start, &map);

    loop_coords.len() / 2
}

fn main() {
    let input = include_str!("input.txt");
    println!("{}", part1(input));
}
