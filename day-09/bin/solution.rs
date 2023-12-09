fn parse_input(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse().unwrap())
                // .rev() -- part2
                .collect()
        })
        .collect()
}

fn next_value(history: &[i64]) -> i64 {
    let mut matrix = vec![history.to_vec()];
    let mut index = 0;

    while !matrix[index].iter().all(|&x| x == 0) {
        let mut next_arr = Vec::new();

        for window in matrix[index].windows(2) {
            next_arr.push(window[1] - window[0]);
        }

        matrix.push(next_arr);
        index += 1;
    }

    let mut last_val = 0;
    for values in matrix.iter_mut().rev().skip(1) {
        last_val += *values.last().unwrap();
        values.push(last_val);
    }

    *matrix[0].last().unwrap()
}

fn part1(input: &str) -> i64 {
    let histories = parse_input(input);
    let extrapolated_values: Vec<i64> = histories
        .iter()
        .map(|history| next_value(history))
        .collect();

    extrapolated_values.iter().sum()
}

fn main() {
    let input = include_str!("input.txt");
    let sum = part1(input);
    println!("{sum}");
}
