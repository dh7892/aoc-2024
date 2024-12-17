advent_of_code::solution!(17);
use scan_fmt::*;

enum Instruction {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl Instruction {
    fn from_int(i: usize) -> Self {
        match i {
            0 => Instruction::Adv,
            1 => Instruction::Bxl,
            2 => Instruction::Bst,
            3 => Instruction::Jnz,
            4 => Instruction::Bxc,
            5 => Instruction::Out,
            6 => Instruction::Bdv,
            7 => Instruction::Cdv,
            _ => panic!("Invalid instruction"),
        }
    }
}

struct Machine {
    register_a: usize,
    register_b: usize,
    register_c: usize,
    instruction_pointer: usize,
    instructions: Vec<usize>,
    output: Vec<usize>,
}

impl Machine {
    fn literal_operand(&self) -> usize {
        self.instructions[self.instruction_pointer + 1]
    }
    fn combo_operand(&self) -> usize {
        match self.literal_operand() {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => self.register_a,
            5 => self.register_b,
            6 => self.register_c,
            _ => panic!("Invalid operand"),
        }
    }
    fn do_op(&mut self) -> bool {
        // Run the next instruction
        // Return false if the instruction pointer is out of bounds

        if self.instruction_pointer >= self.instructions.len() || self.output.len() >= 20 {
            return false;
        }
        let instruction = Instruction::from_int(self.instructions[self.instruction_pointer]);
        match instruction {
            Instruction::Adv => {
                // Division between register A and the 2 ^ combo operand -> register A
                let numerator = self.register_a;
                let denominator = 2usize.pow(self.combo_operand() as u32);
                self.register_a = numerator / denominator;
                self.instruction_pointer += 2;
            }
            Instruction::Bxl => {
                // Bitwise XOR between register B and literal operand -> register B
                let left = self.register_b;
                let right = self.literal_operand();
                self.register_b = left ^ right;
                self.instruction_pointer += 2;
            }
            Instruction::Bst => {
                // Modulo 8 of the combo operator -> register b
                self.register_b = self.combo_operand() % 8;
                self.instruction_pointer += 2;
            }
            Instruction::Jnz => {
                // Jump to the literal operand if the combo operand is not 0
                match self.register_a {
                    0 => self.instruction_pointer += 2,
                    _ => self.instruction_pointer = self.literal_operand(),
                }
            }
            Instruction::Bxc => {
                // Bitwise XOR between register B and register C -> Register B
                self.register_b = self.register_b ^ self.register_c;
                self.instruction_pointer += 2;
            }
            Instruction::Out => {
                // Output the value of the combo operand % 8
                self.output.push(self.combo_operand() % 8);
                self.instruction_pointer += 2;
            }
            Instruction::Bdv => {
                // Division between register A and the 2 ^ combo operand -> register B
                let numerator = self.register_a;
                let denominator = 2usize.pow(self.combo_operand() as u32);
                self.register_b = numerator / denominator;
                self.instruction_pointer += 2;
            }
            Instruction::Cdv => {
                // Division between register A and the 2 ^ combo operand -> register C
                let numerator = self.register_a;
                let denominator = 2usize.pow(self.combo_operand() as u32);
                self.register_c = numerator / denominator;
                self.instruction_pointer += 2;
            }
        };
        true
    }

