use crate::parts::Parts;

pub fn run(part_number: Parts, input: &str) -> String {
    let strategy_guide = collect_assignments(input);
    let result = compute_result(part_number, strategy_guide);
    format!("{}", result)
}

fn compute_result(part_number: Parts, assignments: Vec<String>) -> i32 {
    match part_number {
        Parts::One => compute_fully_overlapping_pairs(assignments),
        Parts::Two => compute_fully_overlapping_pairs(assignments),
    }
}

fn compute_fully_overlapping_pairs(_assignments: Vec<String>) -> i32 {
   0 
}

fn collect_assignments(input: &str) -> Vec<String> {
    input.lines().
        map(|l| l.trim()).
        filter(|l| !l.is_empty()).
        map(|l| String::from(l)).
        collect::<Vec<String>>()
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::test_helpers::vec_compare;

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

    #[test]
    fn test_collect_assignments() {
        let fixture_file = "./data/day04/test.txt";
        let test_input = fs::read_to_string(fixture_file)
            .expect("Failed to read input file");
        let result = collect_assignments(&test_input);
        let expected = fixture_assignments();
        assert!(vec_compare(&result, &expected));
    }

    // #[test]
    // fn test_compute_fully_overlapping_pairs() {
    //     let test_data = fixture_assignments();
    //     let result = compute_fully_overlapping_pairs(test_data);
    //     assert_eq!(result, 2);
    // }
}
