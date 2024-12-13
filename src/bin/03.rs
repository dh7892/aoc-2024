advent_of_code::solution!(3);

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::anychar;
use nom::character::complete::digit1;
use nom::combinator::value;
use nom::combinator::verify;
use nom::multi::many1;
use nom::multi::many_till;
use nom::sequence::{delimited, separated_pair};
use nom::IResult;
use nom::Parser;

#[derive(Debug, PartialEq, Clone)]
enum Operator {
    Mul(u32, u32),
    Do,
    Dont,
}

fn number_1_to_3_digits(input: &str) -> IResult<&str, &str> {
    verify(digit1, |s: &str| !s.is_empty() && s.len() <= 3)(input)
}

fn mul_parser(input: &str) -> IResult<&str, Operator> {
    let (input, (a, b)) = delimited(
        tag("mul("),
        separated_pair(number_1_to_3_digits, tag(","), number_1_to_3_digits),
        tag(")"),
    )(input)?;
    Ok((input, Operator::Mul(a.parse().unwrap(), b.parse().unwrap())))
}

fn operator_parser(input: &str) -> IResult<&str, Operator> {
    // Parse any operator
    alt((
        value(Operator::Do, tag("do()")),
        value(Operator::Dont, tag("don't()")),
        mul_parser,
    ))(input)
}

fn find_all_operators(input: &str) -> Vec<Operator> {
    // Parse a long string that may contain a lot of junk and
    // pull out a vec of all operators in order
    many1(many_till(anychar, operator_parser).map(|(_discard, operator)| operator))(input)
        .unwrap()
        .1
}

pub fn part_one(input: &str) -> Option<u32> {
    let operands = find_all_operators(input);
    let result = operands
        .iter()
        .filter_map(|op| match op {
            Operator::Mul(a, b) => Some(a * b),
            _ => None,
        })
        .sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let operators = find_all_operators(input);
    let mut sum = 0;
    let mut enabled = true;
    for op in operators {
        match op {
            Operator::Mul(a, b) => {
                if enabled {
                    sum += a * b;
                }
            }
            Operator::Do => {
                enabled = true;
            }
            Operator::Dont => {
                enabled = false;
            }
        }
    }
    Some(sum)
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
        assert_eq!(result, Ok(("", Operator::Mul(123, 456))));
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
