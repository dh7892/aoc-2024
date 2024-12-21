advent_of_code::solution!(21);

use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Action {
    Up,
    Down,
    Left,
    Right,
    Activate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Button {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Zero,
    Activate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: usize,
    y: usize,
}

fn position_for_button(button: &Button) -> Position {
    match button {
        Button::One => Position { x: 0, y: 2 },
        Button::Two => Position { x: 1, y: 2 },
        Button::Three => Position { x: 2, y: 2 },
        Button::Four => Position { x: 0, y: 1 },
        Button::Five => Position { x: 1, y: 1 },
        Button::Six => Position { x: 2, y: 1 },
        Button::Seven => Position { x: 0, y: 0 },
        Button::Eight => Position { x: 1, y: 0 },
        Button::Nine => Position { x: 2, y: 0 },
        Button::Zero => Position { x: 1, y: 3 },
        Button::Activate => Position { x: 2, y: 3 },
    }
}

fn position_for_action(action: &Action) -> Position {
    match action {
        Action::Up => Position { x: 1, y: 0 },
        Action::Down => Position { x: 1, y: 1 },
        Action::Left => Position { x: 0, y: 1 },
        Action::Right => Position { x: 2, y: 1 },
        Action::Activate => Position { x: 2, y: 0 },
    }
}

fn get_path_combinations(horizontal: Vec<Action>, vertical: Vec<Action>) -> Vec<Vec<Action>> {
    let total_len = horizontal.len() + vertical.len();

    // We only need to choose the positions for one set of moves
    // The other set will fill in the gaps
    (0..total_len)
        .combinations(horizontal.len())
        .map(|positions| {
            let mut result = vec![vertical[0].clone(); total_len]; // Fill with vertical moves
            for (i, pos) in positions.iter().enumerate() {
                result[*pos] = horizontal[i].clone(); // Put horizontal moves in chosen positions
            }
            result
        })
        .collect()
}

fn path_includes_position(path: &[Action], start: &Position, pos: &Position) -> bool {
    // Return true if moving from start via the steps given in path would include the position pos
    let mut current = *start;
    for step in path {
        let next = match step {
            Action::Up => Position {
                x: current.x,
                y: current.y - 1,
            },
            Action::Down => Position {
                x: current.x,
                y: current.y + 1,
            },
            Action::Left => Position {
                x: current.x - 1,
                y: current.y,
            },
            Action::Right => Position {
                x: current.x + 1,
                y: current.y,
            },
            Action::Activate => current,
        };
        if next == *pos {
            return true;
        }
        current = next;
    }
    false
}

fn ways_to_reach_button(current: &Button, target: &Button) -> Vec<Vec<Action>> {
    // Given a start and an end button, return all possible ways to reach the target button
    // excluding any that pass through the empty space at 0,3
    let current_position = position_for_button(current);
    let target_position = position_for_button(target);
    let dx = target_position.x as isize - current_position.x as isize;
    let dy = target_position.y as isize - current_position.y as isize;
    let horizontal_moves = if dx < 0 {
        vec![Action::Left; dx.unsigned_abs()]
    } else {
        vec![Action::Right; dx.unsigned_abs()]
    };
    let vertical_moves = if dy < 0 {
        vec![Action::Up; dy.unsigned_abs()]
    } else {
        vec![Action::Down; dy.unsigned_abs()]
    };
    let result = get_path_combinations(horizontal_moves, vertical_moves);

    dbg!(&result);
    let danger_position = Position { x: 0, y: 3 };
    result
        .into_iter()
        .filter(|path| !path_includes_position(path, &current_position, &danger_position))
        .collect()
}

fn actions_to_next_digit(current: &Button, target: &Button) -> Vec<Action> {
    // Given we are hovering over the current button
    // Return a vector of optimal vectors of actions to trigger the target button
    // There may be multiple ways to trigger the target button but we just pick one
    // that avoids the empty space ad 0,3
    let current_position = position_for_button(current);
    let target_position = position_for_button(target);

    let dx = target_position.x as isize - current_position.x as isize;
    let dy = target_position.y as isize - current_position.y as isize;
    let mut result = Vec::new();
    match (dx < 0, dy < 0) {
        (true, true) => {
            // Moving left and up
            // So need to take care not to move through the empty space
            // But we'd prefer to move left first if possible (this is more eficient)
            if current_position.y < 3 {
                result.extend(vec![Action::Left; dx.unsigned_abs()]);
                result.extend(vec![Action::Up; dy.unsigned_abs()]);
            } else {
                result.extend(vec![Action::Up; dy.unsigned_abs()]);
                result.extend(vec![Action::Left; dx.unsigned_abs()]);
            }
        }
        (true, false) => {
            // Moving left and down
            result.extend(vec![Action::Down; dy.unsigned_abs()]);
            result.extend(vec![Action::Left; dx.unsigned_abs()]);
        }
        (false, true) => {
            // Moving right and up
            result.extend(vec![Action::Up; dy.unsigned_abs()]);
            result.extend(vec![Action::Right; dx.unsigned_abs()]);
        }
        (false, false) => {
            // Moving right and down so need to move right first and then down
            // Again, we prefer to move down first if possible
            if target_position.y < 3 {
                result.extend(vec![Action::Down; dy.unsigned_abs()]);
                result.extend(vec![Action::Right; dx.unsigned_abs()]);
            } else {
                result.extend(vec![Action::Right; dx.unsigned_abs()]);
                result.extend(vec![Action::Down; dy.unsigned_abs()]);
            }
        }
    };

    // However, we need to press the activate button at the end
    result.push(Action::Activate);
    result
}

fn actions_for_next_action(current: &Action, target: &Action) -> Vec<Action> {
    // Given we are hovering over our current action button
    // Return a vector of optimal actions to get us to our next action
    let dx = position_for_action(target).x as isize - position_for_action(current).x as isize;
    let dy = position_for_action(target).y as isize - position_for_action(current).y as isize;
    let mut result = vec![];
    match (dx < 0, dy < 0) {
        (true, true) => {
            // Moving left and up
            result.extend(vec![Action::Up; dy.unsigned_abs()]);
            result.extend(vec![Action::Left; dx.unsigned_abs()]);
        }
        (true, false) => {
            // Moving left and down
            result.extend(vec![Action::Down; dy.unsigned_abs()]);
            result.extend(vec![Action::Left; dx.unsigned_abs()]);
        }
        (false, true) => {
            // Moving right and up
            result.extend(vec![Action::Right; dx.unsigned_abs()]);
            result.extend(vec![Action::Up; dy.unsigned_abs()]);
        }
        (false, false) => {
            // Moving right and down
            result.extend(vec![Action::Right; dx.unsigned_abs()]);
            result.extend(vec![Action::Down; dy.unsigned_abs()]);
        }
    }
    result.push(Action::Activate);

    result
}

fn possible_actions_for_buttons(buttons: &[Button]) -> Vec<Vec<Action>> {
    let mut current = Button::Activate;
    let mut result: Vec<Vec<Action>> = vec![];
    for button in buttons {
        let actions = ways_to_reach_button(&current, button);
        // We need to combine with all of the previous actions
        result = result
            .iter()
            .flat_map(|prev| {
                actions.iter().map(move |next| {
                    let mut result = prev.clone();
                    result.extend(next.clone());
                    result
                })
            })
            .collect();
        current = *button;
    }
    result
}

fn actions_for_buttons(buttons: &[Button]) -> Vec<Action> {
    let mut result = vec![];
    for window in buttons.windows(2) {
        result.extend(actions_to_next_digit(&window[0], &window[1]));
    }
    result
}

fn actions_for_actions(actions: &[Action]) -> Vec<Action> {
    let mut result = vec![];
    let mut actions_including_start = vec![Action::Activate];
    actions_including_start.extend(actions);
    for window in actions_including_start.windows(2) {
        result.extend(actions_for_next_action(&window[0], &window[1]));
    }
    result
}

fn buttons_for_input(line: &str) -> Vec<Button> {
    let mut result = vec![Button::Activate]; // Start over the activate button
    for digit in line.chars() {
        result.push(match digit {
            '1' => Button::One,
            '2' => Button::Two,
            '3' => Button::Three,
            '4' => Button::Four,
            '5' => Button::Five,
            '6' => Button::Six,
            '7' => Button::Seven,
            '8' => Button::Eight,
            '9' => Button::Nine,
            '0' => Button::Zero,
            'A' => Button::Activate,
            _ => panic!("Invalid digit"),
        });
    }
    result
}

fn final_actions_for_digits(digits: &[Button]) -> Vec<Action> {
    let actions = actions_for_buttons(digits);
    let actions = actions_for_actions(&actions);
    actions_for_actions(&actions)
}

fn shortest_actions_for_digits(digits: &[Button]) -> usize {
    let actions = possible_actions_for_buttons(digits);
    // Expand out all of the possible actions into keypad sequences
    actions
        .iter()
        .map(|actions| actions_for_actions(actions))
        .map(|actions| actions_for_actions(&actions))
        .map(|actions| actions.len())
        .min()
        .unwrap()
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut score = 0;
    for line in input.lines() {
        let digits = buttons_for_input(line);
        let num_actions = shortest_actions_for_digits(&digits);
        let number = line[..line.len() - 1].parse::<usize>().unwrap();
        score += number * num_actions;
    }
    Some(score)
}

pub fn part_two(input: &str) -> Option<usize> {
    None
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
