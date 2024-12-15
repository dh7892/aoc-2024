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

struct Map {
    map: HashMap<Position, Object>,
    max: Position,
}

impl Map {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
            max: Position { x: 0, y: 0 },
        }
    }

    fn add(&mut self, position: Position, object: Object) {
        self.map.insert(position, object);
        // Update the max position
        if position.x > self.max.x {
            self.max.x = position.x;
        }
        if position.y > self.max.y {
            self.max.y = position.y;
        }
    }

    fn get(&self, position: Position) -> Option<&Object> {
        self.map.get(&position)
    }

    fn max(&self) -> Position {
        self.max
    }

    // Debug
    fn print(&self, robot_position: Position) {
        let mut min_x = 0;
        let mut max_x = 0;
        let mut min_y = 0;
        let mut max_y = 0;

        for position in self.map.keys() {
            min_x = min_x.min(position.x);
            max_x = max_x.max(position.x);
            min_y = min_y.min(position.y);
            max_y = max_y.max(position.y);
        }

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                if (robot_position == Position { x, y }) {
                    print!("@");
                    continue;
                }
                match self.get(Position { x, y }) {
                    Some(Object::Wall) => print!("#"),
                    Some(Object::Box) => print!("O"),
                    None => print!("."),
                }
            }
            println!();
        }
    }
    fn iter(&self) -> std::collections::hash_map::Iter<Position, Object> {
        self.map.iter()
    }
}

fn read_map(input: &str) -> (Map, Position) {
    // Read the map and the position of the robot
    let mut position = Position { x: 0, y: 0 };
    let mut map = Map::new();

    let mut y = 0;
    for line in input.lines() {
        let mut x = 0;
        for c in line.chars() {
            match c {
                '#' => map.add(Position { x, y }, Object::Wall),
                'O' => map.add(Position { x, y }, Object::Box),
                '@' => position = Position { x, y },
                _ => {}
            }
            x += 1;
        }
        y += 1;
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

fn steps_to_nearest_gap(map: &Map, position: Position, direction: &Direction) -> usize {
    // Return the number of steps to the nearest gap (not wall or box) in the given direction
    // If there is no gap, return 0
    let mut distance: usize = 0;
    let (dx, dy) = (direction.dx(), direction.dy());

    loop {
        distance += 1;
        let p = Position {
            x: position.x + dx * distance as i32,
            y: position.y + dy * distance as i32,
        };
        if map.get(p) == None {
            break;
        }

        if p.x < 0 || p.y < 0 || p.x > map.max().x || p.y > map.max().y {
            return 0;
        }

        if map.get(p) == Some(&Object::Wall) {
            return 0;
        }
    }

    distance
}

fn calc_score(box_map: &Map) -> usize {
    // Calculate the score of the map
    // Sum up the GPS score for each box
    // The GPS score is y*100 + x
    box_map
        .iter()
        .filter(|(_, o)| **o == Object::Box)
        .map(|(p, _)| (p.y * 100 + p.x) as usize)
        .sum()
}

fn move_robot(map: &mut Map, position: Position, direction: Direction) -> Position {
    // Move the robot in the given direction
    // If the robot can't move, return the original position

    // Check where our nearest gap is
    let distance = steps_to_nearest_gap(map, position, &direction);
    if distance == 0 {
        // Can't move anything as there are no gaps
        return position;
    }

    let (dx, dy) = (direction.dx(), direction.dy());

    // Now loop through the steps and move the box if there is one
    // (Loop backwards so we don't try to move a box on top of another box)
    for i in (1..distance).rev() {
        let p = Position {
            x: position.x + dx * i as i32,
            y: position.y + dy * i as i32,
        };
        // Remove the old box
        map.map.remove(&p);
        // Add the new box
        map.add(
            Position {
                x: p.x + dx,
                y: p.y + dy,
            },
            Object::Box,
        );
    }

    Position {
        x: position.x + dx,
        y: position.y + dy,
    }
}

fn try_push_blocks(
    position: Position,
    map: &Map,
    direction: Direction,
) -> (bool, Vec<(Position, Position)>) {
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

    let mut new_positions = vec![(position, next_position)];
    match map.get(next_position) {
        Some(Object::Wall) => (false, vec![]),
        None => (true, new_positions),
        Some(Object::Box) => {
            let (can_push, other_new_positions) = can_push_blocks(next_position, map, direction);
            if can_push {
                new_positions.extend(other_new_positions);
            } else {
                return (false, vec![]);
            }

            (can_push, new_positions)
        }
    }
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
    None
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
        assert_eq!(result, None);
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
}
