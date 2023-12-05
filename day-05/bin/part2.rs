pub mod share;

use itertools::Itertools;

use crate::share::Rule;

#[derive(Debug, Clone)]
struct Range {
    from: i64,
    to: i64,
}

pub fn part2(input: &str) -> i64 {
    let (seeds_str, maps_str) = input.split_once("\n\n").unwrap();
    let seeds = parse_seeds(seeds_str);
    let maps = parse_maps(maps_str);

    let mut curr_ranges = generate_ranges(seeds.into_iter());

    for map in &maps {
        curr_ranges = apply_rules(&curr_ranges, map);
    }

    curr_ranges.iter().map(|range| range.from).min().unwrap()
}

fn parse_seeds(seeds_str: &str) -> Vec<Range> {
    seeds_str
        .strip_prefix("seeds: ")
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .chunks(2)
        .into_iter()
        .map(|mut chunk| {
            let from = chunk.next().unwrap();
            let range = chunk.next().unwrap();
            Range {
                from,
                to: from + range,
            }
        })
        .collect()
}

fn parse_maps(maps_str: &str) -> Vec<Vec<Rule>> {
    maps_str
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
                .sorted_by(|a, b| a.source.cmp(&b.source))
                .collect()
        })
        .collect()
}

fn generate_ranges(seeds: impl Iterator<Item = Range>) -> Vec<Range> {
    seeds.collect()
}

fn apply_rules(curr_ranges: &[Range], map: &[Rule]) -> Vec<Range> {
    let mut new_ranges: Vec<Range> = Vec::new();

    for range in curr_ranges {
        let mut curr = range.clone();

        for rule in map {
            let offset = rule.destination - rule.source;
            let rule_applies = curr.from <= curr.to
                && curr.from <= rule.source + rule.range
                && curr.to >= rule.source;

            if rule_applies {
                if curr.from < rule.source {
                    new_ranges.push(Range {
                        from: curr.from,
                        to: rule.source - 1,
                    });

                    curr.from = rule.source;

                    if curr.to < rule.source + rule.range {
                        new_ranges.push(Range {
                            from: curr.from + offset,
                            to: curr.to + offset,
                        });

                        curr.from = curr.to + 1;
                    } else {
                        new_ranges.push(Range {
                            from: curr.from + offset,
                            to: rule.source + rule.range - 1 + offset,
                        });

                        curr.from = rule.source + rule.range;
                    }
                } else if curr.to < rule.source + rule.range {
                    new_ranges.push(Range {
                        from: curr.from + offset,
                        to: curr.to + offset,
                    });

                    curr.from = curr.to + 1;
                } else {
                    new_ranges.push(Range {
                        from: curr.from + offset,
                        to: rule.source + rule.range - 1 + offset,
                    });

                    curr.from = rule.source + rule.range;
                }
            }
        }

        if curr.from <= curr.to {
            new_ranges.push(curr);
        }
    }

    new_ranges
}

fn main() {
    let input = include_str!("input.txt");
    println!("{}", part2(input));
}
