advent_of_code::solution!(3);

use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::verify;
use nom::sequence::{delimited, separated_pair};
use nom::IResult;

fn do_parser(input: &str) -> IResult<&str, &str> {
    tag("do()")(input)
}

fn dont_parser(input: &str) -> IResult<&str, &str> {
    tag("don't()")(input)
}

fn number_1_to_3_digits(input: &str) -> IResult<&str, &str> {
    verify(digit1, |s: &str| s.len() >= 1 && s.len() <= 3)(input)
}

fn mul_parser(input: &str) -> IResult<&str, (u32, u32)> {
    delimited(
        tag("mul("),
        separated_pair(number_1_to_3_digits, tag(","), number_1_to_3_digits),
        tag(")"),
    )(input)
    .map(|(remaining, (d1, d2))| (remaining, (d1.parse().unwrap(), d2.parse().unwrap())))
}

fn find_all_muls(input: &str) -> Vec<(u32, u32)> {
    let mut result = Vec::new();
    let mut current_input = input;

    while !current_input.is_empty() {
        match mul_parser(current_input) {
            Ok((remaining, pair)) => {
                result.push(pair);
                current_input = remaining;
            }
            Err(_) => {
                // Skip one character and try again
                current_input = &current_input[1..];
            }
        }
    }

    result
}

fn find_all_enabled_muls(input: &str) -> Vec<(u32, u32)> {
    let mut result = Vec::new();
    let mut current_input = input;
    let mut enabled = true;

    while !current_input.is_empty() {
        match do_parser(current_input) {
            Ok((remaining, _)) => {
                enabled = true;
                current_input = remaining;
            }
            Err(_) => {}
        }
        match dont_parser(current_input) {
            Ok((remaining, _)) => {
                enabled = false;
                current_input = remaining;
            }
            Err(_) => {}
        }
        match mul_parser(current_input) {
            Ok((remaining, pair)) => {
                if enabled {
                    result.push(pair);
                }
                current_input = remaining;
            }
            Err(_) => {
                // Skip one character and try again
                current_input = &current_input[1..];
            }
        }
    }

    result
}

pub fn part_one(input: &str) -> Option<u32> {
    let operands = find_all_muls(input);
    let result = operands.iter().fold(0, |acc, (a, b)| acc + a * b);
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let operands = find_all_enabled_muls(input);
    let result = operands.iter().fold(0, |acc, (a, b)| acc + a * b);
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let custom_input =
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let result = part_two(custom_input);
        assert_eq!(result, Some(48));
    }

    #[test]
    fn test_mult_operation_works() {
        let result = mul_parser("mul(123,456)");
        assert_eq!(result, Ok(("", (123, 456))));
    }
    #[test]
    fn test_mult_operation_does_not_find_spaces() {
        let result = mul_parser("mul( 123 , 456 )");
        // Should find nothing as we don't allow spaces
        let expected_value = Err(nom::Err::Error(nom::error::Error::new(
            " 123 , 456 )",
            nom::error::ErrorKind::Digit,
        )));
        assert_eq!(result, expected_value);
    }

    #[test]
    fn test_example() {
        // let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)";
        let result = part_one(input);
        assert_eq!(result, Some(33));
    }
}
