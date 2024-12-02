advent_of_code::solution!(2);

use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    let data: Vec<Vec<i32>> = input
        .lines()
        .filter_map(|line| {
            let diffs: Vec<_> = line
                .split_whitespace()
                .map(|element| element.parse::<i32>().unwrap())
                .tuple_windows()
                .map(|(a, b)| a - b)
                .collect();
            let (min, max) = diffs.iter().minmax().into_option()?;
            // We are happy if the min is -2 or -3 and the max is -1
            // or if the max is 2 or 3 and the min is 1
            if (min > &-4 && max < &0) || (min > &0 && max < &4) {
                return Some(diffs);
            } else {
                return None;
            };
        })
        .collect();
    Some(data.len() as u32)
}

fn is_safe(data: Vec<i32>) -> bool {
    let (min, max) = data
        .iter()
        .tuple_windows()
        .map(|(a, b)| a - b)
        .minmax()
        .into_option()
        .unwrap();
    // We are happy if the min is -2 or -3 and the max is -1
    // or if the max is 2 or 3 and the min is 1
    if (min > -4 && max < 0) || (min > 0 && max < 4) {
        return true;
    } else {
        return false;
    };
}

fn permute_line(data: Vec<i32>) -> Vec<Vec<i32>> {
    // return a vector of variations of the input that have each element removed in turn
    let mut result = Vec::new();
    for i in 0..data.len() {
        let mut new_data = data.clone();
        new_data.remove(i);
        result.push(new_data);
    }
    result
}

pub fn part_two(input: &str) -> Option<u32> {
    let data: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|element| element.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    // Find lines that are safe or have a permutation that is safe
    let mut safe_lines = Vec::new();
    for line in data {
        if is_safe(line.clone()) {
            safe_lines.push(line);
        } else {
            for permuted_line in permute_line(line) {
                if is_safe(permuted_line.clone()) {
                    safe_lines.push(permuted_line);
                    break;
                }
            }
        }
    }
    Some(safe_lines.len() as u32)
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
