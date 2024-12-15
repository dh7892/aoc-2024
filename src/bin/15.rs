advent_of_code::solution!(15);

use std::collections::HashMap;

// Going to allow negative numbers for now
// This makes it easier to handle going off the edge of the map
// Because we can do a calculation like `x + dx` and not have to worry about bounds checking
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Object {
    Wall,
    Box,
    LeftBox,
    RightBox,
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn dx(&self) -> i32 {
        match self {
            Direction::Left => -1,
            Direction::Right => 1,
            _ => 0,
        }
    }
    fn dy(&self) -> i32 {
        match self {
            Direction::Up => -1,
            Direction::Down => 1,
            _ => 0,
        }
    }
}

type Map = HashMap<Position, Object>;

fn read_map(input: &str) -> (Map, Position) {
    // Read the map and the position of the robot
    let mut position = Position { x: 0, y: 0 };
    let mut map = Map::new();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let x = x as i32;
            let y = y as i32;
            match c {
                '#' => {
                    map.insert(Position { x, y }, Object::Wall);
                }
                'O' => {
                    map.insert(Position { x, y }, Object::Box);
                }
                '@' => position = Position { x, y },
                _ => {}
            }
        }
    }

    (map, position)
}

fn read_directions(input: &str) -> Vec<Direction> {
    input
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| match c {
            '^' => Direction::Up,
            'v' => Direction::Down,
            '<' => Direction::Left,
            '>' => Direction::Right,
            _ => panic!("Invalid direction"),
        })
        .collect()
}

fn parse_input(input: &str) -> (Map, Position, Vec<Direction>) {
    let mut lines = input.lines();
    let map_lines = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .collect::<Vec<_>>();
    let directions_lines = lines.collect::<Vec<_>>();
    let (map, position) = read_map(&map_lines.join("\n"));
    let directions = read_directions(&directions_lines.join("\n"));

    (map, position, directions)
}

fn calc_score(box_map: &Map) -> usize {
    // Calculate the score of the map
    // Sum up the GPS score for each box
    // The GPS score is y*100 + x
    box_map
        .iter()
        .filter(|(_, o)| **o == Object::Box || **o == Object::LeftBox)
        .map(|(p, _)| (p.y * 100 + p.x) as usize)
        .sum()
}

fn move_robot(map: &mut Map, position: Position, direction: Direction) -> Position {
    let next_position = Position {
        x: position.x + direction.dx(),
        y: position.y + direction.dy(),
    };

    let (success, mut new_positions) = try_push_blocks(next_position, map, direction);
    if !success {
        return position;
    }

    // If we're moving vertically and or next position is a left or right box
    // We need to move the other half of the box, too
    if direction.dy() != 0 {
        match map.get(&next_position) {
            Some(Object::LeftBox) => {
                let other_position = Position {
                    x: next_position.x + 1,
                    y: next_position.y,
                };
                let (other_success, other_new_positions) =
                    try_push_blocks(other_position, map, direction);
                if other_success {
                    new_positions.extend(other_new_positions);
                } else {
                    // Couldn't move the other block so can't move
                    return position;
                }
            }
            Some(Object::RightBox) => {
                let other_position = Position {
                    x: next_position.x - 1,
                    y: next_position.y,
                };
                let (other_success, other_new_positions) =
                    try_push_blocks(other_position, map, direction);
                if other_success {
                    new_positions.extend(other_new_positions);
                } else {
                    // Couldn't move the other block so can't move
                    return position;
                }
            }
            _ => (),
        }
    }

    // NOTE due to the way the algorithm works, there will be duplicates in the lists
    // This is fine as both remove and add will not break in these cases
    let old_positions = new_positions.iter().map(|(p, _, _)| *p).collect::<Vec<_>>();
    let new_objects = new_positions
        .iter()
        .map(|(_, p, o)| (*p, *o))
        .collect::<Vec<_>>();
    for p in old_positions {
        map.remove(&p);
    }
    for (p, o) in new_objects {
        map.insert(p, o);
    }
    Position {
        x: position.x + direction.dx(),
        y: position.y + direction.dy(),
    }
}

