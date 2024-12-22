advent_of_code::solution!(22);

use std::collections::{HashMap, HashSet};

fn step(before: usize) -> usize {
    let stage_1 = ((before * 64) ^ before) % 16777216;
    let stage_2 = ((stage_1 / 32) ^ stage_1) % 16777216;
    ((stage_2 * 2048) ^ stage_2) % 16777216
}

fn all_steps_last_digits_deltas(start_num: usize) -> Vec<(i8, i8)> {
    // Given a starting number, work out the sequence of 2000 number that follow
    // Then take only the last digit of each number
    // Then calculate the difference between successive numbers and retunr that with the number
    // itself
    // E.g. for each digit index i, return (d[i] - d[i-1], d[i])
    let mut result = vec![start_num];
    for _ in 0..2000 {
        result.push(step(*result.last().unwrap()));
    }
    let last_digits = result
        .iter()
        .map(|num| (num % 10) as i8)
        .collect::<Vec<i8>>();
    last_digits
        .windows(2)
        .map(|window| (window[1] - window[0], window[1]))
        .collect()
}

fn sequences(start_num: usize) -> HashMap<(i8, i8, i8, i8), u8> {
    let mut result = HashMap::new();
    let all_steps = all_steps_last_digits_deltas(start_num);
    all_steps.windows(4).for_each(|window| {
        let key = (window[0].0, window[1].0, window[2].0, window[3].0);
        let value = window[3].1 as u8;
        // Only insert the result if the key is not already in the map
        // Because we'd sell at the first instance of the key so we only want the first one we find
        result.entry(key).or_insert(value);
    });
    result
}

fn all_sequences(input: &str) -> Vec<HashMap<(i8, i8, i8, i8), u8>> {
    input
        .lines()
        .map(|line| line.parse::<usize>().expect("Invalid input"))
        .map(sequences)
        .collect()
}

fn total_for_sequence(
    sequences: Vec<HashMap<(i8, i8, i8, i8), u8>>,
    keys: (i8, i8, i8, i8),
) -> usize {
    // Given a sequence, sum up the sale prices
    sequences
        .iter()
        .fold(0, |acc, seq| acc + *seq.get(&keys).unwrap_or(&0) as usize)
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            .map(|line| line.parse::<usize>().expect("Invalid input"))
            .map(|start_num| {
                let mut result = start_num;
                for _ in 0..2000 {
                    result = step(result);
                }
                result
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let sequences = all_sequences(input);
    // Now get all possible keys for all sequences
    let all_keys = sequences.iter().fold(HashSet::new(), |acc, seq| {
        acc.union(&seq.keys().collect()).cloned().collect()
    });
    // Now find the key that gives the highest total
    Some(
        all_keys
            .iter()
            .map(|key| total_for_sequence(sequences.clone(), **key))
            .max()
            .unwrap(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(123, 15887950)]
    #[case(15887950, 16495136)]
    #[case(16495136, 527345)]
    #[case(527345, 704524)]
    #[case(704524, 1553684)]
    #[case(1553684, 12683156)]
    #[case(12683156, 11100544)]
    #[case(11100544, 12249484)]
    #[case(12249484, 7753432)]
    #[case(7753432, 5908254)]
    fn test_step(#[case()] start: usize, #[case()] expected: usize) {
        assert_eq!(step(start), expected);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(37327623));
    }

    #[test]
    fn test_part_two() {
        let input = "1
2
3
2024";
        let result = part_two(input);
        assert_eq!(result, Some(23));
    }
}
