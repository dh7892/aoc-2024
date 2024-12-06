advent_of_code::solution!(6);

use rayon::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
enum Entity {
    Empty,              // Nothing to see here
    Obstacle,           // Can't move here
    Visited(Direction), // Already been here and in what direction we were visiting it
}

type Map = HashMap<Position, Entity>;

fn parse_input(input: &str) -> (Map, Position, Direction, Position) {
    // Return the map, the starting position, and the buttom-right position (for size)
    let mut start = Position { x: 0, y: 0 };
    let mut direction = Direction::Down;
    let (mut max_x, mut max_y) = (0, 0);
    let mut map = Map::new();
    for (y, line) in input.lines().enumerate() {
        if y > max_y {
            max_y = y;
        }
        for (x, c) in line.chars().enumerate() {
            if x > max_x {
                max_x = x;
            }
            let position = Position { x, y };
            let entity = match c {
                '.' => Entity::Empty,
                '#' => Entity::Obstacle,
                '>' => {
                    start = position;
                    direction = Direction::Right;
                    Entity::Visited(Direction::Right)
                }
                '<' => {
                    start = position;
                    direction = Direction::Left;
                    Entity::Visited(Direction::Left)
                }
                '^' => {
                    start = position;
                    direction = Direction::Up;
                    Entity::Visited(Direction::Up)
                }
                'v' => {
                    start = position;
                    direction = Direction::Down;
                    Entity::Visited(Direction::Down)
                }

                _ => panic!("Unexpected character: {}", c),
            };
            map.insert(position, entity);
        }
    }
    (map, start, direction, Position { x: max_x, y: max_y })
}

fn next_position(position: Position, direction: Direction, max: Position) -> Option<Position> {
    let (max_x, max_y) = (max.x, max.y);
    // If the next move would take us off the map, return None
    // Note, we use "screen" coordinates, so up is -y
    match direction {
        Direction::Up => {
            if position.y == 0 {
                None
            } else {
                Some(Position {
                    x: position.x,
                    y: position.y - 1,
                })
            }
        }
        Direction::Down => {
            if position.y == max_y {
                None
            } else {
                Some(Position {
                    x: position.x,
                    y: position.y + 1,
                })
            }
        }
        Direction::Left => {
            if position.x == 0 {
                None
            } else {
                Some(Position {
                    x: position.x - 1,
                    y: position.y,
                })
            }
        }
        Direction::Right => {
            if position.x == max_x {
                None
            } else {
                Some(Position {
                    x: position.x + 1,
                    y: position.y,
                })
            }
        }
    }
}

fn print_map(map: &Map, max: Position) {
    for y in 0..=max.y {
        for x in 0..=max.x {
            let position = Position { x, y };
            match map.get(&position).unwrap_or(&Entity::Empty) {
                Entity::Empty => print!("."),
                Entity::Obstacle => print!("#"),
                Entity::Visited(Direction::Down) => print!("v"),
                Entity::Visited(Direction::Left) => print!("<"),
                Entity::Visited(Direction::Right) => print!(">"),
                Entity::Visited(Direction::Up) => print!("^"),
            }
        }
        println!();
    }
}

fn turn_right(direction: Direction) -> Direction {
    match direction {
        Direction::Up => Direction::Right,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
        Direction::Right => Direction::Down,
    }
}

fn make_next_move(
    map: &mut Map,
    position: Position,
    direction: Direction,
    max: Position,
) -> Option<(Position, Direction)> {
    // Make the next move, and return the new position and direction
    // If the next move would take us off the map, return None

    let next = next_position(position, direction, max)?;
    let entity = map.get(&next).unwrap_or(&Entity::Empty);
    match entity {
        Entity::Empty => {
            map.insert(next, Entity::Visited(direction));
            Some((next, direction))
        }
        Entity::Obstacle => {
            // Turn right
            let new_direction = turn_right(direction);
            Some((position, new_direction))
        }
        Entity::Visited(_) => Some((next, direction)),
    }
}

fn next_move_would_cause_loop(
    map: &Map,
    position: Position,
    direction: Direction,
    max: Position,
) -> bool {
    match next_position(position, direction, max) {
        Some(new_position) => {
            let entity = map.get(&new_position).unwrap_or(&Entity::Empty);
            entity == &Entity::Visited(direction)
        }
        None => false,
    }
}

fn move_until_off_map(
    map: &Map,
    position: Position,
    direction: Direction,
    max: Position,
) -> Option<Map> {
    // If we end up leaving the map, we return a copy of the map with all locations visited
    // If we end up in an infinite loop, we return None
    let mut new_map = map.clone();
    let mut position = position;
    let mut direction = direction;
    loop {
        if next_move_would_cause_loop(&new_map, position, direction, max) {
            return None;
        }
        match make_next_move(&mut new_map, position, direction, max) {
            Some((new_position, new_direction)) => {
                position = new_position;
                direction = new_direction;
            }
            None => break,
        }
    }
    Some(new_map.clone())
}

pub fn part_one(input: &str) -> Option<u32> {
    let (map, position, direction, max) = parse_input(input);
    match move_until_off_map(&map, position, direction, max) {
        Some(new_map) => {
            let visited = new_map
                .iter()
                .filter(|(_, entity)| matches!(**entity, Entity::Visited(_)))
                .count();
            return Some(visited as u32);
        }
        None => {
            // We ended up in an infinite loop
            return None;
        }
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let (map, start_position, direction, max) = parse_input(input);
    // Try adding an obstacle at every empyt place in the map in turn
    // and see which options lead to an infinite loop
    //
    // First, solve the map without any obstacles
    // This means we can later limit out trial obstacles to only those that
    // are on the path we took
    let solved_map = move_until_off_map(&map, start_position, direction, max)?;

    let loops = solved_map
        .par_iter()
        .filter(|(obs_position, entity)| {
            if matches!(**entity, Entity::Visited(_)) {
                let mut new_map = map.clone();
                new_map.insert(**obs_position, Entity::Obstacle);
                match move_until_off_map(&new_map, start_position, direction, max) {
                    Some(_) => false,
                    None => true,
                }
            } else {
                false
            }
        })
        .count();

    Some(loops as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
