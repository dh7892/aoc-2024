advent_of_code::solution!(10);

use std::collections::{HashMap, HashSet};

type Coordinate = (usize, usize);

#[derive(Debug, Clone)]
struct Location {
    height: usize,
    connections: Vec<Coordinate>,
}

type Path = Vec<Coordinate>;

type Map = HashMap<Coordinate, Location>;

fn input_to_map(input: &str) -> Map {
    let mut map = Map::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c.is_digit(10) {
                let d = c.to_digit(10).unwrap() as usize;
                map.insert(
                    (x, y),
                    Location {
                        height: d,
                        connections: vec![],
                    },
                );
            }
        }
    }

    // Need a copy of the map so we can random access elements without the borrow
    // checker getting angry. Only need the heights from this, so it's not a problem
    // that we look at old values
    let old_map = map.clone();

    // Now loop over map and add connections
    for (coord, location) in map.iter_mut() {
        let (x, y) = coord;
        let height = location.height;
        for (dx, dy) in vec![(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let new_x = *x as i32 + dx;
            let new_y = *y as i32 + dy;
            if new_x >= 0 && new_y >= 0 {
                let new_coord = (new_x as usize, new_y as usize);
                if let Some(new_location) = old_map.get(&new_coord) {
                    // Only add a connection if the height is one greater
                    if new_location.height == height + 1 {
                        location.connections.push(new_coord);
                    }
                }
            }
        }
    }
    map
}

fn paths_from_location(map: &Map, location: Coordinate, path: Path) -> Vec<Path> {
    // First, add the current location to the path
    let mut path = path;
    path.push(location);

    let mut paths = Vec::new();
    let location = map.get(&location).unwrap();
    // If we've reached the end of a path
    // Only add it to our valid list if it has reached a height of 9
    if location.connections.is_empty() {
        if location.height == 9 {
            paths.push(path);
        }
        return paths;
    }
    let path = path.clone();
    for connection in location.connections.iter() {
        paths.extend(paths_from_location(map, *connection, path.clone()));
    }
    paths
}

fn paths_to_tops(map: &Map) -> Vec<Path> {
    let mut paths = Vec::new();
    for (coord, _) in map.iter() {
        let location = map.get(coord).unwrap();
        if location.height == 0 {
            let mut top_locations = HashSet::new();
            let new_paths = &mut paths_from_location(map, *coord, Vec::new());
            for path in new_paths {
                if !top_locations.contains(path.last().unwrap()) {
                    top_locations.insert(path.clone().last().unwrap().clone());
                    paths.push(path.clone());
                }
            }
        }
    }
    paths
}
fn all_paths(map: &Map) -> Vec<Path> {
    let mut paths = Vec::new();
    for (coord, _) in map.iter() {
        let location = map.get(coord).unwrap();
        if location.height == 0 {
            let new_paths = &mut paths_from_location(map, *coord, Vec::new());
            for path in new_paths {
                paths.push(path.clone());
            }
        }
    }
    paths
}

pub fn part_one(input: &str) -> Option<usize> {
    let map = input_to_map(input);
    let paths = paths_to_tops(&map);
    // Count paths that reach the top (e.g. height = 9)
    let count = paths.len();
    Some(count)
}

pub fn part_two(input: &str) -> Option<usize> {
    let map = input_to_map(input);
    let paths = all_paths(&map);
    // Count paths that reach the top (e.g. height = 9)
    let count = paths.len();
    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_simple() {
        let input = "0123
1234
8765
9876";
        let map = input_to_map(input);
        let paths = paths_to_tops(&map);
        assert_eq!(paths.len(), 1);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
