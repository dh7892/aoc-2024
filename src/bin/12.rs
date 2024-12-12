advent_of_code::solution!(12);

use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum FencePosition {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coordinate {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Plot {
    id: Option<usize>,
    crop: char,
    perimeter: usize,
    fence_positions: HashSet<FencePosition>,
}

fn input_to_map(input: &str) -> (HashMap<Coordinate, Plot>, Coordinate) {
    let mut max = Coordinate { x: 0, y: 0 };
    let mut map = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        if y > max.y {
            max.y = y;
        }
        for (x, c) in line.chars().enumerate() {
            if x > max.x {
                max.x = x;
            }
            let plot = Plot {
                id: None,
                crop: c,
                perimeter: 0,
                fence_positions: HashSet::new(),
            };
            map.insert(Coordinate { x, y }, plot);
        }
    }
    (map, max)
}

fn possible_neighbour_coords(
    coord: Coordinate,
    max: Coordinate,
) -> Vec<(Coordinate, FencePosition)> {
    let mut neighbors = vec![];
    if coord.x > 0 {
        neighbors.push((
            Coordinate {
                x: coord.x - 1,
                y: coord.y,
            },
            FencePosition::West,
        ));
    }
    if coord.x < max.x {
        neighbors.push((
            Coordinate {
                x: coord.x + 1,
                y: coord.y,
            },
            FencePosition::East,
        ));
    }
    if coord.y > 0 {
        neighbors.push((
            Coordinate {
                x: coord.x,
                y: coord.y - 1,
            },
            FencePosition::North,
        ));
    }
    if coord.y < max.y {
        neighbors.push((
            Coordinate {
                x: coord.x,
                y: coord.y + 1,
            },
            FencePosition::South,
        ));
    }
    neighbors
}

fn group_plots(map: &mut HashMap<Coordinate, Plot>, max: Coordinate) {
    let mut id = 0;
    let coords: Vec<Coordinate> = map.keys().cloned().collect();

    for coord in coords {
        if let Some(plot) = map.get(&coord) {
            if plot.id.is_none() {
                id += 1;
                let crop = plot.crop;
                let mut stack = vec![coord];

                while let Some(c) = stack.pop() {
                    // Mark the current plot
                    if let Some(p) = map.get_mut(&c) {
                        p.id = Some(id);
                    }

                    // Find neighbors to process
                    let neighbors = possible_neighbour_coords(c, max);
                    for n in neighbors {
                        let (coord, _) = n;

                        if let Some(p) = map.get(&coord) {
                            if p.id.is_none() && p.crop == crop {
                                stack.push(coord);
                            }
                        }
                    }
                }
            }
        }
    }
}

fn mark_fences(map: &mut HashMap<Coordinate, Plot>, max: Coordinate) {
    let coords: Vec<Coordinate> = map.keys().cloned().collect();
    for coord in coords {
        let crop = map.get(&coord).unwrap().crop;
        let mut fence_positions = HashSet::new();
        // Try North
        if coord.y > 0 {
            let north = Coordinate {
                x: coord.x,
                y: coord.y - 1,
            };
            if map.get(&north).unwrap().crop != crop {
                fence_positions.insert(FencePosition::North);
            }
        } else {
            fence_positions.insert(FencePosition::North);
        }
        // Try East
        if coord.x < max.x {
            let east = Coordinate {
                x: coord.x + 1,
                y: coord.y,
            };
            if map.get(&east).unwrap().crop != crop {
                fence_positions.insert(FencePosition::East);
            }
        } else {
            fence_positions.insert(FencePosition::East);
        }
        // Try South
        if coord.y < max.y {
            let south = Coordinate {
                x: coord.x,
                y: coord.y + 1,
            };
            if map.get(&south).unwrap().crop != crop {
                fence_positions.insert(FencePosition::South);
            }
        } else {
            fence_positions.insert(FencePosition::South);
        }
        // Try West
        if coord.x > 0 {
            let west = Coordinate {
                x: coord.x - 1,
                y: coord.y,
            };
            if map.get(&west).unwrap().crop != crop {
                fence_positions.insert(FencePosition::West);
            }
        } else {
            fence_positions.insert(FencePosition::West);
        }
        let plot = map.get_mut(&coord).unwrap();
        plot.fence_positions = fence_positions.clone();
        plot.perimeter = fence_positions.len();
    }
}

fn score(map: &HashMap<Coordinate, Plot>) -> usize {
    // A Field has an area and a perimeter indexed by id
    let mut fields = HashMap::new();
    for plot in map.values() {
        if let Some(id) = plot.id {
            let field = fields.entry(id).or_insert((0, 0));
            field.0 += 1;
            field.1 += plot.perimeter;
        }
    }
    fields.values().map(|(a, p)| a * p).sum()
}

fn iter_direction_for_fence_position(position: FencePosition) -> Coordinate {
    // Which direction to move when following fences of a certain position
    match position {
        FencePosition::North => Coordinate { x: 1, y: 0 },
        FencePosition::East => Coordinate { x: 0, y: 1 },
        FencePosition::South => Coordinate { x: 1, y: 0 },
        FencePosition::West => Coordinate { x: 0, y: 1 },
    }
}

fn upsert_hashmap(sides: &mut HashMap<usize, usize>, id: usize) {
    // Update or insert anything that's indexed by id and needs to be incremented
    // This just saves a lot of boilerplate
    sides.entry(id).and_modify(|old| *old += 1).or_insert(1);
}

fn find_sides(
    map: &HashMap<Coordinate, Plot>,
    max: Coordinate,
    start: Coordinate,
    position: FencePosition,
) -> HashMap<usize, usize> {
    // Start at a point, move in the diection until we move off the map
    // count up sides per field indexed by id
    let iter = iter_direction_for_fence_position(position);
    let mut sides: HashMap<usize, usize> = HashMap::new();
    let mut last_had_side = false;
    let mut current_coordinate = start;
    let mut prev_id = map.get(&start).unwrap().id.unwrap();
    while current_coordinate.x <= max.x && current_coordinate.y <= max.y {
        let plot = map.get(&current_coordinate).unwrap();
        if plot.id.unwrap() != prev_id {
            // Field changed so we need to add the last side if there was one
            if last_had_side {
                upsert_hashmap(&mut sides, prev_id);
            }
        } else if last_had_side && !plot.fence_positions.contains(&position) {
            // Moved off the end of a fence
            upsert_hashmap(&mut sides, prev_id);
        }
        last_had_side = plot.fence_positions.contains(&position);
        current_coordinate.x += iter.x;
        current_coordinate.y += iter.y;
        prev_id = plot.id.unwrap();
    }
    // Add in the last side we were counting
    if last_had_side {
        upsert_hashmap(&mut sides, prev_id);
    }
    sides
}

fn find_all_sides(map: &HashMap<Coordinate, Plot>, max: Coordinate) -> HashMap<usize, usize> {
    // Given a map, return the number of sides for each field indexed by id
    // find North and South sides
    let mut sides_vec = vec![];
    for y in 0..=max.y {
        let start = Coordinate { x: 0, y };
        let north_sides = find_sides(map, max, start, FencePosition::North);
        sides_vec.push(north_sides);
        let south_sides = find_sides(map, max, start, FencePosition::South);
        sides_vec.push(south_sides);
    }

    // find East and West sides
    for x in 0..=max.x {
        let start = Coordinate { x, y: 0 };
        let east_sides = find_sides(map, max, start, FencePosition::East);
        sides_vec.push(east_sides);
        let west_sides = find_sides(map, max, start, FencePosition::West);
        sides_vec.push(west_sides);
    }

    // Compact the sides
    let mut sides = HashMap::new();
    for side in sides_vec {
        for (id, count) in side {
            sides
                .entry(id)
                .and_modify(|old| *old += count)
                .or_insert(count);
        }
    }

    sides
}

fn score_with_sides(map: &HashMap<Coordinate, Plot>, sides: HashMap<usize, usize>) -> usize {
    // Work out areas of fields
    let mut field_sizes = HashMap::new();
    for plot in map.values() {
        if let Some(id) = plot.id {
            upsert_hashmap(&mut field_sizes, id);
        }
    }
    // now we have fields sizes and sides, both indexed by id so we can calculate the score
    field_sizes
        .iter()
        .map(|(id, size)| (size * sides.get(id).unwrap()))
        .sum()
}

pub fn part_one(input: &str) -> Option<usize> {
    let (mut map, max) = input_to_map(input);
    group_plots(&mut map, max);
    mark_fences(&mut map, max);
    Some(score(&map))
}

pub fn part_two(input: &str) -> Option<usize> {
    let (mut map, max) = input_to_map(input);
    group_plots(&mut map, max);
    mark_fences(&mut map, max);
    let sides = find_all_sides(&map, max);

    Some(score_with_sides(&map, sides))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }

    #[test]
    fn test_find_sides_in_row() {
        let input = &advent_of_code::template::read_file("examples", DAY);
        let (mut map, max) = input_to_map(input);
        group_plots(&mut map, max);
        mark_fences(&mut map, max);
        let direction = FencePosition::North;
        let start = Coordinate { x: 0, y: 0 };

        let sides = find_sides(&map, max, start, direction);
        let total_sides = sides.values().sum::<usize>();
        assert_eq!(total_sides, 4);

        let direction = FencePosition::South;
        let sides = find_sides(&map, max, start, direction);
        let total_sides = sides.values().sum::<usize>();
        assert_eq!(total_sides, 1);
    }
    #[test]
    fn test_find_sides_in_row_2() {
        let input = &advent_of_code::template::read_file("examples", DAY);
        let (mut map, max) = input_to_map(input);
        group_plots(&mut map, max);
        mark_fences(&mut map, max);
        let direction = FencePosition::North;
        let start = Coordinate { x: 0, y: 1 };

        let sides = find_sides(&map, max, start, direction);
        let total_sides = sides.values().sum::<usize>();
        assert_eq!(total_sides, 1);

        // South sides
        let direction = FencePosition::South;
        let start = Coordinate { x: 0, y: 1 };

        let sides = find_sides(&map, max, start, direction);
        let total_sides = sides.values().sum::<usize>();
        assert_eq!(total_sides, 3);
    }
    #[test]
    fn test_find_sides_in_col_9() {
        let input = &advent_of_code::template::read_file("examples", DAY);
        let (mut map, max) = input_to_map(input);
        group_plots(&mut map, max);
        mark_fences(&mut map, max);
        let direction = FencePosition::East;
        let start = Coordinate { x: 9, y: 0 };

        let sides = find_sides(&map, max, start, direction);
        let total_sides = sides.values().sum::<usize>();
        assert_eq!(total_sides, 2);

        let direction = FencePosition::West;
        let start = Coordinate { x: 9, y: 0 };

        let sides = find_sides(&map, max, start, direction);
        let total_sides = sides.values().sum::<usize>();
        assert_eq!(total_sides, 2);
    }
    #[test]
    fn test_find_sides_in_col_8() {
        let input = &advent_of_code::template::read_file("examples", DAY);
        let (mut map, max) = input_to_map(input);
        group_plots(&mut map, max);
        mark_fences(&mut map, max);
        let direction = FencePosition::East;
        let start = Coordinate { x: 8, y: 0 };

        let sides = find_sides(&map, max, start, direction);
        let total_sides = sides.values().sum::<usize>();
        assert_eq!(total_sides, 2);

        let direction = FencePosition::West;
        let start = Coordinate { x: 8, y: 0 };

        let sides = find_sides(&map, max, start, direction);
        let total_sides = sides.values().sum::<usize>();
        assert_eq!(total_sides, 3);
    }
}
