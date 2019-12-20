use crate::code::Digits;
use std::collections::HashMap;

const POSITION_MODE: u32 = 0;
const IMMEDIATE_MODE: u32 = 1;

lazy_static! {
    static ref OPCODE_LENGTHS: HashMap<i64, usize> = {
        let mut map = HashMap::new();
        map.insert(1, 4);
        map.insert(2, 4);
        map.insert(3, 2);
        map.insert(4, 2);
        map.insert(5, 3);
        map.insert(6, 3);
        map.insert(7, 4);
        map.insert(8, 4);
        map.insert(99, 1);
        map
    };
}

pub fn run_program_with_inputs(original: &[i64], noun: i64, verb: i64) -> Vec<i64> {
    let mut program = original.to_owned();
    program[1] = noun;
    program[2] = verb;
    run_program(&program)
}

pub fn run_program_with_inputs_and_get_output(
    original: &[i64],
    noun: i64,
    verb: i64,
) -> i64 {
    let result = run_program_with_inputs(original, noun, verb);
    result[0]
}

pub fn run_program_to_get_output(original: &[i64], desired_output: i64) -> (i64, i64) {
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
        // println!("opcode: {:?}", opcode);
        let digits = Digits::from(*opcode as u32);
        let mut iterator = digits.rev();

        // Interpret opcode
        let mut number_string = "".to_string();
        let ones_place = iterator
            .next()
            .expect("need at least one digit for an opcode");
        let tens_place = iterator.next();
        if let Some(t) = tens_place {
            number_string.push_str(&t.to_string());
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

type Routine = dyn FnMut() -> bool;

trait Runnable {
    fn run(
        &mut self,
        program: &mut Vec<i64>,
        input: Option<i64>,
    ) -> (Option<usize>, Option<i64>);
}

// #[derive(Default)]
struct Instruction {
    opcode: Opcode,
    // routine: Box<Routine>,
    indexes: Vec<Option<usize>>,
    values: Vec<Option<i64>>,
    // evaluated_values: Vec<Option<i64>>,
    pos: usize,
}

impl Runnable for Instruction {
    fn run(
        &mut self,
        program: &mut Vec<i64>,
        input: Option<i64>,
    ) -> (Option<usize>, Option<i64>) {
        match self.opcode.number {
            1 => {
                if let [Some(first), Some(second), _] = self.values.as_slice() {
                    if let [_, _, Some(result_index)] = self.indexes.as_slice() {
                        program[*result_index] = first + second;
                    }
                };

                (Some(self.pos + self.opcode.length), None)
            }
            2 => {
                if let [Some(first), Some(second), _] = self.values.as_slice() {
                    if let [_, _, Some(result_index)] = self.indexes.as_slice() {
                        program[*result_index] = first * second;
                    }
                };

                // println!("multiply values: {:?}", &self.values[0..2]);
                // let result = &self.values[0..2].iter().fold(1, |acc, n| acc * n.unwrap());
                // // println!("result: {:?}", result);
                // let result_index = self.indexes[2].unwrap();
                // // println!("result_index: {:?}", result_index);
                //
                // program[result_index] = *result;

                // println!("program: {:?}", program);
                // println!("");
                (Some(self.pos + self.opcode.length), None)
            }
            3 => {
                if let [Some(result_index)] = self.indexes.as_slice() {
                    program[*result_index] = input.unwrap();
                };

                // let result_index = self.indexes[0].unwrap();
                // program[result_index] = input.unwrap();

                // println!("program: {:?}", program);
                // println!("");

                (Some(self.pos + self.opcode.length), None)
            }
            4 => {
                if let [Some(out)] = self.values.as_slice() {
                    // program[*result_index as usize] = input.unwrap();
                    println!("[program::out]: {}", out);
                    (Some(self.pos + self.opcode.length), Some(*out))
                } else {
                    panic!("No output from output instruction");
                }

                // if let Some(out) = self.values[0] {
                // } else {
                //     panic!("No output from output instruction");
                // }
            }
            // jump-if-true
            5 => match self.values.as_slice() {
                [Some(param), Some(value)] if *param != 0 => {
                    (Some(*value as usize), None)
                }
                _ => (Some(self.pos + self.opcode.length), None),
            },
            // jump-if-false
            6 => match self.values.as_slice() {
                [Some(param), Some(value)] if *param == 0 => {
                    (Some(*value as usize), None)
                }
                _ => (Some(self.pos + self.opcode.length), None),
            },
            // less than
            7 => {
                if let [_, _, Some(store_pos)] = self.indexes.as_slice() {
                    program[(*store_pos) as usize] = match self.values.as_slice() {
                        [Some(first), Some(second), _] if *first < *second => {
                            1
                        },
                        _ => 0,
                    };
                }

                (Some(self.pos + self.opcode.length), None)
            }
            // equals
            8 => {
                if let [_, _, Some(store_pos)] = self.indexes.as_slice() {
                    program[(*store_pos) as usize] = match self.values.as_slice() {
                        [Some(first), Some(second), _] if *first == *second => {
                            1
                        },
                        _ => 0,
                    };
                }

                (Some(self.pos + self.opcode.length), None)
            }
            99 => Default::default(),
            _ => Default::default(),
        }
    }
}

impl Instruction {
    fn new(opcode: Opcode, _routine: Box<Routine>) -> Instruction {
        Instruction {
            opcode,
            // routine,
            indexes: vec![],
            values: vec![],
            // evaluated_values: vec![],
            pos: 0,
        }
    }

    pub fn init(&mut self, program: &[i64], pos: &usize) -> &Instruction {
        self.pos = *pos;

        let mut indexes = Vec::new();
        let mut values = Vec::new();
        // let mut evaluated_values = Vec::new();

        println!("opcode.number: {:?}", self.opcode.number);
        println!("opcode.modes: {:?}", self.opcode.modes);

        for (mode_pos, mode) in self.opcode.modes.iter().enumerate() {
            let parameter_number = mode_pos + 1;
            let value_at_pos = get_at_position(&program, self.pos + parameter_number);

            let (index, value) = match *mode {
                POSITION_MODE => {
                    let index = if let Some(v) = value_at_pos {
                        Some(v as usize)
                    } else {
                        None
                    };

                    // println!("index: {:?}", index);

                    let value =
                        if let Some(i) = index {
                            if i < program.len() {
                                Some(program[i])
                                // match (self.opcode.number, parameter_number) {
                                //     // (1, 3) | (2, 3) => value_at_pos,
                                //     _ => Some(program[i]),
                                // }
                            } else {
                                None
                            }
                        } else {
                            None
                        };

                    // println!("value: {:?}", value);

                    // let evaluated_value =
                    //     if let Some(_) = value {
                    //         match (self.opcode.number, parameter_number) {
                    //             (1, 3) | (2, 3) | (3, 1) => value_at_pos,
                    //             _ => value,
                    //         }
                    //     } else {
                    //         None
                    //     };

                    (index, value)
                }
                IMMEDIATE_MODE => (None, value_at_pos),
                _ => break,
            };

            indexes.push(index);
            values.push(value);
            // evaluated_values.push(evaluated_value);
        }

        println!("indexes: {:?}", indexes);
        println!("values: {:?}", values);
        // println!("evaluated_values: {:?}", evaluated_values);
        println!("::");

        self.indexes = indexes;
        self.values = values;
        // self.evaluated_values = evaluated_values;

        self
    }
}

fn get_at_position(program: &[i64], pos: usize) -> Option<i64> {
    match pos {
        p if p >= program.len() => None,
        _ => Some(program[pos]),
    }
}

pub fn run_program(original: &[i64]) -> Vec<i64> {
    let (program, _) = run_program_with_input_instruction(original, None);
    program
}

pub fn run_program_with_input_instruction(
    original: &[i64],
    input: Option<i64>,
) -> (Vec<i64>, Option<i64>) {
    let mut program: Vec<i64> = original.to_owned();

    let mut pos = 0;
    let mut opcode: Opcode;
    let mut output = None;

    loop {
        let opcode_value = get_at_position(&program, pos).unwrap();
        opcode = Opcode::from(&opcode_value);
        // println!("opcode: {:?}", opcode);
        let mut instruction = Instruction::new(opcode, Box::new(|| true));
        instruction.init(&program, &pos);
        // println!("instruction: {:?}", instruction);

        // hard code input for now
        match instruction.run(&mut program, input) {
            (Some(new_pos), optional_output) => {
                if let Some(o) = optional_output {
                    output = Some(o);
                }
                pos = new_pos;
            }
            (None, _) => break,
        }

        println!("program: {:?}", program);
        println!();
    }

    // println!("output: {:?}", output);
    (program, output)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_program_instruction() {
        assert_eq!(
            run_program(&[1,9,10,3,2,3,11,0,99,30,40,50]),
            vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
        );

        assert_eq!(
            run_program(&[1101, 100, -1, 4, 0]),
            vec![1101, 100, -1, 4, 99]
        );
    }

    #[test]
    fn test_equals() {
        // Consider whether the input is equal to 8; output 1 (if it is) or 0
        // (if it is not).

        // position mode
        let program = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        // immediate mode
        let i_program = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];

        assert_eq!(
            run_program_with_input_instruction(&program, Some(8))
                .1
                .unwrap(),
            1
        );

        assert_eq!(
            run_program_with_input_instruction(&i_program, Some(8))
                .1
                .unwrap(),
            1
        );

        (0..20).filter(|n| *n != 8).for_each(|n| {
            assert_eq!(
                run_program_with_input_instruction(&program, Some(n))
                    .1
                    .unwrap(),
                0
            );

            assert_eq!(
                run_program_with_input_instruction(&i_program, Some(n))
                    .1
                    .unwrap(),
                0
            );
        });
    }

    #[test]
    fn test_less_than() {
        // Consider whether the input is less than 8; output 1 (if it is) or 0
        // (if it is not).
        //
        // position mode
        let program = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
        // immediate mode
        let i_program = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];

        assert_eq!(
            run_program_with_input_instruction(&program, Some(7))
                .1
                .unwrap(),
            1
        );

        assert_eq!(
            run_program_with_input_instruction(&i_program, Some(7))
                .1
                .unwrap(),
            1
        );

        (0..20).filter(|n| *n >= 8).for_each(|n| {
            assert_eq!(
                run_program_with_input_instruction(&program, Some(n))
                    .1
                    .unwrap(),
                0
            );

            assert_eq!(
                run_program_with_input_instruction(&i_program, Some(n))
                    .1
                    .unwrap(),
                0
            );
        });
    }

    #[test]
    fn test_jump_if_true() {
        let program = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
        let i_program = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];

        assert_eq!(
            run_program_with_input_instruction(&program, Some(0))
                .1
                .unwrap(),
            0
        );

        assert_eq!(
            run_program_with_input_instruction(&i_program, Some(0))
                .1
                .unwrap(),
            0
        );

        assert_eq!(
            run_program_with_input_instruction(&program, Some(1))
                .1
                .unwrap(),
            1
        );

        assert_eq!(
            run_program_with_input_instruction(&i_program, Some(1))
                .1
                .unwrap(),
            1
        );
    }

    #[test]
    fn test_jump_if_false() {
        let program = vec![3, 12, 5, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
        let i_program = vec![3, 3, 1106, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];

        assert_eq!(
            run_program_with_input_instruction(&program, Some(0))
                .1
                .unwrap(),
            1
        );

        assert_eq!(
            run_program_with_input_instruction(&i_program, Some(0))
                .1
                .unwrap(),
            1
        );

        assert_eq!(
            run_program_with_input_instruction(&program, Some(1))
                .1
                .unwrap(),
            0
        );

        assert_eq!(
            run_program_with_input_instruction(&i_program, Some(1))
                .1
                .unwrap(),
            0
        );
    }

    #[test]
    fn test_new_instructions_integration() {
        let program = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0,
            36, 98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46,
            1101, 1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99,
        ];

        assert_eq!(
            run_program_with_input_instruction(&program, Some(7))
                .1
                .unwrap(),
            999
        );

        assert_eq!(
            run_program_with_input_instruction(&program, Some(8))
                .1
                .unwrap(),
            1000
        );

        assert_eq!(
            run_program_with_input_instruction(&program, Some(9))
                .1
                .unwrap(),
            1001
        );
    }

    #[test]
    fn test_run_program() {
        assert_eq!(
            run_program(&[1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]),
            vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
        );
        assert_eq!(run_program(&[1, 0, 0, 0, 99]), vec![2, 0, 0, 0, 99]);
        assert_eq!(run_program(&[2, 3, 0, 3, 99]), vec![2, 3, 0, 6, 99]);
        assert_eq!(
            run_program(&[2, 4, 4, 5, 99, 0]),
            vec![2, 4, 4, 5, 99, 9801]
        );
        assert_eq!(
            run_program(&[1, 1, 1, 4, 99, 5, 6, 0, 99]),
            vec![30, 1, 1, 4, 2, 5, 6, 0, 99]
        );
    }
}
