use crate::parts::Parts;

pub fn run(part_number: Parts, input: &str) -> String {
    let rucksacks = collect_rucksacks(input).to_owned();
    let result = compute_result(part_number, rucksacks);
    format!("{}", result)
}

fn compute_result(part_number: Parts, compartments: Vec<(String, String)>) -> i32 {
    match part_number {
        Parts::One => compute_priority_sum(compartments),
        Parts::Two => compute_priority_sum(compartments),
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

fn compute_priority_sum(compartments: Vec<(String, String)>) -> i32 {
    let misplaced = find_misplaced(compartments);
    misplaced.iter().map(|c| priority_for_char(*c)).sum()
}

fn get_shared(rucksack: &(String, String)) -> char {
    let compartment_one = rucksack.0.chars();
    let filtered = compartment_one.filter(|c| 
        rucksack.1.as_str().contains(*c)).collect::<Vec<char>>();
    filtered[0]
}

fn find_misplaced(compartments: Vec<(String, String)>) -> Vec<char> {
    compartments.iter().map(get_shared).collect::<Vec<char>>()
}

fn collect_rucksacks(input: &str) -> Vec<(String, String)> {
    let mut rucksacks: Vec<(String, String)> = vec![];
    for l in input.split('\n') {
        if l.trim().is_empty() { continue }
        let trimmed = String::from(l.trim());
        let half = trimmed.len() / 2;
        let compartments = trimmed.split_at(half);
        rucksacks.push((
            String::from(compartments.0),
            String::from(compartments.1),
        ));
    }
    rucksacks
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::test_helpers::vec_compare;

    use std::fs;

    #[test]
    fn test_collect_rucksacks() {
        let fixture_file = "./data/day03/test.txt";
        let test_input = fs::read_to_string(fixture_file)
            .expect("Failed to read input file");
        let rucksacks = collect_rucksacks(&test_input);
        let result = rucksacks.iter()
                .map(|(a, b)| (a.as_str(), b.as_str()))
                .collect::<Vec<(&str, &str)>>();
        let expected = vec![
            ("vJrwpWtwJgWr", "hcsFMMfFFhFp"),
            ("jqHRNqRjqzjGDLGL", "rsFMfFZSrLrFZsSL"),
            ("PmmdzqPrV", "vPwwTWBwg"),
            ("wMqvLMZHhHMvwLH", "jbvcjnnSBnvTQFn"),
            ("ttgJtRGJ", "QctTZtZT"),
            ("CrZsJsPPZsGz", "wwsLwLmpwMDw"),
        ];
        assert!(vec_compare(&result, &expected));
    }

    struct TestCase<TInput, TExpected> {
        input: TInput,
        expected: TExpected,
    }

    fn cases() -> Vec<TestCase<(String, String), char>> {
        vec![
            TestCase {
                input: (
                    String::from("vJrwpWtwJgWr"),
                    String::from("hcsFMMfFFhFp"),
                ),
                expected: 'p',
            },
            TestCase {
                input: (
                    String::from("jqHRNqRjqzjGDLGL"),
                    String::from("rsFMfFZSrLrFZsSL"),
                ),
                expected: 'L',
            },
            TestCase {
                input: (
                    String::from("PmmdzqPrV"),
                    String::from("vPwwTWBwg"),
                ),
                expected: 'P',
            },
            TestCase {
                input: (
                    String::from("wMqvLMZHhHMvwLH"),
                    String::from("jbvcjnnSBnvTQFn"),
                ),
                expected: 'v',
            },
            TestCase {
                input: (
                    String::from("ttgJtRGJ"),
                    String::from("QctTZtZT"),
                ),
                expected: 't',
            },
            TestCase {
                input: (
                    String::from("CrZsJsPPZsGz"),
                    String::from("wwsLwLmpwMDw"),
                ),
                expected: 's',
            },
        ]
    }

    #[test]
    fn test_get_shared() {
        cases().iter().for_each(|case|
            assert_eq!(get_shared(&case.input), case.expected)
        );
    }

    #[test]
    fn test_find_misplaced() {
        let cases = cases();
        let inputs = cases.iter().map(|case|
            (case.input.0.clone(), case.input.1.clone())
        ).collect::<Vec<(String, String)>>();
        let results = find_misplaced(inputs);
        let expected = cases.iter().map(|case| case.expected).collect::<Vec<char>>();
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
    fn test_compute_priority_sum() {
        let cases = cases();
        let inputs = cases.iter().map(|case|
            (case.input.0.clone(), case.input.1.clone())
        ).collect::<Vec<(String, String)>>();
        let result = compute_priority_sum(inputs);
        assert_eq!(result, 157);
    }

    // #[test]
    // fn test_outcome_strategy() {
    //     let test_data = vec![("A", "Y"), ("B", "X"), ("C", "Z")];
    //     let result = compute_all_turns_score(test_data, RPSStrategy::Outcome);
    //     assert_eq!(result, 12);
    // }
}
