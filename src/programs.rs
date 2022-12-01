pub mod day1a;

pub fn run_program(program_name: &str, input: &str) -> &'static str {
    match program_name {
        "1a" => day1a::run(input),
        _ => "Unknown program"
    }
}
