advent_of_code::solution!(18);
use pathfinding::prelude::*;
use scan_fmt::scan_fmt;
use std::collections::HashSet;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Node {
    position: Position,
}

type Obstacles = HashSet<Position>;

fn successors(node: &Node, obstacles: &Obstacles, max: &Position) -> Vec<(Node, usize)> {
    let mut result = Vec::new();
    let x = node.position.x;
    let y = node.position.y;
    if x > 0 && !obstacles.contains(&Position { x: x - 1, y }) {
        result.push((
            Node {
                position: Position { x: x - 1, y },
            },
            1,
        ));
    }
    if y > 0 && !obstacles.contains(&Position { x, y: y - 1 }) {
        result.push((
            Node {
                position: Position { x, y: y - 1 },
            },
            1,
        ));
    }
    if x < max.x && !obstacles.contains(&Position { x: x + 1, y }) {
        result.push((
            Node {
                position: Position { x: x + 1, y },
            },
            1,
        ));
    }
    if y < max.y && !obstacles.contains(&Position { x, y: y + 1 }) {
        result.push((
            Node {
                position: Position { x, y: y + 1 },
            },
            1,
        ));
    }
    result
}

fn parse_obstacles(input: &str) -> Vec<Position> {
    input
        .lines()
        .map(|line| {
            let (x, y): (usize, usize) = scan_fmt!(line, "{},{}", usize, usize).unwrap();
            Position { x, y }
        })
        .collect()
}

fn has_solution(obstacles: &Obstacles, max: &Position) -> bool {
    let start = Node {
        position: Position { x: 0, y: 0 },
    };
    let end = Node {
        position: max.clone(),
    };
    dijkstra(
        &start,
        |node| successors(node, obstacles, max),
        |node| *node == end,
    )
    .is_some()
}

fn find_first_block_that_blocks_solution(
    all_obstacles: &[Position],
    max: &Position,
    start: usize,
) -> Position {
    let mut obstacles = Obstacles::new();
    for (i, obstacle) in all_obstacles.iter().enumerate() {
        obstacles.insert(obstacle.clone());
        if i < start {
            continue;
        }

        if !has_solution(&obstacles, max) {
            return obstacle.clone();
        }
    }
    panic!("No solution found");
}

fn exponential_search(all_obstacles: &[Position], max: &Position, start: usize) -> Position {
    let mut last_successful = start;
    let mut delta = start;
    while delta > 1 {
        delta /= 2;
        let next = last_successful + delta;
        let has_solution = has_solution(&all_obstacles.iter().take(next).cloned().collect(), max);
        if has_solution {
            last_successful = next;
            // Since we found a solution, we can increase the delta
            // We'd already halved it so multiply by for to achieve double the original
            delta = delta * 4;
        }
    }
    all_obstacles[last_successful].clone()
}

pub fn part_one(input: &str) -> Option<usize> {
    let obstacles = parse_obstacles(input);
    let obstacles = obstacles.iter().take(1024).cloned().collect::<Obstacles>();
    let start = Node {
        position: Position { x: 0, y: 0 },
    };
    let max = Position { x: 70, y: 70 };
    let end = Node {
        position: max.clone(),
    };
    let result = dijkstra(
        &start,
        |node| successors(node, &obstacles, &max),
        |node| *node == end,
    );

    Some(result?.1)
}

pub fn part_two(input: &str) -> Option<String> {
    let obstacles = parse_obstacles(input);
    let start = 1024;
    let position =
        find_first_block_that_blocks_solution(&obstacles, &Position { x: 70, y: 70 }, start);
    let result = format!("{},{}", position.x, position.y);
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
