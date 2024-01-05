use std::fs;

pub mod day1;
pub mod day2;
pub mod day3;

fn main() {
    let input = load_input(String::from("./input/day3.txt"));
    let engine = day3::Engine::from(&input);

    let engine_number = engine.get_engine_num();
    println!("{engine_number}");
}

fn load_input(input_path: String) -> String {
    fs::read_to_string(input_path).expect("Could not read file!")
}

