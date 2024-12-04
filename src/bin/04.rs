advent_of_code::solution!(4);

fn str_to_2d_vec(input: &str) -> Vec<Vec<&str>> {
    // Split our multiline string into a 2D vector of characters
    input
        .lines()
        .map(|line| line.split("").filter(|c| !c.is_empty()).collect())
        .collect()
}

fn iter_in_direction<'a>(
    characters: &Vec<Vec<&'a str>>,
    start: (usize, usize),
    direction: (i32, i32),
) -> Vec<&'a str> {
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
    result
}

fn x_max_at_location(c: &Vec<Vec<&str>>, start: (usize, usize)) -> bool {
    // Return true if the 3x3 grid with top-left at our specified location
    // contains a cross of the word "mas" in any direction
    let (x, y) = start;

    // Return early if we are at the edge of the grid
    if x >= c[0].len() - 2 || y >= c.len() - 2 {
        return false;
    }
    let first_diagonal = String::from_iter([c[y + 0][x + 0], c[y + 1][x + 1], c[y + 2][x + 2]]);
    let second_diagonal = String::from_iter([c[y + 2][x + 0], c[y + 1][x + 1], c[y + 0][x + 2]]);

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
    let mut found_count = 0;
    for y in 0..characters.len() {
        for x in 0..characters[0].len() {
            for direction in possible_directions() {
                let candidate_word = iter_in_direction(&characters, (x, y), direction)
                    .iter()
                    .map(|c| *c)
                    .collect::<String>();
                if candidate_word == word {
                    found_count += 1;
                }
            }
        }
    }

    Some(found_count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let characters = str_to_2d_vec(input);
    let mut count = 0;
    for y in 0..characters.len() {
        for x in 0..characters[0].len() {
            if x_max_at_location(&characters, (x, y)) {
                count += 1;
            }
        }
    }
    Some(count)
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
