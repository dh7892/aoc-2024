advent_of_code::solution!(4);

use itertools::Itertools;

fn str_to_2d_vec(input: &str) -> Vec<Vec<char>> {
    // Split our multiline string into a 2D vector of characters
    input.lines().map(|line| line.chars().collect()).collect()
}

fn iter_in_direction(
    characters: &[Vec<char>],
    start: (usize, usize),
    direction: (i32, i32),
) -> String {
    let start_x = start.0 as i32;
    let start_y = start.1 as i32;
    let mut result = Vec::new();
    let max_steps = 4;
    for step in 0..max_steps {
        let x = start_x + direction.0 * step;
        let y = start_y + direction.1 * step;
        if x < 0 || y < 0 || x >= characters[0].len() as i32 || y >= characters.len() as i32 {
            break;
        }
        result.push(characters[y as usize][x as usize]);
    }
    result.iter().collect()
}

fn x_mas_at_location(c: &[Vec<char>], start: (usize, usize)) -> bool {
    // Return true if the 3x3 grid with top-left at our specified location
    // contains a cross of the word "mas" in any direction
    let (x, y) = start;

    // Return early if we are at the edge of the grid
    if x >= c[0].len() - 2 || y >= c.len() - 2 {
        return false;
    }
    let first_diagonal = String::from_iter([c[y][x], c[y + 1][x + 1], c[y + 2][x + 2]]);
    let second_diagonal = String::from_iter([c[y + 2][x], c[y + 1][x + 1], c[y][x + 2]]);

    (first_diagonal == "MAS" || first_diagonal == "SAM")
        && (second_diagonal == "MAS" || second_diagonal == "SAM")
}

fn possible_directions() -> Vec<(i32, i32)> {
    vec![
        (0, 1),
        (1, 0),
        (1, 1),
        (1, -1),
        (0, -1),
        (-1, 0),
        (-1, 1),
        (-1, -1),
    ]
}

pub fn part_one(input: &str) -> Option<u32> {
    let characters = str_to_2d_vec(input);
    let word = "XMAS";
    let rows = characters.len();
    let cols = characters[0].len();

    let count = (0..cols)
        .cartesian_product(0..rows)
        .cartesian_product(possible_directions())
        .filter(|((col, row), direction)| {
            let candidate_word = iter_in_direction(&characters, (*col, *row), *direction);
            candidate_word == word
        })
        .count();

    Some(count as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let characters = str_to_2d_vec(input);
    let rows = characters.len();
    let cols = characters[0].len();

    let count = (0..cols)
        .cartesian_product(0..rows)
        .filter(|(col, row)| x_mas_at_location(&characters, (*col, *row)))
        .count();

    Some(count as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
