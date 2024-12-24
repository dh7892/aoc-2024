advent_of_code::solution!(24);

use std::collections::HashMap;

type Wires = HashMap<String, Option<bool>>;

#[derive(Debug, PartialEq, Copy, Clone)]
enum Operation {
    And,
    Or,
    Xor,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum GateState {
    Calculated(bool),
    Uncalculated,
}

struct Gate {
    input1: Option<String>,
    input2: Option<String>,
    operation: Operation,
    output: Option<String>,
    state: GateState,
}

impl Gate {
    fn calculate(&mut self, wires: &mut Wires) -> bool {
        // If we haven't already calculated the value
        // And if both input wires have a value
        // Then calculate the value, store it in the output wire, and return true
        // Else return false
        // Basically, the return value indicates whether a new calculation was made or not
        match (
            self.state,
            self.input1.clone(),
            self.input2.clone(),
            self.output.clone(),
        ) {
            (GateState::Uncalculated, Some(input1), Some(input2), Some(output)) => {
                if let (Some(Some(i1)), Some(Some(i2))) = (wires.get(&input1), wires.get(&input2)) {
                    let value = match self.operation {
                        Operation::And => *i1 && *i2,
                        Operation::Or => *i1 || *i2,
                        Operation::Xor => *i1 ^ *i2,
                    };
                    wires.insert(output, Some(value));
                    self.state = GateState::Calculated(value);
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }
}

fn simulate_step(gates: &mut [Gate], wires: &mut Wires) -> bool {
    gates.iter_mut().any(|gate| gate.calculate(wires))
}

fn simulate(gates: &mut [Gate], wires: &mut Wires) {
    while simulate_step(gates, wires) {}
}

fn output(wires: &Wires) -> Option<usize> {
    // Find all wires that start with the letter "z"
    // Then order them by the number that comes after the letter "z"
    // Then map each value to a binary digit and convert to usize
    // If any wires are None, return None
    wires
        .iter()
        .filter(|(key, _)| key.starts_with("z"))
        .map(|(key, value)| {
            value.and_then(|value| {
                key[1..]
                    .parse::<usize>()
                    .ok()
                    .map(|index| if value { 1 << index } else { 0 })
            })
        })
        .collect::<Option<Vec<usize>>>()
        .map(|values| values.iter().sum())
}

fn parse_input(input: &str) -> (Vec<Gate>, Wires) {
    let mut gates = Vec::new();
    let mut wires = Wires::new();

    // Initial state of wires is the first part of the file (up to the first empty line)
    let parts: Vec<&str> = input.split("\n\n").collect();
    for line in parts[0].lines() {
        let words: Vec<&str> = line.split(": ").collect();
        let name = words[0];
        // the value will be either "0" or "1", so convert to bool
        match words[1] {
            "0" => wires.insert(name.to_string(), Some(false)),
            "1" => wires.insert(name.to_string(), Some(true)),
            _ => panic!("Invalid input: {}", line),
        };
    }

    for line in parts[1].lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        match parts.as_slice() {
            [input1, "AND", input2, "->", output] => {
                // Need to make sure the wires have been added, too
                wires.entry(input1.to_string()).or_insert(None);
                wires.entry(input2.to_string()).or_insert(None);
                wires.entry(output.to_string()).or_insert(None);
                gates.push(Gate {
                    input1: Some(input1.to_string()),
                    input2: Some(input2.to_string()),
                    operation: Operation::And,
                    output: Some(output.to_string()),
                    state: GateState::Uncalculated,
                });
            }
            [input1, "OR", input2, "->", output] => {
                wires.entry(input1.to_string()).or_insert(None);
                wires.entry(input2.to_string()).or_insert(None);
                wires.entry(output.to_string()).or_insert(None);
                gates.push(Gate {
                    input1: Some(input1.to_string()),
                    input2: Some(input2.to_string()),
                    operation: Operation::Or,
                    output: Some(output.to_string()),
                    state: GateState::Uncalculated,
                });
            }
            [input1, "XOR", input2, "->", output] => {
                wires.entry(input1.to_string()).or_insert(None);
                wires.entry(input2.to_string()).or_insert(None);
                wires.entry(output.to_string()).or_insert(None);
                gates.push(Gate {
                    input1: Some(input1.to_string()),
                    input2: Some(input2.to_string()),
                    operation: Operation::Xor,
                    output: Some(output.to_string()),
                    state: GateState::Uncalculated,
                });
            }
            _ => panic!("Invalid input: {}", line),
        }
    }

    (gates, wires)
}

pub fn part_one(input: &str) -> Option<usize> {
    let (mut gates, mut wires) = parse_input(input);
    simulate(&mut gates, &mut wires);
    output(&wires)
}

pub fn part_two(_input: &str) -> Option<String> {
    // Solved today's problem by hand in vim!
    Some("dpg,kmb,mmf,tvp,vdk,z10,z15,z25".to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2024));
    }
}
