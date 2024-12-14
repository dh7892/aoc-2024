advent_of_code::solution!(14);

use nalgebra::Vector2;
use scan_fmt::*;

type Vec2 = Vector2<i64>;

use std::collections::HashSet;

#[derive(Debug)]
struct Robot {
    position: Vec2,
    velocity: Vec2,
}

fn move_robot(robot: &mut Robot, max: Vec2) {
    robot.position += robot.velocity;
    robot.position.x = robot.position.x.rem_euclid(max.x);
    robot.position.y = robot.position.y.rem_euclid(max.y);
}

fn parse_input(input: &str) -> Vec<Robot> {
    input
        .lines()
        .map(|line| {
            let (x, y, dx, dy) = scan_fmt!(line, "p={},{} v={},{}", i64, i64, i64, i64).unwrap();
            Robot {
                position: Vec2::new(x, y),
                velocity: Vec2::new(dx, dy),
            }
        })
        .collect()
}

fn calculate_safety_factor(robots: &[Robot], max: Vec2) -> usize {
    let mut top_left = 0;
    let mut bottom_right = 0;
    let mut top_right = 0;
    let mut bottom_left = 0;
    let mid_point = Vec2::new(max.x / 2, max.y / 2);
    for robot in robots {
        // We deliberately ignore the robots that are on the mid point in either direction
        if robot.position.x < mid_point.x && robot.position.y < mid_point.y {
            top_left += 1;
        } else if robot.position.x > mid_point.x && robot.position.y > mid_point.y {
            bottom_right += 1;
        } else if robot.position.x > mid_point.x && robot.position.y < mid_point.y {
            top_right += 1;
        } else if robot.position.x < mid_point.x && robot.position.y > mid_point.y {
            bottom_left += 1;
        }
    }
    top_left * bottom_right * top_right * bottom_left
}

fn might_be_christmas_tree(robots: &[Robot], max: Vec2) -> bool {
    let mut map = HashSet::new();
    for robot in robots.iter() {
        map.insert(robot.position);
    }
    // Look for 10 robots in a row
    let needed_count = 10;
    for row in 0..max.y {
        let mut count = 0;
        for col in 0..max.x - 10 {
            for i in 0..needed_count {
                if map.contains(&Vec2::new(col + i, row)) {
                    count += 1;
                } else {
                    count = 0;
                    break;
                }
            }
            if count >= 10 {
                return true;
            }
        }
    }
    false
}

fn print_map(robots: &[Robot], max: Vec2) {
    let mut map = HashSet::new();
    for robot in robots.iter() {
        map.insert(robot.position);
    }
    for y in 0..max.y {
        for x in 0..max.x {
            if map.contains(&Vec2::new(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn find_tree(robots: &mut [Robot], max: Vec2) -> usize {
    // Keep looping in time
    // If we find a possible tree
    // print out the current map and ask the user if it is a tree
    let cutoff = 100000000;
    for t in 0..cutoff {
        for robot in robots.iter_mut() {
            move_robot(robot, max);
        }

        if might_be_christmas_tree(robots, max) {
            print_map(robots, max);
            println!("Is this a tree (time: {})? (y/n)", t);
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            if input.trim() == "y" {
                return t;
            }
        };
    }
    0
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut robots = parse_input(input);
    let max = Vec2::new(101, 103);
    for _ in 0..100 {
        for robot in robots.iter_mut() {
            move_robot(robot, max);
        }
    }
    Some(calculate_safety_factor(&robots, max))
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut robots = parse_input(input);
    let max = Vec2::new(101, 103);
    let time = find_tree(&mut robots, max);
    Some(time)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = &advent_of_code::template::read_file("examples", DAY);
        let mut robots = parse_input(input);
        println!("--------------");
        let max = Vec2::new(11, 7);
        for _ in 0..100 {
            for robot in robots.iter_mut() {
                move_robot(robot, max);
            }
        }
        let result = Some(calculate_safety_factor(&robots, max));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_mid_point() {
        let max = Vec2::new(101, 103);
        let mid_point = Vec2::new(max.x / 2, max.y / 2);
        assert_eq!(mid_point, Vec2::new(50, 51));
    }

    #[test]
    fn test_move_robot() {
        let mut robot = Robot {
            position: Vec2::new(0, 0),
            velocity: Vec2::new(1, -1),
        };
        let max = Vec2::new(2, 2);
        move_robot(&mut robot, max);
        assert_eq!(robot.position, Vec2::new(1, 1));
        move_robot(&mut robot, max);
        assert_eq!(robot.position, Vec2::new(0, 0));
        move_robot(&mut robot, max);
        assert_eq!(robot.position, Vec2::new(1, 1));
    }
}
