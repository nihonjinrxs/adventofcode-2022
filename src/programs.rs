pub mod day01;
pub mod day02;

use crate::parts::Parts;

pub fn run_program(program_name: &str, input: &str) -> String {
    match program_name {
        "01a" | "1a" => day01::run(Parts::One, &input).to_owned(),
        "01b" | "1b" => day01::run(Parts::Two, &input).to_owned(),
        "02a" | "2a" => day02::run(Parts::One, &input).to_owned(),
        "02b" | "2b" => day02::run(Parts::Two, &input).to_owned(),
        _ => String::from("Unknown program")
    }
}
