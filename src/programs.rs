pub mod day1a;

pub fn run_program(program_name: &str, input: &str) -> String {
    match program_name {
        "1a" => day1a::run(&input).to_owned(),
        _ => String::from("Unknown program")
    }
}
