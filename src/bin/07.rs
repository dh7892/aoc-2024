advent_of_code::solution!(7);

fn can_reach_target(
    target: usize,
    current_total: usize,
    operands: &[usize],
    index: usize,
    allow_concat: bool,
) -> bool {
    if index >= operands.len() {
        return target == current_total;
    }

    if current_total > target {
        return false;
    }

    let next_operand = operands[index];

    can_reach_target(
        target,
        current_total + next_operand,
        operands,
        index + 1,
        allow_concat,
    ) || can_reach_target(
        target,
        current_total * next_operand,
        operands,
        index + 1,
        allow_concat,
    ) || (allow_concat
        && can_reach_target(
            target,
            concat_numbers(current_total, next_operand),
            operands,
            index + 1,
            allow_concat,
        ))
}

fn parse_line(input: &str) -> (usize, Vec<usize>) {
    let mut parts = input.split(": ");
    let target = parts.next().unwrap().parse().unwrap();
    let operands = parts
        .next()
        .unwrap()
        .split(" ")
        .map(|part| part.parse().unwrap())
        .collect::<Vec<usize>>();

    (target, operands)
}

fn concat_numbers(left: usize, right: usize) -> usize {
    // Concatenate the two numbers e.g. 12 and 34 -> 1234
    let mut right = right;
    let original_right = right;
    let mut left = left;
    while right > 0 {
        left *= 10;
        right /= 10;
    }
    left + original_right
}

pub fn part_one(input: &str) -> Option<usize> {
    // Initialise tracing
    tracing_subscriber::fmt::init();
    let allow_concat = false;

    Some(
        input
            .lines()
            .filter_map(|line| {
                let (target, operands) = parse_line(line);
                if can_reach_target(target, operands[0], &operands, 1, allow_concat) {
                    Some(target)
                } else {
                    None
                }
            })
            .sum::<usize>(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let allow_concat = true;
    Some(
        input
            .lines()
            .filter_map(|line| {
                let (target, operands) = parse_line(line);
                if can_reach_target(target, operands[0], &operands, 1, allow_concat) {
                    Some(target)
                } else {
                    None
                }
            })
            .sum::<usize>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }

    #[test]
    fn test_example() {
        let target = 10;
        let operands = vec![2, 3, 4];
        let first_operand = operands[0];
        let remaining_operands = &operands[1..];
        let result = can_reach_target(target, first_operand, &remaining_operands, 0, false);
        assert_eq!(result, true);
    }

    #[rstest]
    #[case(1, 2, 12)]
    #[case(12, 34, 1234)]
    #[case(123, 456, 123456)]
    fn test_concat_numbers(#[case] left: usize, #[case] right: usize, #[case] expected: usize) {
        let result = concat_numbers(left, right);
        assert_eq!(result, expected);
    }
}
