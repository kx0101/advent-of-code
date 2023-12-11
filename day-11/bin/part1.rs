pub mod share;

use crate::share::{galaxy_coordinates, parse};
use itertools::Itertools;

fn part1(input: &str) -> usize {
    let grid = parse(input);
    let galaxies = galaxy_coordinates(&grid, None);

    galaxies
        .iter()
        .combinations(2)
        .map(|pair| pair[0].calculate_distance(pair[1]))
        .sum()
}

fn main() {
    let input = include_str!("input.txt");
    println!("{}", part1(input));
}
