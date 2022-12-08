pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day06;
pub mod day07;

use crate::parts::Parts;

pub fn run_program(program_name: &str, input: &str) -> String {
    match program_name {
        "01a" | "1a" => day01::run(Parts::One, &input).to_owned(),
        "01b" | "1b" => day01::run(Parts::Two, &input).to_owned(),
        "02a" | "2a" => day02::run(Parts::One, &input).to_owned(),
        "02b" | "2b" => day02::run(Parts::Two, &input).to_owned(),
        "03a" | "3a" => day03::run(Parts::One, &input).to_owned(),
        "03b" | "3b" => day03::run(Parts::Two, &input).to_owned(),
        "04a" | "4a" => day04::run(Parts::One, &input).to_owned(),
        "04b" | "4b" => day04::run(Parts::Two, &input).to_owned(),
        "06a" | "6a" => day06::run(Parts::One, &input).to_owned(),
        "06b" | "6b" => day06::run(Parts::Two, &input).to_owned(),
        "07a" | "7a" => day07::run(Parts::One, &input).to_owned(),
        "07b" | "7b" => day07::run(Parts::Two, &input).to_owned(),
        _ => String::from("Unknown program"),
    }
}
