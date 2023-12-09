use itertools::{Itertools, Position};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
pub enum HandType {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

fn calculate_values(counts: &HashMap<char, usize>) -> String {
    // for part 1, we would just ignore the joker_count logic and only do the last line of this
    // function
    if let Some(joker_count) = counts.get(&'J') {
        if *joker_count == 5 {
            return "5".to_string();
        } else {
            return counts
                .iter()
                .filter_map(|(key, value)| (key != &'J').then_some(value))
                .sorted()
                .with_position()
                .map(|(position, value)| match position {
                    Position::Last | Position::Only => value + joker_count,
                    _ => *value,
                })
                .join("");
        }
    }
    counts.values().sorted().join("")
}

fn determine_hand_type(values: &str) -> Result<HandType, &'static str> {
    match values {
        "5" => Ok(HandType::FiveOfAKind),
        "14" => Ok(HandType::FourOfAKind),
        "23" => Ok(HandType::FullHouse),
        "113" => Ok(HandType::ThreeOfAKind),
        "122" => Ok(HandType::TwoPair),
        "1112" => Ok(HandType::OnePair),
        "11111" => Ok(HandType::HighCard),
        _ => Err("Unexpected value encountered"),
    }
}

fn map_card_values(card: char) -> u32 {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 1,
        'T' => 10,
        value => value.to_digit(10).unwrap(),
    }
}

fn get_card_scores(hand: &str) -> (u32, u32, u32, u32, u32) {
    hand.chars().map(map_card_values).collect_tuple().unwrap()
}

pub fn get_score(hand: &str) -> Result<(HandType, (u32, u32, u32, u32, u32)), &'static str> {
    let counts = hand.chars().counts();
    let values = calculate_values(&counts);
    let hand_type = determine_hand_type(&values)?;

    Ok((hand_type, get_card_scores(hand)))
}

pub fn get_hands(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (hand, bid) = line.split_once(' ').unwrap();
            (hand, bid.parse::<u32>().unwrap(), get_score(hand).unwrap())
        })
        .sorted_by_key(|x| (x.2 .0 as u8, x.2 .1))
        .enumerate()
        .map(|(index, (_hand, bid, _))| (index as u32 + 1) * bid)
        .sum::<u32>()
}

fn main() {}