fn try_push_blocks(
    position: Position,
    map: &Map,
    direction: Direction,
) -> (bool, Vec<(Position, Position, Object)>) {
    // Check recursively if we can push all the blocks in the given direction
    // The check goes like this:
    // If the next positon is a wall, return false
    // If the next position is a space, return true
    // If the next position is a box, call recursively with the next position
    // We also return a list of old and new positions of the boxes that were moved
    // If this function has returned false, the list will be empty

    let (dx, dy) = (direction.dx(), direction.dy());
    let next_position = Position {
        x: position.x + dx,
        y: position.y + dy,
    };

    let current_object = match map.get(&position) {
        Some(Object::Wall) => return (false, vec![]),
        Some(o) => *o,
        None => return (true, vec![]),
    };
    let mut new_positions = vec![(position, next_position, current_object)];
    match map.get(&next_position) {
        Some(Object::Wall) => (false, vec![]),
        None => (true, new_positions),
        Some(Object::Box) => {
            let (can_push, other_new_positions) = try_push_blocks(next_position, map, direction);
            if can_push {
                new_positions.extend(other_new_positions);
            } else {
                return (false, vec![]);
            }

            (can_push, new_positions)
        }
        Some(Object::LeftBox) => {
            let (can_push, other_new_positions) = try_push_blocks(next_position, map, direction);
            if can_push {
                new_positions.extend(other_new_positions);
            } else {
                return (false, vec![]);
            }

            // If we are moving vertically, we also need to try to push the other half of this box
            if dy != 0 {
                let other_position = Position {
                    x: next_position.x + 1,
                    y: next_position.y,
                };
                let (other_can_push, other_other_new_positions) =
                    try_push_blocks(other_position, map, direction);
                if other_can_push {
                    new_positions.extend(other_other_new_positions);
                } else {
                    return (false, vec![]);
                }
            }

            (can_push, new_positions)
        }
        Some(Object::RightBox) => {
            let (can_push, other_new_positions) = try_push_blocks(next_position, map, direction);
            if can_push {
                new_positions.extend(other_new_positions);
            } else {
                return (false, vec![]);
            }

            // If we are moving vertically, we also need to try to push the other half of this box
            if dy != 0 {
                let other_position = Position {
                    x: next_position.x - 1,
                    y: next_position.y,
                };
                let (other_can_push, other_other_new_positions) =
                    try_push_blocks(other_position, map, direction);
                if other_can_push {
                    new_positions.extend(other_other_new_positions);
                } else {
                    return (false, vec![]);
                }
            }

            (can_push, new_positions)
        }
    }
}

fn expand_map(map: &Map, position: Position) -> (Map, Position) {
    // Expand by doubling in the x direction
    let mut new_map = Map::new();
    for (p, o) in map.iter() {
        let new_p = Position { x: p.x * 2, y: p.y };
        let new_extra_p = Position {
            x: p.x * 2 + 1,
            y: p.y,
        };
        match o {
            Object::Wall => {
                new_map.insert(new_p, Object::Wall);
                new_map.insert(new_extra_p, Object::Wall);
            }
            Object::Box => {
                new_map.insert(new_p, Object::LeftBox);
                new_map.insert(new_extra_p, Object::RightBox);
            }
            Object::LeftBox => panic!("LeftBox not implemented"),
            Object::RightBox => panic!("RightBox not implemented"),
        }
    }
    let new_position = Position {
        x: position.x * 2,
        y: position.y,
    };
    (new_map, new_position)
}

pub fn part_one(input: &str) -> Option<usize> {
    let (mut map, mut position, directions) = parse_input(input);
    for direction in directions {
        position = move_robot(&mut map, position, direction);
    }
    let score = calc_score(&map);

    Some(score)
}

pub fn part_two(input: &str) -> Option<usize> {
    let (map, position, directions) = parse_input(input);
    let (mut map, mut position) = expand_map(&map, position);
    for direction in directions {
        position = move_robot(&mut map, position, direction);
    }
    let score = calc_score(&map);

    Some(score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9021));
    }

    #[test]
    fn test_part_one_smaller_example() {
        let input = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";
        let result = part_one(input);
        assert_eq!(result, Some(2028));
    }

    #[test]
    fn test_move_robot() {
        let input = "########
#.@O.O.#
##..O..#
#...O..#
#.#.O..#
#...O..#
#......#
########";
        let (mut map, position) = read_map(input);
        let direction = Direction::Right;
        assert_eq!(position, Position { x: 2, y: 1 });
        let new_position = move_robot(&mut map, position, direction);
        assert_eq!(new_position, Position { x: 3, y: 1 });
    }
    #[test]
    fn test_move_robot_2() {
        let input = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########";
        let (mut map, position) = read_map(input);
        let direction = Direction::Left;
        assert_eq!(position, Position { x: 2, y: 2 });
        let new_position = move_robot(&mut map, position, direction);
        // Shouldn't be able to move
        assert_eq!(new_position, Position { x: 2, y: 2 });
    }

    #[test]
    fn test_part_2_small_example() {
        let input = "#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^";
        let result = part_two(input);
        assert_eq!(result, Some(618));
    }
}
