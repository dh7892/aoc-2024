advent_of_code::solution!(16);

use pathfinding::prelude::dijkstra;
use pathfinding::prelude::yen;
use std::collections::HashSet;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Node {
    x: usize,
    y: usize,
    direction: Direction,
}

// Our map is just a set of coordinates that are valid to walk on.
type Map = HashSet<(usize, usize)>;

fn change_for_direction(direction: &Direction) -> (isize, isize) {
    match direction {
        Direction::North => (0, -1),
        Direction::East => (1, 0),
        Direction::South => (0, 1),
        Direction::West => (-1, 0),
    }
}
fn successors(node: &Node, map: &Map) -> Vec<(Node, usize)> {
    let mut result = Vec::new();
    let (dx, dy) = change_for_direction(&node.direction);
    let new_x = node.x as isize + dx;
    let new_y = node.y as isize + dy;
    if map.contains(&(new_x as usize, new_y as usize)) {
        result.push((
            Node {
                x: new_x as usize,
                y: new_y as usize,
                direction: node.direction.clone(),
            },
            1, // Costs 1 to move forwards
        ));
    }
    // Now try turning
    match node.direction {
        Direction::North | Direction::South => {
            result.push((
                Node {
                    x: node.x,
                    y: node.y,
                    direction: Direction::East,
                },
                1000, // Costs 1000 to turn
            ));
            result.push((
                Node {
                    x: node.x,
                    y: node.y,
                    direction: Direction::West,
                },
                1000, // Costs 1000 to turn
            ));
        }
        Direction::East | Direction::West => {
            result.push((
                Node {
                    x: node.x,
                    y: node.y,
                    direction: Direction::North,
                },
                1000, // Costs 1000 to turn
            ));
            result.push((
                Node {
                    x: node.x,
                    y: node.y,
                    direction: Direction::South,
                },
                1000, // Costs 1000 to turn
            ));
        }
    }

    result
}

fn input_to_map(input: &str) -> (Map, (usize, usize), (usize, usize)) {
    // Generate our map and start and end coordinates.
    let mut start = (0, 0);
    let mut end = (0, 0);
    let mut map = Map::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '.' {
                map.insert((x, y));
            }
            if c == 'S' {
                start = (x, y);
                map.insert((x, y));
            }
            if c == 'E' {
                end = (x, y);
                map.insert((x, y));
            }
        }
    }
    (map, start, end)
}

fn solve(map: &Map, start: (usize, usize), end: (usize, usize)) -> Option<usize> {
    let start_node = Node {
        x: start.0,
        y: start.1,
        direction: Direction::East,
    };
    // Could end facing either North or East, so we'll need to try both and see which is shorter.
    let result = dijkstra(
        &start_node,
        |node| successors(node, map),
        |node| (node.x == end.0 && node.y == end.1),
    )?;
    Some(result.1)
}

pub fn part_one(input: &str) -> Option<usize> {
    let (map, start, end) = input_to_map(input);
    solve(&map, start, end)
}

pub fn part_two(input: &str) -> Option<usize> {
    let (map, start, end) = input_to_map(input);
    let start_node = Node {
        x: start.0,
        y: start.1,
        direction: Direction::East,
    };
    // Could end facing either North or East, so we'll need to try both and see which is shorter.
    let result = yen(
        &start_node,
        |node| successors(node, &map),
        |node| (node.x == end.0 && node.y == end.1),
        100,
    );
    let shortest_cost = result[0].1;
    let optimal_paths = result
        .iter()
        .filter(|r| r.1 == shortest_cost)
        .map(|r| r.0.clone())
        .collect::<Vec<_>>();
    println!("{:?}", optimal_paths.len());
    let mut node_locations: HashSet<(usize, usize)> = HashSet::new();
    for path in optimal_paths {
        for node in path {
            node_locations.insert((node.x, node.y));
        }
    }
    Some(node_locations.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7036));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
