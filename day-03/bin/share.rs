pub struct Coord {
    pub x: usize,
    pub y: usize,
    pub c: char,
}

pub struct Number {
    pub left_boundary: usize,
    pub right_boundary: usize,
    pub row: usize,
    pub value: String,
}

pub fn get_symbols(input: &str) -> Vec<Coord> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| {
                if c.is_numeric() || c == '.' {
                    None
                } else {
                    Some(Coord { x, y, c })
                }
            })
        })
        .collect()
}

pub fn get_numbers(input: &str) -> Vec<Number> {
    let mut numbers = Vec::new();

    input.lines().enumerate().for_each(|(y, line)| {
        let char_indices = line.char_indices();

        for (x, c) in char_indices {
            if !c.is_numeric() {
                continue;
            }

            if x > 0 && line.chars().nth(x - 1).is_some_and(|f| f.is_numeric()) {
                let last: &mut Number = numbers.last_mut().unwrap();
                last.right_boundary = x;
                last.value.push(c);
            } else {
                numbers.push(Number {
                    left_boundary: x,
                    right_boundary: x,
                    row: y,
                    value: c.to_string(),
                });
            }
        }
    });

    numbers
}

pub fn is_adjacent_to_symbol(number: &Number, symbol: &Coord) -> bool {
    let left_boundary = if number.left_boundary == 0 {
        0
    } else {
        number.left_boundary - 1
    };

    let top_boundary = if number.row == 0 { 0 } else { number.row - 1 };

    symbol.x >= left_boundary
        && symbol.x <= number.right_boundary + 1
        && symbol.y <= number.row + 1
        && symbol.y >= top_boundary
}

fn main() {}
