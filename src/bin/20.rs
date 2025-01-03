advent_of_code::solution!(20);

use pathfinding::prelude::astar;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Node {
    position: Position,
}

fn input_to_map(input: &str) -> (HashSet<Position>, Position, Position) {
    // Return a map or the wall positions plus the start and end positions
    let mut start = Position { x: 0, y: 0 };
    let mut end = Position { x: 0, y: 0 };
    let mut map = HashSet::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '#' => {
                    map.insert(Position { x, y });
                }
                'S' => {
                    start = Position { x, y };
                }
                'E' => {
                    end = Position { x, y };
                }
                _ => {}
            }
        }
    }
    (map, start, end)
}

fn move_options(node: &Node, max: &Position) -> Vec<Node> {
    let mut result = Vec::new();
    // Options without checking for walls, but checking for out of bounds
    let mut deltas = Vec::new();
    if node.position.x > 0 {
        deltas.push((-1, 0));
    }
    if node.position.y > 0 {
        deltas.push((0, -1));
    }
    if node.position.x < max.x {
        deltas.push((1, 0));
    }
    if node.position.y < max.y {
        deltas.push((0, 1));
    }
    for delta in deltas {
        let new_position = Position {
            x: (node.position.x as isize + delta.0) as usize,
            y: (node.position.y as isize + delta.1) as usize,
        };
        result.push(Node {
            position: new_position,
        });
    }

    result
}

fn successors(node: &Node, map: &HashSet<Position>) -> Vec<(Node, usize)> {
    let mut result = Vec::new();
    let max = map
        .iter()
        .fold(Position { x: 0, y: 0 }, |acc, pos| Position {
            x: acc.x.max(pos.x),
            y: acc.y.max(pos.y),
        });

    for next_move in move_options(node, &max) {
        if !map.contains(&next_move.position) {
            result.push((next_move, 1));
        }
    }

    result
}

fn find_path(map: &HashSet<Position>, start: Node, end: &Position) -> Option<Vec<Node>> {
    let result = astar(
        &start,
        |node| successors(node, map),
        |_| 0,
        |node| node.position == *end,
    );
    result.map(|x| x.0)
}

fn locations_n_away(
    map: &HashSet<Position>,
    start: &Position,
    n: usize,
    max: &Position,
) -> Vec<Position> {
    // Return a list of all positions that are on a path and have a manhattan distance of n
    // from the starting point
    // We do this by checking all points in a diamond shape around the starting point
    let mut result = Vec::new();

    // From left point to top point
    let min_x = start.x as isize - n as isize;
    let max_x = start.x as isize + n as isize;
    for x in min_x..=max_x {
        let dy = n as isize - (start.x as isize - x).abs();
        let y1 = start.y as isize + dy;
        if y1 >= 0 && y1 < max.y as isize {
            let pos = Position {
                x: x.max(0) as usize,
                y: y1 as usize,
            };
            if !map.contains(&pos) {
                result.push(pos);
            }
        }
        if dy > 0 {
            let y2 = start.y as isize - dy;
            if y2 >= 0 && y2 < max.y as isize {
                let pos = Position {
                    x: x.max(0) as usize,
                    y: y2 as usize,
                };
                if !map.contains(&pos) {
                    result.push(pos);
                }
            }
        }
    }

    result
}

fn try_cheat_distance(
    map: &HashSet<Position>,
    start: &Node,
    path: &[Node],
    path_hash: &HashMap<Node, usize>,
    cheat_distance: usize,
    max: &Position,
) -> Vec<usize> {
    let possible_positions = locations_n_away(map, &start.position, cheat_distance, max);
    let mut result = Vec::new();
    let length = path.len();
    let current_distance_to_end = length - path.iter().position(|x| x == start).unwrap();

    for pos in possible_positions {
        let node = Node { position: pos };
        if path_hash.contains_key(&node) {
            let path_posn = path_hash[&node];
            let distance_to_end = length - path_posn + cheat_distance;
            // println!(
            //     "Start: {:?}, Path: {:?}, Cheat: {:?}, Distance: {}, Current: {}",
            //     start.position, path_posn, pos, distance_to_end, current_distance_to_end
            // );
            if distance_to_end < current_distance_to_end {
                result.push(current_distance_to_end - distance_to_end);
            }
        }
    }
    result
}

