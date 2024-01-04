use std::fs;

pub mod day1;
pub mod day2;

fn main() {
    let input = load_input(String::from("./input/day2.txt"));

    let games = day2::get_games(&input);
    let sum_min_hands_value = day2::sum_min_hands_value(&games);

    println!("{sum_min_hands_value}");

}

fn load_input(input_path: String) -> String {
    fs::read_to_string(input_path).expect("Could not read file!")
}

