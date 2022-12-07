use crate::parts::Parts;

pub const PACKET_MARKER_LENGTH: usize = 4;
pub const MESSAGE_MARKER_LENGTH: usize = 14;

pub fn run(part_number: Parts, input: &str) -> String {
    let result = compute_result(part_number, &input);
    format!("{}", result)
}

fn compute_result(part_number: Parts, input: &str) -> i32 {
    match part_number {
        Parts::One => find_marker(input, PACKET_MARKER_LENGTH),
        Parts::Two => find_marker(input, MESSAGE_MARKER_LENGTH),
    }
}

fn has_duplicated_chars(data: &str) -> bool {
    let mut data_chars = data.chars().collect::<Vec<char>>();
    data_chars.sort();
    data_chars.dedup();
    data_chars.len() < data.len()
}

fn find_marker(data: &str, marker_length: usize) -> i32 {
    for n in marker_length..data.len() {
        if let Some(candidate_marker) = data.get(n - marker_length..n) {
            if candidate_marker.len() < marker_length {
                panic!("n = {}, candidate_marker = '{}', marker_length = {}, data = '{}', error: candidate_marker.len() < 4", n, candidate_marker, marker_length, data);
            }
            // process the chunk
            if !has_duplicated_chars(candidate_marker) {
                return (n).try_into().unwrap();
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
            TestCase {
                input: "aabc",
                expected: true,
            },
            TestCase {
                input: "abac",
                expected: true,
            },
            TestCase {
                input: "abca",
                expected: true,
            },
            TestCase {
                input: "abbc",
                expected: true,
            },
            TestCase {
                input: "babc",
                expected: true,
            },
            TestCase {
                input: "abcb",
                expected: true,
            },
            TestCase {
                input: "bacb",
                expected: true,
            },
            TestCase {
                input: "abcd",
                expected: false,
            },
            TestCase {
                input: "bacbdefghijklm",
                expected: true,
            },
            TestCase {
                input: "abcdefghijklmn",
                expected: false,
            },
        ];

        let mut result: bool = false;
        test_cases.iter().for_each(|case| {
            result = has_duplicated_chars(case.input);
            println!(
                "Testing '{}'... result {} =?= expected {} = {}",
                case.input,
                result,
                case.expected,
                result == case.expected
            );
            assert_eq!(result, case.expected);
        });
    }

    #[test]
    fn test_find_marker_from_fixture_data() {
        let fixture_file = "./data/day06/test.txt";
        let test_input = fs::read_to_string(fixture_file).expect("Failed to read input file");
        let packet_expected = 7;
        let message_expected = 19;
        assert_eq!(
            find_marker(&test_input, PACKET_MARKER_LENGTH),
            packet_expected
        );
        assert_eq!(
            find_marker(&test_input, MESSAGE_MARKER_LENGTH),
            message_expected
        );
    }

    #[test]
    fn test_find_marker() {
        let packet_test_data = vec![
            TestCase {
                input: "mjqjpqmgbljsphdztnvjfqwrcgsmlb",
                expected: 7,
            },
            TestCase {
                input: "bvwbjplbgvbhsrlpgdmjqwftvncz",
                expected: 5,
            },
            TestCase {
                input: "nppdvjthqldpwncqszvftbrmjlhg",
                expected: 6,
            },
            TestCase {
                input: "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
                expected: 10,
            },
            TestCase {
                input: "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
                expected: 11,
            },
        ];
        let message_test_data = vec![
            TestCase {
                input: "mjqjpqmgbljsphdztnvjfqwrcgsmlb",
                expected: 19,
            },
            TestCase {
                input: "bvwbjplbgvbhsrlpgdmjqwftvncz",
                expected: 23,
            },
            TestCase {
                input: "nppdvjthqldpwncqszvftbrmjlhg",
                expected: 23,
            },
            TestCase {
                input: "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
                expected: 29,
            },
            TestCase {
                input: "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
                expected: 26,
            },
        ];

        packet_test_data.iter().for_each(|case| {
            assert_eq!(find_marker(case.input, PACKET_MARKER_LENGTH), case.expected);
        });
        message_test_data.iter().for_each(|case| {
            assert_eq!(
                find_marker(case.input, MESSAGE_MARKER_LENGTH),
                case.expected
            );
        });
    }
}
