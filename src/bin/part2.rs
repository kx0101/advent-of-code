fn main() {
    let input = include_str!("input.txt");
    part2(input);
}

fn part2(input: &str) -> u64 {
    let mut min_cubes_sum = 0;

    for game_line in input.lines() {
        let mut game_iter = game_line.split(':');
        let hands = game_iter.nth(1).unwrap_or("").split(';');

        let (mut min_red, mut min_green, mut min_blue) = (0, 0, 0);

        for hand in hands {
            let hand_colors = get_colors(hand);

            min_red = min_red.max(hand_colors.0);
            min_green = min_green.max(hand_colors.1);
            min_blue = min_blue.max(hand_colors.2);
        }

        min_cubes_sum += u64::from(min_red) * u64::from(min_green) * u64::from(min_blue);
    }

    println!("{}", min_cubes_sum);
    min_cubes_sum
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
