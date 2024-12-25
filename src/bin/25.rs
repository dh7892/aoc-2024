advent_of_code::solution!(25);

type Lock = [usize; 5];
type Key = [usize; 5];

enum LockOrKey {
    Lock(Lock),
    Key(Key),
}

fn key_fits_lock(key: &Key, lock: &Lock) -> bool {
    for (key_pin, lock_pin) in key.iter().zip(lock.iter()) {
        if key_pin > lock_pin {
            return false;
        }
    }
    true
}

fn lock_or_key_from_block(block: &str) -> LockOrKey {
    let mut pins = [0; 5];
    let is_lock = block.starts_with("#");
    for line in block.lines() {
        for (i, c) in line.chars().enumerate() {
            if c == '#' {
                pins[i] += 1;
            }
        }
    }
    if is_lock {
        // Need the inverse of the pins (6 - pin)
        pins.iter_mut().for_each(|pin| *pin = 6 - *pin);
        LockOrKey::Lock(pins)
    } else {
        // We ignore the first layer of pins (that's the shaft)
        pins.iter_mut().for_each(|pin| *pin -= 1);
        LockOrKey::Key(pins)
    }
}

fn parse_input(input: &str) -> (Vec<Lock>, Vec<Key>) {
    let mut locks = Vec::new();
    let mut keys = Vec::new();
    for block in input.split("\n\n") {
        match lock_or_key_from_block(block) {
            LockOrKey::Lock(lock) => locks.push(lock),
            LockOrKey::Key(key) => keys.push(key),
        }
    }
    (locks, keys)
}

pub fn part_one(input: &str) -> Option<usize> {
    let (locks, keys) = parse_input(input);
    // Find whihc keys fit
    Some(
        keys.iter()
            .map(|key| locks.iter().filter(|lock| key_fits_lock(key, lock)).count())
            .sum(),
    )
}

pub fn part_two(_input: &str) -> Option<usize> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
