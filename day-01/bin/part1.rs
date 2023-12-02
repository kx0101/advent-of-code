fn main() {
    let file = include_str!("input.txt");

    println!("{}", part1(file));
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let first: u32 = line.chars().find_map(|c| c.to_digit(10)).unwrap();
            let second: u32 = line.chars().rev().find_map(|c| c.to_digit(10)).unwrap();

            (first * 10) + second
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
            ";

        assert_eq!(part1(input), 142);
    }
}
