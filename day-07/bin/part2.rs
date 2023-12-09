pub mod share;

use share::get_hands;

fn main() {
    let input = include_str!("input.txt");
    println!("{:#?}", get_hands(input));
}
