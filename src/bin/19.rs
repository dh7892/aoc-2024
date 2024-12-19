advent_of_code::solution!(19);

fn parse_input(input: &str) -> (Vec<String>, Vec<String>) {
    let mut parts = input.split("\n\n");
    let towels = parts
        .next()
        .unwrap()
        .split(", ")
        .map(|s| s.to_owned())
        .collect();
    let patterns = parts
        .next()
        .unwrap()
        .lines()
        .map(|s| s.to_owned())
        .collect();
    (towels, patterns)
}

fn make_pattern(towels: &[String], patterns: &[String]) -> Option<Vec<String>> {
    // See if we can make the pattern by concatenating any number of any of the allowable towels
    // togeether
    // If we can, return the pattern
    // If we can't, return an error
    todo!()
}

pub fn part_one(input: &str) -> Option<usize> {
    let (towels, patterns) = parse_input(input);
    let num_achievable = patterns
        .iter()
        .filter(|pattern| make_pattern(&towels, &patterns).is_some())
        .count();
    Some(num_achievable)
}

pub fn part_two(input: &str) -> Option<usize> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
