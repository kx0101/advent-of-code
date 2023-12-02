fn main() {
    let input = include_str!("input.txt");
    part1(input);
}

fn part1(input: &str) -> u32 {
    let mut possible_sum = 0;
    let target_colors = (12, 13, 14);

    for game_line in input.lines() {
        let mut game_iter = game_line.split(':');
        let game_id: u32 = game_iter
            .next()
            .and_then(|id| id.split(' ').nth(1))
            .unwrap()
            .parse()
            .unwrap();

        let mut hands = game_iter.next().unwrap_or("").split(';');

        let is_game_possible = hands.all(|hand| {
            let hand_colors = get_colors(hand);

            hand_colors.0 <= target_colors.0
                && hand_colors.1 <= target_colors.1
                && hand_colors.2 <= target_colors.2
        });

        if is_game_possible {
            possible_sum += game_id;
        }
    }

    possible_sum
}

fn get_colors(hand: &str) -> (u8, u8, u8) {
    let mut colors: (u8, u8, u8) = (0, 0, 0);

    for color_part in hand.split(',') {
        let color_data: Vec<&str> = color_part.split(' ').collect();

        if let [_, color_count, color_name] = &color_data[..] {
            match *color_name {
                "red" => colors.0 = color_count.parse().unwrap_or(0),
                "green" => colors.1 = color_count.parse().unwrap_or(0),
                "blue" => colors.2 = color_count.parse().unwrap_or(0),
                _ => {}
            }
        }
    }

    colors
}
