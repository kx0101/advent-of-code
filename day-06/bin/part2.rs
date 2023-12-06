fn part2(input: &str) -> u64 {
    let mut race_data = Vec::new();

    for line in input.lines() {
        let values: Vec<u64> = line
            .split_whitespace()
            .flat_map(|s| s.chars().filter_map(|c| c.to_digit(10).map(|d| d as u64)))
            .collect();

        race_data.push(values);
    }

    let ways_to_win = calculate_ways(&race_data);

    println!("{}", ways_to_win);
    ways_to_win
}

fn calculate_ways(race_data: &[Vec<u64>]) -> u64 {
    let mut ways_to_win = 0;

    if race_data.len() == 2 {
        let times = &race_data[0];
        let distances = &race_data[1];

        let race_time = times.iter().fold(0, |acc, &x| acc * 10 + x);
        let record_distance = distances.iter().fold(0, |acc, &x| acc * 10 + x);

        for hold_time in 14..=race_time - 14 {
            let remaining_time = race_time - hold_time;
            let total_distance = hold_time * remaining_time;

            if total_distance > record_distance {
                ways_to_win += 1;
            }
        }
    }

    ways_to_win
}

fn main() {
    let input = include_str!("input.txt");
    part2(input);
}
