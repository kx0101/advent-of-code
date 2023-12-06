fn part1(input: &str) -> u64 {
    let (times, distances) = parse_race_data(input);
    let ways_to_win = calculate_ways(&times, &distances);

    println!("{}", ways_to_win);
    ways_to_win
}

fn parse_race_data(input: &str) -> (Vec<u64>, Vec<u64>) {
    let mut times = Vec::new();
    let mut distances = Vec::new();

    for line in input.lines() {
        let values: Vec<u64> = line
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        if line.starts_with("Time:") {
            times.extend(values);
        } else if line.starts_with("Distance:") {
            distances.extend(values);
        }
    }

    (times, distances)
}

fn calculate_ways(times: &[u64], distances: &[u64]) -> u64 {
    let mut ways_to_win = 1;

    for i in 0..times.len() {
        let record_distance = distances[i];
        let race_time = times[i];

        let mut ways_for_race = 0;

        for hold_time in 0..race_time {
            let total_distance = hold_time * (race_time - hold_time);

            if total_distance > record_distance {
                ways_for_race += 1;
            }
        }

        ways_to_win *= ways_for_race;
    }

    ways_to_win
}

fn main() {
    let input = include_str!("input.txt");
    part1(input);
}
