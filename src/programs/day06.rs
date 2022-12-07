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

fn has_duplicated_chars(data: &str) -> bool {
    let mut data_chars = data.chars().collect::<Vec<char>>();
    data_chars.sort();
    data_chars.dedup();
    data_chars.len() < data.len()
}

fn find_marker(data: &str) -> i32 {
    for n in 4usize..data.len() {
        if let Some(last4) = data.get(n-4..n) {
            if last4.len() < 4 {
                panic!("n = {}, last4 = '{}', data = '{}', error: last4.len() < 4", n, last4, data);
            }
            // process the chunk
            if !has_duplicated_chars(last4) {
                return (n).try_into().unwrap()
            }
        } else {
            // end of string
            panic!("Did not find a start of packet marker.");
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::test_helpers::TestCase;

    use std::fs;

    #[test]
    fn test_has_duplicated_chars() {
        let test_cases: Vec<TestCase<&str, bool>> = vec![
            TestCase { input: "aabc", expected: true },
            TestCase { input: "abac", expected: true },
            TestCase { input: "abca", expected: true },
            TestCase { input: "abbc", expected: true },
            TestCase { input: "babc", expected: true },
            TestCase { input: "abcb", expected: true },
            TestCase { input: "bacb", expected: true },
            TestCase { input: "abcd", expected: false },
        ];

        let mut result: bool = false;
        test_cases.iter().for_each(|case| {
            result = has_duplicated_chars(case.input);
            println!("Testing '{}'... result {} =?= expected {} = {}", case.input, result, case.expected, result == case.expected);
            assert_eq!(result, case.expected);
        });
    }

    #[test]
    fn test_find_marker_from_fixture_data() {
        let fixture_file = "./data/day06/test.txt";
        let test_input = fs::read_to_string(fixture_file).expect("Failed to read input file");
        let result = find_marker(&test_input);
        let expected = 7;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_find_marker() {
        let test_data = vec![
            TestCase { input: "bvwbjplbgvbhsrlpgdmjqwftvncz", expected: 5 },
            TestCase { input: "nppdvjthqldpwncqszvftbrmjlhg", expected: 6 },
            TestCase { input: "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", expected: 10 },
            TestCase { input: "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", expected: 11 },
        ];
        
        test_data.iter().for_each(|case| {
            println!("Input: '{}', Expected: {}, Result: {}", case.input, case.expected, find_marker(case.input));
            assert_eq!(find_marker(case.input), case.expected);
        });
    }
}






