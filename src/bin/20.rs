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

fn find_best_path(map: &HashSet<Position>, start: Node, end: &Position) -> Option<Vec<Node>> {
    let result = astar(
        &start,
        |node| successors(node, map),
        |_| 0, // Don't care about the heuristic
        |node| node.position == *end,
    );
    result.map(|(path, _)| path)
}

fn try_cheat(
    map: &HashSet<Position>,
    start: Node,
    end: &Position,
    max: &Position,
    cache: &mut HashMap<Node, Vec<usize>>,
) -> Vec<usize> {
    // From a given start point, return the best paths
    // that start from a wall position next to our start point
    //
    if let Some(cached) = cache.get(&start) {
        return cached.clone();
    }
    let possible_moves = move_options(&start, max);
    let mut result = Vec::new();
    for next_move in possible_moves {
        if map.contains(&next_move.position) {
            // We can cheat from here
            let path = find_best_path(map, next_move, end);
            if let Some(path) = path {
                result.push(path.len());
            }
        }
    }
    cache.insert(start, result.clone());
    result
}

fn try_all_cheats_along_path(
    map: &HashSet<Position>,
    path: Vec<Node>,
    end: &Position,
    max: &Position,
    cache: &mut HashMap<Node, Vec<usize>>,
) -> Vec<usize> {
    // Given a valid path, try starting from each point in turn and cheating from that point
    // Return the best path length for each of those starting points
    path.iter()
        .enumerate()
        .flat_map(|(i, node)| {
            try_cheat(map, *node, end, max, cache)
                .into_iter()
                .map(|x| x + i)
                .filter(|x| *x <= path.len()) // Only consider paths that are shorter than the
                // original
                // .inspect(|x| println!("{}: {}", i, x))
                .collect::<Vec<usize>>()
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<usize> {
    let (map, start, end) = input_to_map(input);
    let start = Node { position: start };
    let max = map
        .iter()
        .fold(Position { x: 0, y: 0 }, |acc, pos| Position {
            x: acc.x.max(pos.x),
            y: acc.y.max(pos.y),
        });
    let path_without_cheating = find_best_path(&map, start, &end)?;
    let len_path_without_cheating = path_without_cheating.len() - 1;
    println!("Path without cheating: {}", len_path_without_cheating);
    let mut cache = HashMap::new();
    let cheat_paths =
        try_all_cheats_along_path(&map, path_without_cheating, &end, &max, &mut cache);

    let count = cheat_paths.len();
    Some(count)
}

pub fn part_two(input: &str) -> Option<usize> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = &advent_of_code::template::read_file("examples", DAY);
        let (map, start, end) = input_to_map(input);
        let start = Node { position: start };
        let path_without_cheating = find_best_path(&map, start, &end).unwrap();
        let len_path_without_cheating = path_without_cheating.len() - 1;

        assert_eq!(len_path_without_cheating, 84);

        // Check that if we start cheating from a certain position, we get the right result
        let cheat_posn = path_without_cheating[12].position;
        assert_eq!(cheat_posn, Position { x: 7, y: 1 });
        let cheat_start = Node {
            position: cheat_posn,
        };
        let cheat_path = find_best_path(&map, cheat_start, &end).unwrap();
        let len_cheat_path = cheat_path.len() + 12;
        assert_eq!(len_cheat_path - 1, 72);
        let max = map
            .iter()
            .fold(Position { x: 0, y: 0 }, |acc, pos| Position {
                x: acc.x.max(pos.x),
                y: acc.y.max(pos.y),
            });

        let all_cheat_paths = try_all_cheats_along_path(&map, path_without_cheating, &end, &max);
        let savings = vec![
            //(time_saved, number_of_ways_to achieve)
            (2, 14),
            (4, 14),
            (6, 2),
            (8, 4),
            (10, 2),
            (12, 3),
            (20, 1),
            (36, 1),
            (38, 1),
            (40, 1),
            (64, 1),
        ];
        for (saving, expected_count) in savings {
            let count = all_cheat_paths
                .iter()
                .map(|x| len_path_without_cheating - x)
                .filter(|x| *x == saving)
                .count();
            assert_eq!(count, expected_count);
        }
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
