pub fn get_matched_cards(card: &str) -> usize {
    let (winner, hand) = card.split_once(':').unwrap().1.split_once('|').unwrap();

    let winner = winner.split_whitespace().collect::<Vec<_>>();

    hand.split_whitespace()
        .filter(|card| winner.contains(card))
        .count()
}

fn main() {}
