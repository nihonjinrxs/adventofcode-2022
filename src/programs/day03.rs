use crate::parts::Parts;

pub fn run(part_number: Parts, input: &str) -> String {
    let rucksacks = collect_rucksacks(input).to_owned();
    let result = compute_result(part_number, rucksacks);
    format!("{}", result)
}

fn compute_result(part_number: Parts, rucksacks: Vec<String>) -> i32 {
    match part_number {
        Parts::One => compute_misplaced_priority_sum(rucksacks),
        Parts::Two => compute_badge_priority_sum(rucksacks),
    }
}

fn priority_for_char(c: char) -> i32 {
    let ref_cap_a = 'A' as i32;
    let ref_a = 'a' as i32;
    let char_ord = c as i32;
    if char_ord < ref_a {
        char_ord - ref_cap_a + 27
    } else {
        char_ord - ref_a + 1
    }
}

fn compute_misplaced_priority_sum(rucksacks: Vec<String>) -> i32 {
    let compartments = split_compartments(rucksacks);
    let misplaced = find_misplaced(compartments);
    misplaced.iter().map(|c| priority_for_char(*c)).sum()
}

fn compute_badge_priority_sum(rucksacks: Vec<String>) -> i32 {
    rucksacks.len() as i32
}

fn get_shared_in_rucksack(rucksack: &(String, String)) -> char {
    let compartment_one = rucksack.0.chars();
    let filtered = compartment_one.filter(|c| 
        rucksack.1.as_str().contains(*c)).collect::<Vec<char>>();
    filtered[0]
}

fn find_misplaced(compartments: Vec<(String, String)>) -> Vec<char> {
    compartments.iter().map(get_shared_in_rucksack).collect::<Vec<char>>()
}

fn split_compartments(rucksacks: Vec<String>) -> Vec<(String, String)> {
    let mut rucksack_compartments: Vec<(String, String)> = vec![];
    rucksacks.iter().for_each(|rucksack| {
        let half = rucksack.len() / 2;
        let compartments = rucksack.split_at(half);
        rucksack_compartments.push((
            String::from(compartments.0),
            String::from(compartments.1),
        ));
    });
    rucksack_compartments
}

fn collect_rucksacks(input: &str) -> Vec<String> {
    let mut rucksacks: Vec<String> = vec![];
    for l in input.split('\n') {
        if l.trim().is_empty() { continue }
        let trimmed = String::from(l.trim());
        rucksacks.push(trimmed);
    }
    rucksacks
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::test_helpers::{
        vec_compare,
        TestCase,
    };

    use std::fs;

    fn fixture_rucksacks() -> Vec<String> {
        vec![
            String::from("vJrwpWtwJgWrhcsFMMfFFhFp"),
            String::from("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"),
            String::from("PmmdzqPrVvPwwTWBwg"),
            String::from("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"),
            String::from("ttgJtRGJQctTZtZT"),
            String::from("CrZsJsPPZsGzwwsLwLmpwMDw"),
        ]
    }

    fn fixture_compartments() -> Vec<(String, String)> {
        vec![
            (
                String::from("vJrwpWtwJgWr"),
                String::from("hcsFMMfFFhFp"),
            ),
            (
                String::from("jqHRNqRjqzjGDLGL"),
                String::from("rsFMfFZSrLrFZsSL"),
            ),
            (
                String::from("PmmdzqPrV"),
                String::from("vPwwTWBwg"),
            ),
            (
                String::from("wMqvLMZHhHMvwLH"),
                String::from("jbvcjnnSBnvTQFn"),
            ),
            (
                String::from("ttgJtRGJ"),
                String::from("QctTZtZT"),
            ),
            (
                String::from("CrZsJsPPZsGz"),
                String::from("wwsLwLmpwMDw"),
            ),
        ]
    }

    fn fixture_missing() -> Vec<char> {
        vec!['p', 'L', 'P', 'v', 't', 's']
    }

    #[test]
    fn test_collect_rucksacks() {
        let fixture_file = "./data/day03/test.txt";
        let test_input = fs::read_to_string(fixture_file)
            .expect("Failed to read input file");
        let result = collect_rucksacks(&test_input);
        let expected = fixture_rucksacks();
        assert!(vec_compare(&result, &expected));
    }

    #[test]
    fn test_split_compartments() {
        let rucksacks = fixture_rucksacks();
        let expected = fixture_compartments();
        let result = split_compartments(rucksacks);
        assert!(vec_compare(&result, &expected));
    }

    #[test]
    fn test_get_shared_in_rucksack() {
        let inputs = fixture_compartments();
        let expecteds = fixture_missing();
        TestCase::create_many(inputs, expecteds).iter().for_each(|case|
            assert_eq!(get_shared_in_rucksack(&case.input), case.expected)
        );
    }

    #[test]
    fn test_find_misplaced() {
        let inputs = fixture_compartments();
        let expected = fixture_missing();
        let results = find_misplaced(inputs);
        assert!(vec_compare(&results, &expected));
    }

    #[test]
    fn test_priority_for_char() {
        let cases = vec![
            TestCase{ input: 'a', expected: 1i32 },
            TestCase{ input: 'z', expected: 26 },
            TestCase{ input: 'A', expected: 27 },
            TestCase{ input: 'Z', expected: 52 },
        ];
        cases.iter().for_each(|case| {
            assert_eq!(priority_for_char(case.input), case.expected)
        });
    }

    #[test]
    fn test_compute_misplaced_priority_sum() {
        let inputs = fixture_rucksacks();
        let result = compute_misplaced_priority_sum(inputs);
        assert_eq!(result, 157);
    }

    // #[test]
    // fn test_outcome_strategy() {
    //     let test_data = vec![("A", "Y"), ("B", "X"), ("C", "Z")];
    //     let result = compute_all_turns_score(test_data, RPSStrategy::Outcome);
    //     assert_eq!(result, 12);
    // }
}
