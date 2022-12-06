use crate::parts::Parts;

pub fn run(part_number: Parts, input: &str) -> String {
    let result = compute_result(part_number, &input);
    format!("{}", result)
}

fn compute_result(part_number: Parts, input: &str) -> i32 {
    match part_number {
        Parts::One => find_marker(input),
        Parts::Two => find_marker(input),
    }
}

fn find_marker(data: &str) -> {
    let mut start_of_last4: usize = 0;
    for n in (3usize..data).len() {
        if let Some(last4) = data.get(n-3..n) {
            // process the chunk
            let current = last4[3];
            let previous = last4.get(0..3);
            if current in previous {
                start_of_last4 = previous.find(current) + 1;
            }
        } else {
            // end of string
            panic!("Did not find a start of packet marker.");
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::test_helpers::TestCase;

    use std::fs;

    #[test]
    fn test_find_marker_from_fixture_data() {
        let fixture_file = "./data/day01/test.txt";
        let test_input = fs::read_to_string(fixture_file).expect("Failed to read input file");
        let result = find_marker(&test_input);
        let expected = 7;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_find_marker() {
        let mut test_data = vec![
            TestCase { input: "bvwbjplbgvbhsrlpgdmjqwftvncz", expected: 5 },
            TestCase { input: "nppdvjthqldpwncqszvftbrmjlhg", expected: 6 },
            TestCase { input: "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", expected: 10 },
            TestCase { input: "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", expected: 11 },
        ];
        test_data.iter().for_each(|case| {
            assert_eq!(find_marker(case.input), case.expected);
        });
    }
}






