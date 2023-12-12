use std::collections::HashMap;

pub fn count_arrangements(
    record: &str,
    groups: &[usize],
    run_length: usize,
    visited: &mut HashMap<(usize, usize, usize), usize>,
) -> usize {
    if let Some(&result) = visited.get(&(record.len(), groups.len(), run_length)) {
        return result;
    }

    if record.is_empty() {
        if (run_length == 0 && groups.is_empty()) || groups == [run_length] {
            return 1;
        }

        return 0;
    }

    let mut total_arrangements = 0;

    for current_condition in ["#", "."] {
        if ![current_condition, "?"].contains(&&record[0..1]) {
            continue;
        }

        if current_condition == "."
            && run_length > 0
            && !groups.is_empty()
            && groups[0] == run_length
        {
            total_arrangements += count_arrangements(&record[1..], &groups[1..], 0, visited);
        } else if current_condition == "." && run_length == 0 {
            total_arrangements += count_arrangements(&record[1..], groups, run_length, visited);
        } else if current_condition == "#" {
            total_arrangements += count_arrangements(&record[1..], groups, run_length + 1, visited);
        }
    }

    visited.insert((record.len(), groups.len(), run_length), total_arrangements);

    total_arrangements
}

pub fn parse_input(input: &str) -> SpringCondition {
    input
        .trim()
        .lines()
        .map(|line| {
            let (record, groups) = line.split_once(' ').unwrap();

            (
                record,
                groups.split(',').map(|x| x.parse().unwrap()).collect(),
            )
        })
        .collect()
}

pub type SpringCondition<'a> = Vec<(&'a str, Vec<usize>)>;

fn main() {}
