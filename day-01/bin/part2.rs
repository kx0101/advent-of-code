fn main() {
    let file_content = include_str!("input.txt");
    println!("{}", part2(file_content));
}

pub fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (first, last) = find_first_and_last_digit_positions(line);
            digit_to_number(first) * 10 + digit_to_number(last)
        })
        .sum()
}

fn find_first_and_last_digit_positions(line: &str) -> (&str, &str) {
    let mut first_position = line.len();
    let mut last_position = 0;

    let mut first_digit = "";
    let mut last_digit = "";

    for digit_text in &TEXT_DIGITS {
        if let Some(position) = line.find(digit_text) {
            if position < first_position {
                first_position = position;
                first_digit = digit_text;
            }
        }

        if let Some(position) = line.rfind(digit_text) {
            if position > last_position {
                last_position = position;
                last_digit = digit_text;
            }
        }
    }

    (first_digit, last_digit)
}

const TEXT_DIGITS: [&str; 18] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4",
    "5", "6", "7", "8", "9",
];

fn digit_to_number(digit: &str) -> u32 {
    match digit {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        n => n.parse().unwrap_or(0),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let input = "
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
";

        assert_eq!(part2(input), 281);
    }
}
