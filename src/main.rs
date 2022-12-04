use adventofcode2022::programs;
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let program = &args[1];
    let input_file_path = &args[2];

    let input = fs::read_to_string(input_file_path).expect("Failed to read input file");

    println!("{}", programs::run_program(program, &input));
}
