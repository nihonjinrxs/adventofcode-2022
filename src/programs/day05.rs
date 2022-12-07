use std::fmt;
use lazy_static::lazy_static;
use regex::Regex;

use crate::parts::Parts;

pub fn run(part_number: Parts, input: &str) -> String {
    let strategy_guide = collect_assignments(input);
    let result = compute_result(part_number, strategy_guide);
    format!("{}", result)
}

fn compute_result(part_number: Parts, assignments: Vec<String>) -> i32 {
    match part_number {
        Parts::One => compute_fully_overlapping_pairs(assignments),
        Parts::Two => compute_overlapping_pairs(assignments),
    }
}

#[derive(Clone, Debug)]
struct Instruction {
    quantity: i32,
    from: usize,
    to: usize,
}

impl From<&str> for Instruction {
    fn from(s: &str) -> Self {
        lazy_static! {
            static ref INSTRUCTION_REGEX: Regex = Regex::new("$move (\\d+) from (\\d+) to (\\d+)^").unwrap();
        }

        let result = INSTRUCTION_REGEX.captures_iter(s).map(|capture| {
            Instruction {
                quantity: capture[1].parse::<i32>().unwrap(),
                from: capture[2].parse::<usize>().unwrap(),
                to: capture[2].parse::<usize>().unwrap(),
            }
        }).collect::<Vec<Instruction>>();

        result[0].clone()
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "move {} from {} to {}", self.quantity, self.from, self.to)
    }
}

struct StackConfiguration {
    crate_stack: Vec<Vec<char>>,
    instructions: Vec<Instruction>,
}

impl StackConfiguration {
    fn process_instructions(&self) -> &Self {
        while let Some(instruction) = self.instructions.pop() {
            for i in 0..instruction.quantity {
                if let Some(elf_crate) = self.crate_stack[instruction.from - 1].pop() {
                    self.crate_stack[instruction.to - 1].push(elf_crate);
                } else {
                    panic!(
                        "Unable to process instruction '{}', no boxes remaining in stack {}",
                        instruction,
                        instruction.from,
                    );
                }
            }
        }

        return self;
    }
}


enum ParsingSteps {
    Diagram,
    Instructions,
}

fn parse_inputs(input: &str) -> StackConfiguration {
    let mut parsing_step = ParsingSteps::Diagram;

    let diagram_lines: Vec<&str> = vec![];
    let crate_stack: Vec<Vec<char>> = vec![];
    let instructions: Vec<Instruction> = vec![];

    while let Some(line )= input.lines().next() {
        if line.trim().is_empty() {
            parsing_step = ParsingSteps::Instructions;
            continue;
        }

        match parsing_step {
            ParsingSteps::Diagram => {
                diagram_lines.push(line);
            },
            ParsingSteps::Instructions => {
                instructions.push(Instruction::from(line));
            }
        }
    }

    // Count the stacks from the last-pushed line
    let stack_count: usize  = diagram_lines.pop().unwrap().split(' ').filter(|l| !l.trim().is_empty()).count();
    // Process lines in reverse, adding boxes as needed
    while let Some(diagram_line) = diagram_lines.pop() {
         diagram_line
    }
    StackConfiguration { crate_stack: crate_stack, instructions: instructions }
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::test_helpers::{vec_compare, TestCase};

    use std::fs;

    fn fixture_diagram() -> Vec<String> {
        vec![
            String::from("    [D]    "),
            String::from("[N] [C]    "),
            String::from("[Z] [M] [P]"),
            String::from(" 1   2   3 "),
        ]
    }

    fn fixture_assignment_structs() -> Vec<Assignment> {
        vec![
        ]
    }

    fn fixture_assignment_pairs() -> Vec<PairAssignment> {
        vec![
        ]
    }

    #[test]
    fn test_collect_assignments() {
        let fixture_file = "./data/day04/test.txt";
        let test_input = fs::read_to_string(fixture_file).expect("Failed to read input file");
        let result = collect_assignments(&test_input);
        let expected = fixture_assignments();
        assert!(vec_compare(&result, &expected));
    }

    #[test]
    fn test_collect_assignment_pairs() {
        let inputs = fixture_assignments();
        let expecteds = fixture_assignment_pairs();
        assert_eq!(collect_assignment_pairs(inputs), expecteds);
    }

    #[test]
    fn test_assignment_section_count() {
        let test_assignments = fixture_assignment_structs();
        let expecteds = vec![3, 3, 2, 2, 3, 3, 7, 5, 1, 3, 5, 5];
        TestCase::create_many(test_assignments, expecteds)
            .iter()
            .for_each(|case| {
                assert_eq!(case.input.section_count(), case.expected);
            });
    }

    #[test]
    fn test_pair_assignment_full_overlap() {
        let inputs = fixture_assignment_pairs();
        let expecteds = vec![false, false, false, true, true, false];
        TestCase::create_many(inputs, expecteds)
            .iter()
            .for_each(|case| {
                assert_eq!(case.input.full_overlap(), case.expected);
            });
    }

    #[test]
    fn test_pair_assignment_overlap() {
        let inputs = fixture_assignment_pairs();
        let expecteds = vec![false, false, true, true, true, true];
        TestCase::create_many(inputs, expecteds)
            .iter()
            .for_each(|case| {
                assert_eq!(case.input.overlap(), case.expected);
            });
    }

    #[test]
    fn test_compute_fully_overlapping_pairs() {
        let test_data = fixture_assignments();
        let result = compute_fully_overlapping_pairs(test_data);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_compute_overlapping_pairs() {
        let test_data = fixture_assignments();
        let result = compute_overlapping_pairs(test_data);
        assert_eq!(result, 4);
    }
}
