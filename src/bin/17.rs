advent_of_code::solution!(17);
use rustc_hash::FxHashSet;

fn hard_coded_program(a: usize) -> usize {
    // B and C are both set before they are used. So only A
    // contributes to state
    let mut b = a % 8; // Take last 3 binary digits of a
    b ^= 3;
    let c = a / 2usize.pow(b as u32);
    b ^= c;
    // let a = a / 8; // Shift right 3 binary digits
    b ^= 5;
    b % 8
}

fn find_a(expected_output: &[usize]) -> usize {
    let mut possible_answers = FxHashSet::default();
    possible_answers.insert(0);
    for num in expected_output.iter().rev() {
        let mut new_possible_answers = FxHashSet::default();
        for curr in possible_answers {
            for i in 0..8 {
                let new = (curr << 3) + i;
                if hard_coded_program(new) == *num {
                    new_possible_answers.insert(new);
                }
            }
        }
        possible_answers = new_possible_answers;
    }

    *possible_answers.iter().min().unwrap()
}

pub fn part_one(_input: &str) -> Option<usize> {
    let mut a = 52884621;
    let mut output = Vec::new();
    while a > 0 {
        output.push(hard_coded_program(a));
        a /= 8;
    }

    let result = output
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join("")
        .parse::<usize>()
        .unwrap();

    Some(result)
}

pub fn part_two(_input: &str) -> Option<usize> {
    let expected_output = vec![2, 4, 1, 3, 7, 5, 4, 7, 0, 3, 1, 5, 5, 5, 3, 0];
    let result = find_a(expected_output.as_slice());
    Some(result)
}
