pub mod share;

use crate::share::Rule;

fn part1(input: &str) -> i64 {
    let (seeds_str, maps_str) = input.split_once("\n\n").unwrap();
    let seeds = seeds_str.strip_prefix("seeds: ").unwrap();
    let seeds = seeds.split_whitespace().map(|s| s.parse::<i64>().unwrap());

    let maps: Vec<Vec<Rule>> = maps_str
        .split("\n\n")
        .map(|block| {
            block
                .lines()
                .skip(1)
                .map(|line| {
                    let mut nums = line.splitn(3, ' ');

                    Rule {
                        destination: nums.next().unwrap().parse().unwrap(),
                        source: nums.next().unwrap().parse().unwrap(),
                        range: nums.next().unwrap().parse().unwrap(),
                    }
                })
                .collect()
        })
        .collect();

    seeds
        .map(|seed| {
            maps.iter().fold(seed, |curr, rules| {
                if let Some(rule) = rules
                    .iter()
                    .find(|rule| curr >= rule.source && curr <= rule.source + rule.range)
                {
                    let offset = curr - rule.source;
                    rule.destination + offset
                } else {
                    curr
                }
            })
        })
        .min()
        .unwrap()
}

fn main() {
    let input = include_str!("input.txt");
    println!("{}", part1(input));
}
