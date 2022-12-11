use std::{str::Lines};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    combinator::{all_consuming, map},
    sequence::{preceded},
    Finish, IResult,
};

use crate::parts::Parts;

pub fn run(part_number: Parts, input: &str) -> String {
    let result = compute_result(part_number, input.lines());
    format!("{}", result)
}

fn compute_result(part_number: Parts, instructions: Lines) -> i32 {
    match part_number {
        Parts::One => execute(instructions),
        Parts::Two => execute(instructions),
    }
}

#[derive(Debug)]
struct Noop;

#[derive(Debug)]
struct Addx(i32);

fn parse_noop(instruction: &str) -> IResult<&str, Noop> {
    map(tag("noop"), |_| Noop)(instruction)
}

fn parse_int(s: &str) -> i32 {
    s.parse::<i32>().unwrap()
}

fn parse_value(instruction_value: &str) -> IResult<&str, i32> {
    map(
        take_while1(|c| "-0123456789".contains(c)),
        parse_int
    )(instruction_value)
}

fn parse_addx(instruction: &str) -> IResult<&str, Addx> {
    map(preceded(tag("addx "), parse_value), Addx)(instruction)
}

#[derive(Debug)]
enum Instruction {
    Noop(Noop),
    Addx(Addx),
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    alt((
        map(parse_noop, Instruction::Noop),
        map(parse_addx, Instruction::Addx),
    ))(input)
}

fn parse_instructions(input_lines: Lines) -> Vec<Instruction> {
    input_lines
        .map(|instr|
            all_consuming(parse_instruction)(instr)
                .finish().unwrap().1
        ).collect::<Vec<Instruction>>()
}

#[derive(Debug)]
struct Machine {
    register_x: i32,
    cycle_number: u64,
}

impl Machine {
    fn process_instruction(&self, instruction: Instruction) -> Self {
        match instruction {
            Instruction::Noop(Noop) =>
                Machine {
                    register_x: self.register_x,
                    cycle_number: self.cycle_number + 1,
                },
            Instruction::Addx(Addx(val)) =>
                Machine {
                    register_x: self.register_x + val,
                    cycle_number: self.cycle_number + 2,
                },
        }
    }
}

fn execute(input_lines: Lines) -> i32 {
    let instructions = parse_instructions(input_lines);
    let mut machine = Machine { register_x: 1, cycle_number: 0 };
    for instruction in instructions {
        machine = machine.process_instruction(instruction)
    }
    println!("{:?}", machine);
    machine.register_x
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::test_helpers::{vec_compare, TestCase};

    use std::fs;

    fn fixture_short_input() -> &'static str {
        "noop\naddx 3\naddx -5"
    }

    #[test]
    fn test_short_input_final_value() {
        let instructions = fixture_short_input().lines();
        assert_eq!(execute(instructions), -1);
    }
}
