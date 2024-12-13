advent_of_code::solution!(2);

use itertools::Itertools;

fn is_safe(data: &Vec<i32>) -> bool {
    let (min, max) = data
        .iter()
        .tuple_windows()
        .map(|(a, b)| a - b)
        .minmax()
        .into_option()
        .unwrap();
    (min > -4 && max < 0) || (min > 0 && max < 4)
}

fn permute_line(data: &Vec<i32>) -> Vec<Vec<i32>> {
    // return a vector of variations of the input that have each element removed in turn
    let mut result = Vec::new();
    for i in 0..data.len() {
        let mut new_data = data.clone();
        new_data.remove(i);
        result.push(new_data);
    }
    result
}

pub fn part_one(input: &str) -> Option<u32> {
    let result = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse::<i32>().unwrap())
                .collect()
        })
        .filter(is_safe)
        .count() as u32;
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let data: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    // Find lines that are safe or have a permutation that is safe
    let safe_lines = data
        .iter()
        .filter(|line| is_safe(line) || permute_line(line).iter().any(is_safe))
        .count() as u32;
    Some(safe_lines)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
