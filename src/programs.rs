pub mod day01a;

pub fn run_program(program_name: &str, input: &str) -> String {
    match program_name {
        "01a" | "1a" => day01a::run(&input).to_owned(),
        _ => String::from("Unknown program")
    }
}