    fn print(&self) {
        println!(
            "A: {}, B: {}, C: {}, IP: {}, Instructions: {:?}, Output: {:?}",
            self.register_a,
            self.register_b,
            self.register_c,
            self.instruction_pointer,
            self.instructions,
            self.output,
        );
    }
}

fn input_to_machine(input: &str) -> Machine {
    let mut lines = input.lines();
    let register_a = scan_fmt!(lines.next().unwrap(), "Register A: {}", usize).unwrap();
    let register_b = scan_fmt!(lines.next().unwrap(), "Register B: {}", usize).unwrap();
    let register_c = scan_fmt!(lines.next().unwrap(), "Register C: {}", usize).unwrap();
    lines.next(); // Skip the blank line
    let instruction_pointer = 0;
    // We can just ignore the "Program: " part of the string
    let instructions_string = lines.next().unwrap()[9..].to_string();
    let instructions = instructions_string
        .split(",")
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    Machine {
        register_a,
        register_b,
        register_c,
        instruction_pointer,
        instructions,
        output: Vec::new(),
    }
}

fn find_self_a_that_gives_output(machine: &mut Machine, output: &Vec<usize>) -> usize {
    // find the lowest value of register A that will cause the machine to repeat itself
    // let min = 234358885653444;
    //242236155494400
    // let max = 6522275610215704;
    let min = 0;
    let max = 2usize.pow(64);

    println!("Min: {}, Max: {}", min, max);
    for i in min..max {
        // Reset the machine
        machine.output = Vec::new();
        machine.register_a = i;
        machine.instruction_pointer = 0;
        while machine.do_op() {}
        if machine.output == *output {
            return i;
        }
    }
    0
}

fn all_output(register_a: usize, num_values: usize) -> Vec<usize> {
    let mut reg_a = register_a;
    let mut output = Vec::new();
    for _ in 0..num_values {
        let (a, value) = hard_coded_program(reg_a);
        reg_a = a;
        output.push(value);
    }
    output
}

fn hard_find(output: &[usize]) -> usize {
    for i in 0..2usize.pow(64) {
        let trial_outputs = all_output(i, output.len());
        if trial_outputs == *output {
            return i;
        }
    }
    0
}

fn hard_coded_program(a: usize) -> (usize, usize) {
    // B and C are both set before they are used. So only A
    // contributes to state
    let mut b = a % 8; // Take last 3 binary digits of a
    b = b ^ 3;
    let c = a / 2usize.pow(b as u32);
    b = b ^ c;
    let a = a / 8; // Shift right 3 binary digits
    b = b ^ 5;
    (a * 8, b)
}
fn find_lowest_num_that_gives_expected(expected: usize, previous_state: usize) -> usize {
    println!(
        "Finding state for expected {}, previous {}",
        expected, previous_state
    );
    let next_part = previous_state * 8; // Shift left 3 binary digits
    for i in 0..8 {
        let state = next_part + i;
        let (remainder, output) = hard_coded_program(state);
        if output == expected && remainder == previous_state {
            return state;
        }
    }
    0
}

const EXPECTED_PROGRAM: [usize; 16] = [2, 4, 1, 3, 7, 5, 4, 7, 0, 3, 1, 5, 5, 5, 3, 0];

fn find_all_parts() -> usize {
    let mut previous_state = 0;
    for expected in EXPECTED_PROGRAM.iter().rev() {
        previous_state = find_lowest_num_that_gives_expected(*expected, previous_state);
    }
    previous_state
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut machine = input_to_machine(input);
    while machine.do_op() {}
    machine.print();
    let output_string = machine
        .output
        .iter()
        .map(|i| i.to_string())
        .collect::<Vec<String>>()
        .join(",");
    println!("Output: {}", output_string);
    let output = machine
        .output
        .iter()
        .map(|i| i.to_string())
        .collect::<String>()
        .parse::<usize>()
        .unwrap();
    Some(output)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut m1 = input_to_machine(input);
    let check = hard_find(&[0]);
    println!("Check: {}", check);
    let check = hard_find(&[3, 0]);
    println!("Check: {}", check);
    let mut m1 = input_to_machine(input);
    let check = hard_find(&[5, 3, 0]);
    println!("Check: {}", check);

    // let mut machine = input_to_machine(input);
    // let analytic_solution = find_all_parts();
    // machine.register_a = analytic_solution;
    // while machine.do_op() {}
    // machine.print();

    // Some(analytic_solution)
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4635635210));
    }

    #[test]
    fn test_part_two() {
        let input = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";
        let result = part_two(input);
        assert_eq!(result, Some(117440));
    }

    #[test]
    fn test_op_2() {
        let mut machine = Machine {
            register_a: 0,
            register_b: 0,
            register_c: 9,
            instruction_pointer: 0,
            instructions: vec![2, 6],
            output: Vec::new(),
        };
        machine.do_op();
        assert_eq!(machine.register_b, 1);
    }

    #[test]
    fn test_example_2() {
        let mut machine = Machine {
            register_a: 10,
            register_b: 0,
            register_c: 0,
            instruction_pointer: 0,
            instructions: vec![5, 0, 5, 1, 5, 4],
            output: Vec::new(),
        };
        machine.do_op();
        machine.do_op();
        machine.do_op();
        assert_eq!(machine.output, vec![0, 1, 2]);
    }

    #[test]
    fn test_example_3() {
        let mut machine = Machine {
            register_a: 2024,
            register_b: 0,
            register_c: 0,
            instruction_pointer: 0,
            instructions: vec![0, 1, 5, 4, 3, 0],
            output: Vec::new(),
        };
        while machine.do_op() {}
        assert_eq!(machine.output, vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
        assert_eq!(machine.register_a, 0);
    }

    #[test]
    fn test_op_1() {
        let mut machine = Machine {
            register_a: 0,
            register_b: 29,
            register_c: 0,
            instruction_pointer: 0,
            instructions: vec![1, 7],
            output: Vec::new(),
        };
        machine.do_op();
        assert_eq!(machine.register_b, 26);
    }

    #[test]
    fn test_op_4() {
        let mut machine = Machine {
            register_a: 0,
            register_b: 2024,
            register_c: 43690,
            instruction_pointer: 0,
            instructions: vec![4, 0],
            output: Vec::new(),
        };
        machine.do_op();
        assert_eq!(machine.register_b, 44354);
    }
}
