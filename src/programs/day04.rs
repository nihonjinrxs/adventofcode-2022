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

fn compute_fully_overlapping_pairs(assignments: Vec<String>) -> i32 {
    let pairs = collect_assignment_pairs(assignments);
    pairs.iter().filter(|pair| pair.full_overlap()).count() as i32
}

fn compute_overlapping_pairs(assignments: Vec<String>) -> i32 {
    let pairs = collect_assignment_pairs(assignments);
    pairs.iter().filter(|pair| pair.overlap()).count() as i32
}

#[derive(Clone, Copy, PartialEq, Debug)]
struct Assignment {
    start_section: i32,
    end_section: i32,
}

impl Assignment {
    fn section_count(&self) -> i32 {
        self.end_section - self.start_section + 1
    }
}

type PairAssignment = Vec<Assignment>;

trait Overlap {
    fn full_overlap(&self) -> bool;
    fn overlap(&self) -> bool;
}

impl Overlap for PairAssignment {
    fn full_overlap(&self) -> bool {
        let elf1: Assignment;
        let elf2: Assignment;
        if self[0].section_count() >= self[1].section_count() {
            elf1 = self[0];
            elf2 = self[1];
        } else {
            elf2 = self[0];
            elf1 = self[1];
        }

        elf1.start_section <= elf2.start_section && elf1.end_section >= elf2.end_section
    }

    fn overlap(&self) -> bool {
        if self.full_overlap() {
            return true;
        }

        let elf1: Assignment;
        let elf2: Assignment;
        if self[0].section_count() >= self[1].section_count() {
            elf1 = self[0];
            elf2 = self[1];
        } else {
            elf2 = self[0];
            elf1 = self[1];
        }

        let left_overlap =
            elf1.start_section <= elf2.start_section && elf1.end_section >= elf2.start_section;
        let right_overlap =
            elf1.start_section >= elf2.start_section && elf1.start_section <= elf2.end_section;

        left_overlap || right_overlap
    }
}

fn collect_assignment_pairs(assignments: Vec<String>) -> Vec<PairAssignment> {
    assignments
        .iter()
        .map(|assignment| {
            let elves = assignment.split(',').collect::<Vec<&str>>();
            let elf1 = elves[0]
                .split('-')
                .map(|val| val.parse::<i32>().unwrap().clone())
                .collect::<Vec<i32>>();
            let elf2 = elves[1]
                .split('-')
                .map(|val| val.parse::<i32>().unwrap().clone())
                .collect::<Vec<i32>>();
            vec![
                Assignment {
                    start_section: elf1[0],
                    end_section: elf1[1],
                },
                Assignment {
                    start_section: elf2[0],
                    end_section: elf2[1],
                },
            ]
        })
        .collect::<Vec<PairAssignment>>()
}

fn collect_assignments(input: &str) -> Vec<String> {
    input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| String::from(l))
        .collect::<Vec<String>>()
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::test_helpers::{vec_compare, TestCase};

    use std::fs;

    fn fixture_assignments() -> Vec<String> {
        vec![
            String::from("2-4,6-8"),
            String::from("2-3,4-5"),
            String::from("5-7,7-9"),
            String::from("2-8,3-7"),
            String::from("6-6,4-6"),
            String::from("2-6,4-8"),
        ]
    }

    fn fixture_assignment_structs() -> Vec<Assignment> {
        vec![
            Assignment {
                start_section: 2,
                end_section: 4,
            },
            Assignment {
                start_section: 6,
                end_section: 8,
            },
            Assignment {
                start_section: 2,
                end_section: 3,
            },
            Assignment {
                start_section: 4,
                end_section: 5,
            },
            Assignment {
                start_section: 5,
                end_section: 7,
            },
            Assignment {
                start_section: 7,
                end_section: 9,
            },
            Assignment {
                start_section: 2,
                end_section: 8,
            },
            Assignment {
                start_section: 3,
                end_section: 7,
            },
            Assignment {
                start_section: 6,
                end_section: 6,
            },
            Assignment {
                start_section: 4,
                end_section: 6,
            },
            Assignment {
                start_section: 2,
                end_section: 6,
            },
            Assignment {
                start_section: 4,
                end_section: 8,
            },
        ]
    }

    fn fixture_assignment_pairs() -> Vec<PairAssignment> {
        vec![
            vec![
                Assignment {
                    start_section: 2,
                    end_section: 4,
                },
                Assignment {
                    start_section: 6,
                    end_section: 8,
                },
            ],
            vec![
                Assignment {
                    start_section: 2,
                    end_section: 3,
                },
                Assignment {
                    start_section: 4,
                    end_section: 5,
                },
            ],
            vec![
                Assignment {
                    start_section: 5,
                    end_section: 7,
                },
                Assignment {
                    start_section: 7,
                    end_section: 9,
                },
            ],
            vec![
                Assignment {
                    start_section: 2,
                    end_section: 8,
                },
                Assignment {
                    start_section: 3,
                    end_section: 7,
                },
            ],
            vec![
                Assignment {
                    start_section: 6,
                    end_section: 6,
                },
                Assignment {
                    start_section: 4,
                    end_section: 6,
                },
            ],
            vec![
                Assignment {
                    start_section: 2,
                    end_section: 6,
                },
                Assignment {
                    start_section: 4,
                    end_section: 8,
                },
            ],
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
