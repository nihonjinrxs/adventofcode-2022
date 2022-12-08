use crate::parts::Parts;

pub fn run(part_number: Parts, input: &str) -> String {
    let result = compute_result(part_number, &input);
    format!("{}", result)
}

fn compute_result(part_number: Parts, input: &str) -> i32 {
    match part_number {
        Parts::One => directories_total_size(input),
        Parts::Two => directories_total_size(input),
    }
}

fn directories_total_size(input: &str) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::TestCase;

    use std::fs;

    #[test]
    fn test_testing_works() {
        assert_eq!(3, 3);
    }
}
