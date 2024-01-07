use std::fs;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;

fn main() {
    let input = load_input(String::from("./input/day4.txt"));
    let cards_worth = day4::get_all_cards_worth(&input);
    println!("The worth of the cards: {cards_worth}");
}

fn load_input(input_path: String) -> String {
    fs::read_to_string(input_path).expect("Could not read file!")
}
