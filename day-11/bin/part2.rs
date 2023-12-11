pub mod share;

use crate::share::{galaxy_coordinates, parse};
use itertools::Itertools;

fn part2(input: &str) -> usize {
    let grid = parse(input);
    let galaxies = galaxy_coordinates(&grid, Some(1_000_000));

    galaxies
        .iter()
        .combinations(2)
        .map(|pair| pair[0].calculate_distance(pair[1]))
        .sum()
}

fn main() {
    let input = include_str!("input.txt");
    println!("{}", part2(input));
}
