advent_of_code::solution!(11);

use memoize::memoize;

#[memoize]
fn blink_a_number(num: usize, count: usize) -> usize {
    // Given a number, return the number of numbers it would become
    // after blinking count times
    let num_as_string = num.to_string();
    let num_digits = num_as_string.len();
    if count == 1 {
        return match num_digits % 2 {
            0 => 2, // Even number of digits so we'd split
            _ => 1, // Odd number of digits so we wouldn't split
        };
    }

    // We have more than one blink left so we need to use recursion
    match (num, num_digits % 2) {
        (0, _) => blink_a_number(1, count - 1),
        (_, 0) => {
            let (left_str, right_str) = num_as_string.split_at(num_digits / 2);
            let left = left_str.parse::<usize>().unwrap();
            let right = right_str.parse::<usize>().unwrap();
            blink_a_number(left, count - 1) + blink_a_number(right, count - 1)
        }
        (n, _) => blink_a_number(n * 2024, count - 1),
    }
}

fn blink_all_numbers(numbers: &[usize], count: usize) -> usize {
    numbers.iter().map(|n| blink_a_number(*n, count)).sum()
}

fn input_to_numbers(input: &str) -> Vec<usize> {
    input
        .trim()
        .split(' ')
        .map(|x| x.parse::<usize>().unwrap())
        .collect()
}

pub fn part_one(input: &str) -> Option<usize> {
    let numbers = input_to_numbers(input);
    let num_blinks = 25;
    Some(blink_all_numbers(&numbers, num_blinks))
}

pub fn part_two(input: &str) -> Option<usize> {
    let numbers = input_to_numbers(input);
    let num_blinks = 75;
    Some(blink_all_numbers(&numbers, num_blinks))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let numbers = input_to_numbers(&input);
        let num_blinks = 6;
        let result = Some(blink_all_numbers(&numbers, num_blinks));
        assert_eq!(result, Some(22));
    }

    // No need to test part 2 as it's the same as part one, only much longer!
}
