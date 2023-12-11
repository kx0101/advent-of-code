pub mod share;

use crate::share::Tile::{Ground, NorthEast, NorthSouth, NorthWest};
use crate::share::{build_loop, clean_map, parse};

pub fn part2(input: &str) -> usize {
    let (map, start) = parse(input);
    let loop_coords = build_loop(start, &map);
    let map = clean_map(start, &loop_coords, map);

    // scan from top to bottom and left to right, counting how many tiles are inside the loop.
    // keep track of a boolean that tells me if I'm inside the loop
    // every time I cross a vertical pipe that does not horizontally block the top (the place where I am in the loop), flip that state
    
    let mut inside = false;
    map.into_iter()
        .flatten()
        .filter(|tile| match tile {
            Ground => inside,
            NorthSouth | NorthWest | NorthEast => {
                inside = !inside;
                false
            }
            _ => false,
        })
        .count()
}

fn main() {
    let input = include_str!("input.txt");
    println!("{}", part2(input));
}
