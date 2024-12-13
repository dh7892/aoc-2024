advent_of_code::solution!(13);

use nalgebra::Vector2;
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, line_ending},
    combinator::map_res,
    multi::separated_list1,
    sequence::{pair, preceded, separated_pair},
    IResult,
};

fn parse_number(input: &str) -> IResult<&str, i64> {
    map_res(
        // This will handle both +123 and -123
        preceded(
            nom::combinator::opt(nom::character::complete::one_of("+-")),
            digit1,
        ),
        |num_str: &str| num_str.parse::<i64>(),
    )(input)
}

fn parse_ivec2(input: &str) -> IResult<&str, Vector2<i64>> {
    // Given some input, try to parse it to get an Vector2
    // Format will be: X<number>, Y<number>
    let (input, (x, y)) = separated_pair(
        preceded(tag("X"), parse_number),
        tag(", "),
        preceded(tag("Y"), parse_number),
    )(input)?;
    Ok((input, Vector2::new(x, y)))
}

fn parse_target(input: &str) -> IResult<&str, Vector2<i64>> {
    // Given some input, try to parse it to get an Vector2
    // Format will be: Prize: X=<number>, Y=<number>
    let (input, _) = tag("X=")(input)?;
    let (input, x) = parse_number(input)?;
    let (input, _) = tag(", Y=")(input)?;
    let (input, y) = parse_number(input)?;
    Ok((input, Vector2::new(x, y)))
}

fn parse_block(input: &str) -> IResult<&str, (Vector2<i64>, Vector2<i64>, Vector2<i64>)> {
    // Given some input, try to parse it to get the A, B and target values
    // Format will be:
    // Button A: X<number], Y<number>
    // Button B: X<number], Y<number>
    // Prize: X<number], Y<number>
    let (input, _) = tag("Button A: ")(input)?;
    let (input, a) = parse_ivec2(input)?;
    let (input, _) = tag("\nButton B: ")(input)?;
    let (input, b) = parse_ivec2(input)?;
    let (input, _) = tag("\nPrize: ")(input)?;
    let (input, prize) = parse_target(input)?;
    Ok((input, (a, b, prize)))
}

fn find_solutions_alg(
    a_vec: Vector2<i64>,
    b_vec: Vector2<i64>,
    target: Vector2<i64>,
) -> Vec<(i64, i64)> {
    let denomenator = a_vec.x * b_vec.y - a_vec.y * b_vec.x;
    let numerator = a_vec.x * target.y - a_vec.y * target.x;
    if denomenator == 0 {
        // The only solution if is we can reach the target by multiples of a alone
        let a = target.x / a_vec.x;
        if a_vec * a == target {
            return vec![(a, 0)];
        }
    }
    let b = numerator / denomenator;
    let a = (target.x - b * b_vec.x) / a_vec.x;
    if a_vec * a + b_vec * b == target {
        vec![(a, b)]
    } else {
        vec![]
    }
}

fn cost_for_solution(a: i64, b: i64) -> i64 {
    // Given a solution, calculate the cost
    // a costs 3, b costs 1
    a * 3 + b
}

fn parse_input(input: &str) -> IResult<&str, Vec<(Vector2<i64>, Vector2<i64>, Vector2<i64>)>> {
    separated_list1(pair(line_ending, line_ending), parse_block)(input)
}

pub fn part_one(input: &str) -> Option<usize> {
    let (_, blocks) = parse_input(input).unwrap();
    let result = blocks
        .iter()
        .map(|(a, b, prize)| {
            find_solutions_alg(*a, *b, *prize)
                .iter()
                .map(|(a, b)| cost_for_solution(*a, *b))
                .min()
                .unwrap_or(0)
        })
        .sum::<i64>();
    Some(result as usize)
}

pub fn part_two(input: &str) -> Option<usize> {
    let (_, blocks) = parse_input(input).unwrap();
    let offset = Vector2::new(10000000000000, 10000000000000);
    let result = blocks
        .iter()
        .map(|(a, b, prize)| {
            find_solutions_alg(*a, *b, *prize + offset)
                .iter()
                .map(|(a, b)| cost_for_solution(*a, *b))
                .min()
                .unwrap_or(0)
        })
        .sum::<i64>();
    Some(result as usize)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(875318608908));
    }

    #[test]
    fn test_parse_block() {
        let input = "Button A: X+1, Y+2\nButton B: X+3, Y+4\nPrize: X=5, Y=6";
        let (_, (a, b, prize)) = parse_block(input).unwrap();
        assert_eq!(a, Vector2::new(1, 2));
        assert_eq!(b, Vector2::new(3, 4));
        assert_eq!(prize, Vector2::new(5, 6));
    }
}
