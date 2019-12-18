use std::collections::HashMap;
use crate::code::Digits;

const POSITION_MODE: u32 = 0;
const IMMEDIATE_MODE: u32 = 1;

lazy_static! {
    static ref OPCODE_LENGTHS: HashMap<i64, usize> = {
        let mut map = HashMap::new();
        map.insert(1, 4);
        map.insert(2, 4);
        map.insert(99, 1);
        map
    };
}

pub fn run_program_with_inputs(
    original: &Vec<i64>,
    noun: i64,
    verb: i64,
) -> Vec<i64> {
    let mut program = original.clone();
    program[1] = noun;
    program[2] = verb;
    run_program(&program)
}

pub fn run_program_with_inputs_and_get_output(
    original: &Vec<i64>,
    noun: i64,
    verb: i64,
) -> i64 {
    let result = run_program_with_inputs(original, noun, verb);
    result[0]
}

pub fn run_program_to_get_output(
    original: &Vec<i64>,
    desired_output: i64,
) -> (i64, i64) {
    for i in 0..=99 {
        for j in 0..=99 {
            if run_program_with_inputs_and_get_output(original, i, j) == desired_output {
                return (i, j);
            }
        }
    }
    (0, 0)
}

#[derive(Debug, Default)]
struct Opcode {
    number: i64,
    modes: Vec<u32>,
    length: usize,
}

impl From<&i64> for Opcode {
    fn from(opcode: &i64) -> Self {
        let digits = Digits::from(*opcode as u32);
        let mut iterator = digits.rev();

        // Interpret opcode
        let mut number_string = "".to_string();
        let ones_place = iterator.next().expect("need at least one digit for an opcode");
        let tens_place = iterator.next();
        match tens_place {
            Some(t) => number_string.push_str(&t.to_string()),
            None => (),
        }
        number_string.push_str(&ones_place.to_string());
        let number = number_string.parse::<i64>().expect("need an i64");

        let mut modes = iterator.collect::<Vec<_>>();

        let length_opt = OPCODE_LENGTHS.get(&number);
        assert!(length_opt.is_some(), "no length for opcode {}", number);
        let length = length_opt.unwrap();

        if modes.len() < length - 1 {
            for _ in modes.len()..length - 1 {
                modes.push(0);
            }
        }

        // println!("opcode number: {:?}", number);
        // println!("modes: {:?}", modes);

        assert_eq!(modes.len(), length - 1);

        Opcode {
            number,
            modes,
            length: *length,
        }
    }
}

struct Instruction {
    opcode: Opcode,
    indexes: Vec<Option<usize>>,
    values: Vec<Option<i64>>,
}

impl Instruction {
    fn new(opcode: Opcode, program: &Vec<i64>, pos: &usize) -> Instruction {
        let mut indexes = Vec::new();
        let mut values = Vec::new();

        // println!("opcode.number: {:?}", opcode.number);
        // println!("opcode.modes: {:?}", opcode.modes);

        for (mode_pos, mode) in opcode.modes.iter().enumerate() {
            let value_at_pos = get_at_position(&program, *pos + mode_pos + 1);

            let (index, value) = match *mode {
                POSITION_MODE => {
                    let index =
                        if let Some(v) = value_at_pos {
                            Some(v as usize)
                        } else {
                            None
                        };

                    // println!("index: {:?}", index);

                    let value =
                        if let Some(i) = index {
                            if i < program.len() {
                                Some(program[i])
                            } else {
                                None
                            }
                        } else {
                            None
                        };
                    // println!("value: {:?}", value);

                    (index, value)
                },
                IMMEDIATE_MODE => {
                    (None, value_at_pos)
                },
                _ => break,
            };

            indexes.push(index);
            values.push(value);
        }

        // println!("indexes: {:?}", indexes);
        // println!("values: {:?}", values);
        // println!("::");

        Instruction {
            opcode,
            indexes,
            values,
        }
    }

    // Returns the length of the instruction.
    pub fn run(&self, program: &mut Vec<i64>) -> Option<usize> {
        match self.opcode.number {
            1 => {
                // println!("add values: {:?}", &self.values[0..2]);
                let result = &self.values[0..2].iter().fold(0, |acc, n| acc + n.unwrap());
                // println!("result: {:?}", result);
                let result_index = self.indexes[2].unwrap();
                // println!("result_index: {:?}", result_index);

                program[result_index] = *result;

                // println!("program: {:?}", program);
                // println!("");
                Some(self.opcode.length)
            },
            2 => {
                // println!("multiply values: {:?}", &self.values[0..2]);
                let result = &self.values[0..2].iter().fold(1, |acc, n| acc * n.unwrap());
                // println!("result: {:?}", result);
                let result_index = self.indexes[2].unwrap();
                // println!("result_index: {:?}", result_index);

                program[result_index] = *result;

                // println!("program: {:?}", program);
                // println!("");
                Some(self.opcode.length)
            },
            99 => None,
            _ => None,
        }
    }
}

fn get_at_position(program: &Vec<i64>, pos: usize) -> Option<i64> {
    match pos {
        p if p >= program.len() => None,
        _ => Some(program[pos]),
    }
}

pub fn run_program(original: &Vec<i64>) -> Vec<i64> {
    let mut program = original.clone();

    let mut pos = 0;
    let mut opcode: Opcode;

    loop {
        let opcode_value = get_at_position(&program, pos).unwrap();
        opcode = Opcode::from(&opcode_value);
        let instruction = Instruction::new(opcode, &program, &pos);

        match instruction.run(&mut program) {
            Some(length) => pos += length,
            None => break,
        }
    }

    program
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_program_instruction() {
        assert_eq!(
            run_program(&vec![1101,100,-1,4,0]),
            vec![1101,100,-1,4,99]
        );
    }

    #[test]
    fn test_run_program() {
        assert_eq!(
            run_program(&vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]),
            vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
        );
        assert_eq!(run_program(&vec![1, 0, 0, 0, 99]), vec![2, 0, 0, 0, 99]);
        assert_eq!(run_program(&vec![2, 3, 0, 3, 99]), vec![2, 3, 0, 6, 99]);
        assert_eq!(
            run_program(&vec![2, 4, 4, 5, 99, 0]),
            vec![2, 4, 4, 5, 99, 9801]
        );
        assert_eq!(
            run_program(&vec![1, 1, 1, 4, 99, 5, 6, 0, 99]),
            vec![30, 1, 1, 4, 2, 5, 6, 0, 99]
        );
    }
}
