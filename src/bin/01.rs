advent_of_code::solution!(1);

fn appearances(num: i32, list: &[i32]) -> usize {
    // Return the number of times num appears in the list
    list.iter().filter(|&a| a == &num).count()
}

fn input_to_columns(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut column1: Vec<i32> = Vec::new();
    let mut column2: Vec<i32> = Vec::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 2 {
            if let Ok(num1) = parts[0].parse::<i32>() {
                column1.push(num1);
            }
            if let Ok(num2) = parts[1].parse::<i32>() {
                column2.push(num2);
            }
        }
    }

    (column1, column2)
}

pub fn part_one(input: &str) -> Option<u32> {
    // Create two vectors to store the integers from each column
    let (mut column1, mut column2) = input_to_columns(input);

    column1.sort();
    column2.sort();

    let sum_of_differences: i32 = column1
        .iter()
        .zip(column2.iter())
        .map(|(&a, &b)| (a - b).abs())
        .sum();

    // For now, we'll just return None. You can implement the rest of the logic later.
    Some(sum_of_differences as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (column1, column2) = input_to_columns(input);

    let result: i32 = column1
        .iter()
        .map(|&a| a * appearances(a, &column2) as i32)
        .sum();
    Some(result as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
