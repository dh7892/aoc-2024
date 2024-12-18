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

fn exponential_search(all_obstacles: &[Position], max: &Position, start: usize) -> Position {
    let mut last_successful = start;
    let mut delta = start;
    // let mut iterations = 0;
    while delta > 1 {
        // iterations += 1;
        delta /= 2;
        let next = last_successful + delta;
        let has_solution = has_solution(&all_obstacles.iter().take(next).cloned().collect(), max);
        if has_solution {
            last_successful = next;
            // Since we found a solution, we can increase the delta
            // We'd already halved it so multiply by for to achieve double the original
            delta *= 4;
        }
    }
    // println!("Iterations: {}, value {}", iterations, last_successful);
    all_obstacles[last_successful].clone()
}

fn binary_search(all_obstacles: &[Position], max: &Position, start: usize) -> Position {
    let mut low = start;
    let mut high = all_obstacles.len();
    while low < high {
        let mid = (low + high) / 2;
        let has_solution = has_solution(&all_obstacles.iter().take(mid).cloned().collect(), max);
        if has_solution {
            low = mid + 1;
        } else {
            high = mid;
        }
    }
    all_obstacles[low - 1].clone()
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
    let position = binary_search(&obstacles, &Position { x: 70, y: 70 }, start);
    let result = format!("{},{}", position.x, position.y);
    Some(result)
}
