advent_of_code::solution!(8);

use std::collections::HashMap;
use std::collections::HashSet;

use itertools::Itertools;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

// Impl debug output for Position that sits neatly on one line
impl std::fmt::Debug for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Position({}, {})", self.x, self.y)
    }
}

impl std::ops::Add for Position {
    type Output = Position;

    fn add(self, other: Position) -> Position {
        Position {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::Sub for Position {
    type Output = Position;

    fn sub(self, other: Position) -> Position {
        Position {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

fn parse_input(input: &str) -> (HashMap<char, Vec<Position>>, Position) {
    let mut map = HashMap::new();
    let mut position = Position { x: 0, y: 0 };
    for (y, line) in input.lines().enumerate() {
        if y as i32 > position.y {
            position.y = y as i32;
        }
        for (x, c) in line.chars().enumerate() {
            if x as i32 > position.x {
                position.x = x as i32;
            }
            if c == '.' {
                continue;
            }
            map.entry(c).or_insert_with(Vec::new).push(Position {
                x: x as i32,
                y: y as i32,
            });
        }
    }

    (map, position)
}

fn find_antinode(first: Position, second: Position, max: Position) -> Option<Position> {
    // Given twp nodes, calculate the position of the antinode
    // which done by calculating the vector difference between the two nodes
    // and adding that on to the second node
    let diff = second - first;
    let antinode = second + diff;
    // dbg!(first, second, antinode);
    if antinode.x < 0 || antinode.y < 0 || antinode.x > max.x || antinode.y > max.y {
        None
    } else {
        Some(antinode)
    }
}

fn all_antinodes(map: &HashMap<char, Vec<Position>>, max: Position) -> HashSet<Position> {
    let mut antinodes = HashSet::new();
    for positions in map.values() {
        // Consider the cartesian product of all pairs of positions
        for (first, second) in positions.iter().tuple_combinations() {
            if let Some(antinode) = find_antinode(*first, *second, max) {
                antinodes.insert(antinode);
            }
            // Need to try the other way around as well
            if let Some(antinode) = find_antinode(*second, *first, max) {
                antinodes.insert(antinode);
            }
        }
    }

    antinodes
}

fn all_antinodes_including_resonant_frequencies(
    map: &HashMap<char, Vec<Position>>,
    max: Position,
) -> HashSet<Position> {
    let mut antinodes = HashSet::new();
    for positions in map.values() {
        // Consider the cartesian product of all pairs of positions
        for (first, second) in positions.iter().tuple_combinations() {
            // While there is an antinode from the two nodes
            // We keep checking for more antinodes by considering the second node and the antinode
            let mut current = *first;
            let mut next = *second;
            // For this algorithm, we include the loction of the nodes as well
            antinodes.insert(next);
            while let Some(antinode) = find_antinode(current, next, max) {
                antinodes.insert(antinode);
                current = next;
                next = antinode;
            }
            // Need to look in the other direction as well
            current = *second;
            next = *first;
            antinodes.insert(next);
            while let Some(antinode) = find_antinode(current, next, max) {
                antinodes.insert(antinode);
                current = next;
                next = antinode;
            }
        }
    }

    antinodes
}

pub fn part_one(input: &str) -> Option<usize> {
    let (map, max) = parse_input(input);
    let antinodes = all_antinodes(&map, max);
    Some(antinodes.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let (map, max) = parse_input(input);
    let antinodes = all_antinodes_including_resonant_frequencies(&map, max);
    // dbg!(&antinodes);
    Some(antinodes.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
