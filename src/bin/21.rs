advent_of_code::solution!(21);
use std::collections::{HashMap, VecDeque};
use std::fs;
use std::rc::Rc;
use std::time::Instant;

pub const DIRS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

const NUMERIC: [[u8; 3]; 4] = [
    [b'7', b'8', b'9'],
    [b'4', b'5', b'6'],
    [b'1', b'2', b'3'],
    [b' ', b'0', b'A'],
];

const DIRECTIONAL: [[u8; 3]; 2] = [[b' ', b'^', b'A'], [b'<', b'v', b'>']];

fn find_shortest_paths(
    keypad: &[[u8; 3]],
    from: u8,
    to: u8,
    cache: &mut HashMap<(u8, u8), Rc<Vec<Vec<u8>>>>,
) -> Rc<Vec<Vec<u8>>> {
    if let Some(cached) = cache.get(&(from, to)) {
        return cached.clone();
    }

    if from == to {
        let result = Rc::new(vec![vec![b'A']]);
        cache.insert((from, to), result.clone());
        return result;
    }

    // find 'from' and 'to' on keypad
    let mut start = (0, 0);
    let mut end = (0, 0);
    for (y, row) in keypad.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            if c == from {
                start = (x, y);
            }
            if c == to {
                end = (x, y);
            }
        }
    }

    // flood fill keypad to find the shortest distances
    let mut dists = vec![[usize::MAX; 3]; keypad.len()];
    let mut queue = VecDeque::new();
    queue.push_back((start.0, start.1, 0));

    while let Some((x, y, steps)) = queue.pop_front() {
        dists[y][x] = steps;
        for (dx, dy) in DIRS {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx >= 0
                && ny >= 0
                && nx < 3
                && ny < keypad.len() as i32
                && keypad[ny as usize][nx as usize] != b' '
                && dists[ny as usize][nx as usize] == usize::MAX
            {
                queue.push_back((nx as usize, ny as usize, steps + 1));
            }
        }
    }

    // backtrack from 'end' back to 'start' and collect all paths
    let mut paths = Vec::new();
    let mut stack = Vec::new();
    stack.push((end.0, end.1, vec![b'A']));
    while let Some((x, y, path)) = stack.pop() {
        if x == start.0 && y == start.1 {
            paths.push(path);
            continue;
        }
        for (i, (dx, dy)) in DIRS.iter().enumerate() {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx >= 0
                && ny >= 0
                && nx < 3
                && ny < keypad.len() as i32
                && dists[ny as usize][nx as usize] < dists[y][x]
            {
                // do everything in reverse
                let c = match i {
                    0 => b'<',
                    1 => b'^',
                    2 => b'>',
                    3 => b'v',
                    _ => panic!(),
                };
                let mut new_path = vec![c];
                new_path.extend(&path);
                stack.push((nx as usize, ny as usize, new_path));
            }
        }
    }

    let result = Rc::new(paths);
    cache.insert((from, to), result.clone());
    result
}

fn find_shortest_sequence(
    s: &[u8],
    depth: usize,
    highest: bool,
    cursors: &mut Vec<u8>,
    cache: &mut HashMap<(Vec<u8>, usize, u8), usize>,
    path_cache: &mut HashMap<(u8, u8), Rc<Vec<Vec<u8>>>>,
) -> usize {
    let cache_key = (s.to_vec(), depth, cursors[depth]);
    if let Some(cached) = cache.get(&cache_key) {
        return *cached;
    }

    let mut result = 0;
    for &c in s {
        let paths = find_shortest_paths(
            if highest { &NUMERIC } else { &DIRECTIONAL },
            cursors[depth],
            c,
            path_cache,
        );
        if depth == 0 {
            result += paths.iter().map(|l| l.len()).min().unwrap();
        } else {
            result += paths
                .iter()
                .map(|p| find_shortest_sequence(p, depth - 1, false, cursors, cache, path_cache))
                .min()
                .unwrap();
        }
        cursors[depth] = c;
    }

    cache.insert(cache_key, result);

    result
}

pub fn part_one(input: &str) -> Option<usize> {
    let lines = input.lines().collect::<Vec<_>>();

    let mut cache = HashMap::new();
    let mut path_cache = HashMap::new();

    let max_depth = 2;

    let mut total = 0;
    for l in &lines {
        let mut cursors = vec![b'A'; max_depth + 1];
        let len = find_shortest_sequence(
            l.as_bytes(),
            max_depth,
            true,
            &mut cursors,
            &mut cache,
            &mut path_cache,
        );

        let n = l[0..3].parse::<usize>().unwrap();
        total += n * len;
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<usize> {
    let lines = input.lines().collect::<Vec<_>>();

    let mut cache = HashMap::new();
    let mut path_cache = HashMap::new();

    let max_depth = 25;

    let mut total = 0;
    for l in &lines {
        let mut cursors = vec![b'A'; max_depth + 1];
        let len = find_shortest_sequence(
            l.as_bytes(),
            max_depth,
            true,
            &mut cursors,
            &mut cache,
            &mut path_cache,
        );

        let n = l[0..3].parse::<usize>().unwrap();
        total += n * len;
    }
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(126384));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    // #[rstest]
    // #[case("029A", 12)]
    // #[case("179A", 14)]
    // fn test_actions_for_digits(#[case] input: &str, #[case] length: usize) {
    //     let digits = buttons_for_input(input);
    //     let actions = actions_for_buttons(&digits);
    //     assert_eq!(actions.len(), length);
    // }

    // #[rstest]
    // #[case(Action::Activate, Action::Left, 4)]
    // fn test_actions_for_actions(
    //     #[case] current: Action,
    //     #[case] target: Action,
    //     #[case] expected_length: usize,
    // ) {
    //     let actions = actions_for_next_action(&current, &target);
    //     assert_eq!(actions.len(), expected_length);
    // }

    // #[rstest]
    // #[case("029A", 28)]
    // #[case("179A", 28)]
    // #[case("379A", 28)]
    // fn test_actions_for_actions_for_digits(#[case] input: &str, #[case] length: usize) {
    //     let digits = buttons_for_input(input);
    //     let actions = actions_for_buttons(&digits);
    //     let actions = actions_for_actions(&actions);
    //     dbg!(&actions);
    //     assert_eq!(actions.len(), length);
    // }

    // #[rstest]
    // #[case("029A", 68)]
    // #[case("980A", 60)]
    // #[case("179A", 68)]
    // #[case("456A", 64)]
    // #[case("379A", 64)]
    // fn test_sequence_for_number(#[case] input: &str, #[case] length: usize) {
    //     let digits = buttons_for_input(input);
    //     let actions = final_actions_for_digits(&digits);
    //     assert_eq!(actions.len(), length);
    // }

    // #[rstest]
    // #[case(Action::Up, Action::Right, 3)]
    // fn test_action_to_next_action(
    //     #[case] current: Action,
    //     #[case] target: Action,
    //     #[case] expected_length: usize,
    // ) {
    //     let actions = actions_for_next_action(&current, &target);
    //     assert_eq!(actions.len(), expected_length);
    // }
    #[test]
    fn test_ways_to_reach_button() {
        let current = Button::One;
        let target = Button::Five;
        let result = ways_to_reach_button(&current, &target);
        assert_eq!(result.len(), 2);
        let current = Button::One;
        let target = Button::Nine;
        let result = ways_to_reach_button(&current, &target);
        assert_eq!(result.len(), 6);
    }
}