fn try_all_cheats_along_path(
    map: &HashSet<Position>,
    path: Vec<Node>,
    end: &Position,
    max: &Position,
    max_cheat_distance: usize,
) -> Vec<usize> {
    let mut result = Vec::new();
    let path_hash_set: HashMap<Node, usize> = path
        .iter()
        .enumerate()
        .map(|(i, node)| (*node, i))
        .collect();
    for node in path.iter() {
        if node.position == *end {
            break;
        }
        for cheat_distance in 2..=max_cheat_distance {
            let cheats = try_cheat_distance(map, node, &path, &path_hash_set, cheat_distance, max);
            result.extend(cheats);
        }
    }
    result
}

pub fn part_one(input: &str) -> Option<usize> {
    let (map, start, end) = input_to_map(input);
    let start = Node { position: start };
    let path_without_cheating = find_path(&map, start, &end).unwrap();
    let max = map
        .iter()
        .fold(Position { x: 0, y: 0 }, |acc, pos| Position {
            x: acc.x.max(pos.x),
            y: acc.y.max(pos.y),
        });
    let cheat_path_savings = try_all_cheats_along_path(&map, path_without_cheating, &end, &max, 2);
    let target = 100;
    let result = cheat_path_savings.iter().filter(|x| **x >= target).count();

    Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    let (map, start, end) = input_to_map(input);
    let start = Node { position: start };
    let path_without_cheating = find_path(&map, start, &end).unwrap();
    let max = map
        .iter()
        .fold(Position { x: 0, y: 0 }, |acc, pos| Position {
            x: acc.x.max(pos.x),
            y: acc.y.max(pos.y),
        });
    let cheat_path_savings = try_all_cheats_along_path(&map, path_without_cheating, &end, &max, 20);
    let target = 100;
    let result = cheat_path_savings.iter().filter(|x| **x >= target).count();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = &advent_of_code::template::read_file("examples", DAY);
        let (map, start, end) = input_to_map(input);
        let start = Node { position: start };
        let path_without_cheating = find_path(&map, start, &end).unwrap();
        let len_path_without_cheating = path_without_cheating.len() - 1;

        assert_eq!(len_path_without_cheating, 84);

        // Check that if we start cheating from a certain position, we get the right result
        let cheat_posn = path_without_cheating[12].position;
        assert_eq!(cheat_posn, Position { x: 7, y: 1 });
        let cheat_start = Node {
            position: cheat_posn,
        };

        let max = map
            .iter()
            .fold(Position { x: 0, y: 0 }, |acc, pos| Position {
                x: acc.x.max(pos.x),
                y: acc.y.max(pos.y),
            });

        let path_hash: HashMap<Node, usize> = path_without_cheating
            .iter()
            .enumerate()
            .map(|(i, node)| (*node, i))
            .collect();
        let cheat_path = try_cheat_distance(
            &map,
            &cheat_start,
            &path_without_cheating,
            &path_hash,
            2,
            &max,
        );
        assert_eq!(cheat_path, vec![12]);

        let all_cheat_paths = try_all_cheats_along_path(&map, path_without_cheating, &end, &max, 2);
        assert_eq!(all_cheat_paths.len(), 44);
    }

    #[test]
    fn test_locations_n_away() {
        let input = &advent_of_code::template::read_file("examples", DAY);
        let (map, start, _) = input_to_map(input);
        let max = map
            .iter()
            .fold(Position { x: 0, y: 0 }, |acc, pos| Position {
                x: acc.x.max(pos.x),
                y: acc.y.max(pos.y),
            });
        let result = locations_n_away(&map, &start, 2, &max);
        assert_eq!(result.len(), 2);
        let other_posn = Position { x: 5, y: 3 };
        let result = locations_n_away(&map, &other_posn, 2, &max);
        assert_eq!(result.len(), 3);
    }
    #[test]
    fn test_locations_n_away_again() {
        let input = &advent_of_code::template::read_file("examples", DAY);
        let (map, start, _) = input_to_map(input);
        let max = map
            .iter()
            .fold(Position { x: 0, y: 0 }, |acc, pos| Position {
                x: acc.x.max(pos.x),
                y: acc.y.max(pos.y),
            });
        let result = locations_n_away(&map, &start, 2, &max);
        assert_eq!(result.len(), 2);
        let other_posn = Position { x: 9, y: 7 };
        let result = locations_n_away(&map, &other_posn, 2, &max);
        assert_eq!(result.len(), 4);
    }
}
