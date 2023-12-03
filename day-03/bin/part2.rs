pub mod share;

use share::{Coord, Number};

use crate::share::{get_numbers, get_symbols, is_adjacent_to_symbol};

pub fn part2(input: &str) -> i32 {
    let symbols = get_symbols(input);
    let numbers = get_numbers(input);

    calculate_gear_sums(&symbols, &numbers)
}

fn calculate_gear_sums(symbols: &[Coord], numbers: &[Number]) -> i32 {
    symbols
        .iter()
        .filter_map(|s| {
            if s.c != '*' {
                None
            } else {
                let filter_map =
                    numbers
                        .iter()
                        .filter_map(|number| match is_adjacent_to_symbol(number, s) {
                            true => Some(number.value.parse::<i32>().unwrap()),
                            false => None,
                        });

                if filter_map.clone().count() == 2 {
                    Some(filter_map.reduce(|a, b| a * b).unwrap())
                } else {
                    None
                }
            }
        })
        .sum()
}

fn main() {
    let input = include_str!("input.txt");
    part2(input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_2() {
        let input = "
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";

        assert_eq!(part2(input), 467835);
    }
}
