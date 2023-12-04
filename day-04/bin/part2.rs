pub mod share;

use crate::share::get_matched_cards;

pub fn part2(input: &str) -> usize {
    let matched_card_counts = input.lines().map(get_matched_cards).collect::<Vec<_>>();
    let path = vec![0; matched_card_counts.len()];

    fn dfs(
        total_matched_cards: usize,
        index: usize,
        path: Vec<usize>,
        matched_card_counts: Vec<usize>,
    ) -> usize {
        match path.len() {
            0 => total_matched_cards,
            _ => {
                let matches_for_card = matched_card_counts[index];
                let card_matches = path[0] + 1;

                dfs(
                    total_matched_cards + card_matches,
                    index + 1,
                    path[1..]
                        .iter()
                        .enumerate()
                        .map(|(index, x)| match index {
                            n if n < matches_for_card => x + card_matches,
                            _ => *x,
                        })
                        .collect(),
                    matched_card_counts,
                )
            }
        }
    }

    dfs(0, 0, path, matched_card_counts)
}

fn main() {
    let input = include_str!("input.txt");
    println!("{}", part2(input));
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";

        assert_eq!(part2(input), 30);
    }
}
