pub mod share;

use crate::share::{get_numbers, get_symbols, is_adjacent_to_symbol};

pub fn part1(input: &str) -> i32 {
    let symbols = get_symbols(input);
    let numbers = get_numbers(input);

    let used_numbers = numbers.iter().filter(|number| {
        symbols
            .iter()
            .any(|symbol| is_adjacent_to_symbol(number, symbol))
    });

    used_numbers.map(|k| k.value.parse::<i32>().unwrap()).sum()
}

fn main() {
    let input = include_str!("input.txt");
    part1(input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
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

        assert_eq!(part1(input), 4361);
    }
}
