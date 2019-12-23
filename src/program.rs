use crate::code::Digits;
use itertools::Itertools;
use rayon::prelude::*;
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

#[derive(Debug, Default)]
pub struct Program {
    pub code: Vec<i64>,
    pub inputs: Vec<Option<i64>>,
    output: Vec<Option<i64>>,
}

impl From<&[i64]> for Program {
    fn from(code: &[i64]) -> Self {
        Program {
            code: code.to_owned(),
            ..Default::default()
        }
    }
}

impl Program {
    fn new(code: &[i64], inputs: &[Option<i64>]) -> Program {
        Program {
            code: code.to_owned(),
            inputs: inputs.to_owned(),
            ..Default::default()
        }
    }

    fn get(&self, pos: usize) -> Option<i64> {
        match pos {
            p if p >= self.code.len() => None,
            _ => Some(self.code[pos]),
        }
    }

    pub fn set(&mut self, pos: usize, value: i64) -> Option<i64> {
        match pos {
            p if p >= self.code.len() => None,
            _ => {
                self.code[pos] = value;
                Some(value)
            }
        }
    }

    pub fn output(&self) -> i64 {
        if self.output.is_empty() || self.output[self.output.len() - 1].is_none() {
            panic!("Expected output");
        }
        self.output[self.output.len() - 1].unwrap()
    }

    pub fn find_best_phase_settings(&self, amplifier_count: usize) -> (Vec<usize>, i64) {
        (0..amplifier_count)
            .permutations(amplifier_count)
            .map(|permutation| {
                let mut input = 0;

                for phase in &permutation {
                    // let phase = permutation[i];
                    input = run_program_with_inputs(
                        &self.code,
                        &[Some(*phase as i64), Some(input)],
                    )
                    .output();
                }

                (permutation, input)
            })
            .max_by_key(|(_, power)| *power)
            .unwrap()
    }

    pub fn run() {}
}

pub fn run_program_with_noun_and_verb(original: &[i64], noun: i64, verb: i64) -> Program {
    let mut program = original.to_owned();
    program[1] = noun;
    program[2] = verb;
    run_program(&program)
}

pub fn run_program_with_noun_and_verb_and_get_output(
    original: &[i64],
    noun: i64,
    verb: i64,
) -> i64 {
    let result = run_program_with_noun_and_verb(original, noun, verb);
    result.code[0]
}

pub fn run_program_to_get_output(
    original: &[i64],
    desired_output: i64,
) -> Option<(i64, i64)> {
    let permutations: Vec<(i64, i64)> =
        (0..=99).permutations(2).map(|v| (v[0], v[1])).collect();

    permutations.into_par_iter().find_first(|(i, j)| {
        run_program_with_noun_and_verb_and_get_output(original, *i, *j) == desired_output
    })
}

#[derive(Debug, Default)]
struct Opcode {
    number: i64,
    modes: Vec<u32>,
    length: usize,
}

impl From<i64> for Opcode {
    fn from(opcode: i64) -> Self {
        // println!("opcode: {:?}", opcode);
        let digits = Digits::from(opcode as u32);
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

// #[derive(Default)]
struct Instruction<'a> {
    program: &'a mut Program,
    opcode: Opcode,
    indexes: Vec<Option<usize>>,
    values: Vec<Option<i64>>,
    pos: usize,
}

impl Instruction<'_> {
    fn new(program: &mut Program, opcode: Opcode) -> Instruction<'_> {
        Instruction {
            program,
            opcode,
            indexes: vec![],
            values: vec![],
            pos: 0,
        }
    }

    pub fn init(&mut self, pos: &usize) -> &Instruction<'_> {
        // self.inputs = inputs.to_vec();
        self.pos = *pos;

        let mut indexes = Vec::new();
        let mut values = Vec::new();
        // let mut evaluated_values = Vec::new();

        // println!("opcode.number: {:?}", self.opcode.number);
        // println!("opcode.modes: {:?}", self.opcode.modes);

        for (mode_pos, mode) in self.opcode.modes.iter().enumerate() {
            let parameter_number = mode_pos + 1;
            let value_at_pos = self.program.get(self.pos + parameter_number);

            let (index, value) = match *mode {
                POSITION_MODE => {
                    let index = if let Some(v) = value_at_pos {
                        Some(v as usize)
                    } else {
                        None
                    };

                    // println!("index: {:?}", index);

                    let value = if let Some(i) = index {
                        self.program.get(i)
                    // if i < program.len() {
                    //     self.program.get(i)
                    //     Some(self.program.get(i))
                    // // match (self.opcode.number, parameter_number) {
                    // //     // (1, 3) | (2, 3) => value_at_pos,
                    // //     _ => Some(program[i]),
                    // // }
                    // } else {
                    //     None
                    // }
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

        // println!("indexes: {:?}", indexes);
        // println!("values: {:?}", values);
        // println!("evaluated_values: {:?}", evaluated_values);
        // println!("::");

        self.indexes = indexes;
        self.values = values;
        // self.evaluated_values = evaluated_values;

        self
    }

    fn run(&mut self) -> Option<usize> {
        // program: &mut Vec<i64>,

        // let mut program = &self.program.code;
        // let mut inputs = &self.program.inputs;

        // println!("(inputs): {:?}", self.program.inputs);

        match self.opcode.number {
            1 => {
                if let [Some(first), Some(second), _] = self.values.as_slice() {
                    if let [_, _, Some(result_index)] = self.indexes.as_slice() {
                        self.program.set(*result_index, first + second);
                    }
                };

                Some(self.pos + self.opcode.length)
            }
            2 => {
                if let [Some(first), Some(second), _] = self.values.as_slice() {
                    if let [_, _, Some(result_index)] = self.indexes.as_slice() {
                        self.program.set(*result_index, first * second);
                    }
                };

                // println!("multiply values: {:?}", &self.values[0..2]);
                // let result = &self.values[0..2].iter().fold(1, |acc, n| acc * n.unwrap());
                // // println!("result: {:?}", result);
                // let result_index = self.indexes[2].unwrap();
                // // println!("result_index: {:?}", result_index);
                //
                // self.program.code[result_index] = *result;

                // println!("program: {:?}", self.program.code);
                // println!("");
                Some(self.pos + self.opcode.length)
            }
            3 => {
                let input = self.program.inputs.remove(0);
                // println!("input: {:?}", input);
                // println!("(inputs left): {:?}", self.program.inputs);

                if let [Some(result_index)] = self.indexes.as_slice() {
                    // self.program.code[*result_index] = input.unwrap();
                    self.program.set(*result_index, input.unwrap());
                };

                // let result_index = self.indexes[0].unwrap();
                // self.program.code[result_index] = input.unwrap();

                // println!("program: {:?}", self.program.code);
                // println!("");

                Some(self.pos + self.opcode.length)
            }
            4 => {
                if let [Some(out)] = self.values.as_slice() {
                    // self.program.code[*result_index as usize] = input.unwrap();
                    println!("[program::out]: {}", out);
                    self.program.output.push(Some(*out));
                    Some(self.pos + self.opcode.length)
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
                [Some(param), Some(value)] if *param != 0 => Some(*value as usize),
                _ => Some(self.pos + self.opcode.length),
            },
            // jump-if-false
            6 => match self.values.as_slice() {
                [Some(param), Some(value)] if *param == 0 => Some(*value as usize),
                _ => Some(self.pos + self.opcode.length),
            },
            // less than
            7 => {
                if let [_, _, Some(store_pos)] = self.indexes.as_slice() {
                    self.program.set(
                        (*store_pos) as usize,
                        // input.unwrap()
                        match self.values.as_slice() {
                            [Some(first), Some(second), _] if *first < *second => 1,
                            _ => 0,
                        },
                    );
                    // self.program.code[(*store_pos) as usize] = match self.values.as_slice() {
                    //     [Some(first), Some(second), _] if *first < *second => 1,
                    //     _ => 0,
                    // };
                }

                Some(self.pos + self.opcode.length)
            }
            // equals
            8 => {
                if let [_, _, Some(store_pos)] = self.indexes.as_slice() {
                    // self.program.code[(*store_pos) as usize] = match self.values.as_slice() {
                    //     [Some(first), Some(second), _] if *first == *second => 1,
                    //     _ => 0,
                    // };
                    self.program.set(
                        (*store_pos) as usize,
                        // input.unwrap()
                        match self.values.as_slice() {
                            [Some(first), Some(second), _] if *first == *second => 1,
                            _ => 0,
                        },
                    );
                }

                Some(self.pos + self.opcode.length)
            }
            99 => Default::default(),
            _ => Default::default(),
        }
    }
}

pub fn run_program(original: &[i64]) -> Program {
    run_program_with_input(original, None)
}

pub fn run_program_with_input(original: &[i64], input: Option<i64>) -> Program {
    run_program_with_inputs(original, &[input])
}

pub fn run_program_with_inputs(original: &[i64], inputs: &[Option<i64>]) -> Program {
    // let mut program: Vec<i64> = original.to_owned();

    let mut pos = 0;
    let mut prog = Program::new(&original, inputs);
    let mut opcode: Opcode;
    // let mut output = None;

    loop {
        let opcode_value = prog.get(pos).unwrap();
        opcode = Opcode::from(opcode_value);
        // println!("opcode: {:?}", opcode);
        let mut instruction = Instruction::new(&mut prog, opcode);
        instruction.init(&pos);

        match instruction.run() {
            Some(new_pos) => {
                pos = new_pos;
            }
            None => break,
        }

        // println!("program: {:?}", prog.code);
        // println!();
    }

    // println!("output: {:?}", prog.output);
    // (program, output)
    prog
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_program_instruction() {
        assert_eq!(
            run_program(&[1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]).code,
            vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
        );

        assert_eq!(
            run_program(&[1101, 100, -1, 4, 0]).code,
            vec![1101, 100, -1, 4, 99]
        );
    }

    #[test]
    fn test_run_program_with_inputs() {
        let program = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        assert_eq!(run_program_with_inputs(&program, &[Some(8)]).output(), 1);
    }

    #[test]
    fn test_equals() {
        // Consider whether the input is equal to 8; output 1 (if it is) or 0
        // (if it is not).

        // position mode
        let program = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        // immediate mode
        let i_program = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];

        assert_eq!(run_program_with_input(&program, Some(8)).output(), 1);

        assert_eq!(run_program_with_input(&i_program, Some(8)).output(), 1);

        (0..20).filter(|n| *n != 8).for_each(|n| {
            assert_eq!(run_program_with_input(&program, Some(n)).output(), 0);

            assert_eq!(run_program_with_input(&i_program, Some(n)).output(), 0);
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

        assert_eq!(run_program_with_input(&program, Some(7)).output(), 1);

        assert_eq!(run_program_with_input(&i_program, Some(7)).output(), 1);

        (0..20).filter(|n| *n >= 8).for_each(|n| {
            assert_eq!(run_program_with_input(&program, Some(n)).output(), 0);

            assert_eq!(run_program_with_input(&i_program, Some(n)).output(), 0);
        });
    }

    #[test]
    fn test_jump_if_true() {
        let program = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
        let i_program = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];

        assert_eq!(run_program_with_input(&program, Some(0)).output(), 0);

        assert_eq!(run_program_with_input(&i_program, Some(0)).output(), 0);

        assert_eq!(run_program_with_input(&program, Some(1)).output(), 1);

        assert_eq!(run_program_with_input(&i_program, Some(1)).output(), 1);
    }

    #[test]
    fn test_jump_if_false() {
        let program = vec![3, 12, 5, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
        let i_program = vec![3, 3, 1106, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];

        assert_eq!(run_program_with_input(&program, Some(0)).output(), 1);

        assert_eq!(run_program_with_input(&i_program, Some(0)).output(), 1);

        assert_eq!(run_program_with_input(&program, Some(1)).output(), 0);

        assert_eq!(run_program_with_input(&i_program, Some(1)).output(), 0);
    }

    #[test]
    fn test_new_instructions_integration() {
        let program = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0,
            36, 98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46,
            1101, 1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99,
        ];

        assert_eq!(run_program_with_input(&program, Some(7)).output(), 999);

        assert_eq!(run_program_with_input(&program, Some(8)).output(), 1000);

        assert_eq!(run_program_with_input(&program, Some(9)).output(), 1001);
    }

    #[test]
    fn test_run_program() {
        assert_eq!(
            run_program(&[1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]).code,
            vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
        );
        assert_eq!(run_program(&[1, 0, 0, 0, 99]).code, vec![2, 0, 0, 0, 99]);
        assert_eq!(run_program(&[2, 3, 0, 3, 99]).code, vec![2, 3, 0, 6, 99]);
        assert_eq!(
            run_program(&[2, 4, 4, 5, 99, 0]).code,
            vec![2, 4, 4, 5, 99, 9801]
        );
        assert_eq!(
            run_program(&[1, 1, 1, 4, 99, 5, 6, 0, 99]).code,
            vec![30, 1, 1, 4, 2, 5, 6, 0, 99]
        );
    }

    #[test]
    fn test_phase_settings() {
        let program: &[i64] = &[
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
        ];

        // phase settings: 4,3,2,1,0
        assert_eq!(
            run_program_with_inputs(&program, &[Some(4), Some(0)]).output(),
            4
        );

        assert_eq!(
            run_program_with_inputs(&program, &[Some(3), Some(4)]).output(),
            43
        );

        assert_eq!(
            run_program_with_inputs(&program, &[Some(2), Some(43)]).output(),
            432
        );

        assert_eq!(
            run_program_with_inputs(&program, &[Some(1), Some(432)]).output(),
            4321
        );

        assert_eq!(
            run_program_with_inputs(&program, &[Some(0), Some(4321)]).output(),
            43210
        );
    }

    #[test]
    fn test_find_best_phase_settings() {
        let program: &[i64] = &[
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
        ];
        let prog = Program::from(program);
        let best = prog.find_best_phase_settings(5);
        assert_eq!(best.0, vec![4, 3, 2, 1, 0]);
        assert_eq!(best.1, 43210);

        let program2: &[i64] = &[
            3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23,
            23, 4, 23, 99, 0, 0,
        ];
        let prog2 = Program::from(program2);
        let best2 = prog2.find_best_phase_settings(5);
        assert_eq!(best2.0, vec![0, 1, 2, 3, 4]);
        assert_eq!(best2.1, 54321);

        // let program3: &[i64] = &[3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0];
        // let prog3 = Program::from(program3);
        // assert_eq!(prog3.find_best_phase_settings(5).1, 54321);
        let program3: &[i64] = &[
            3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33,
            7, 33, 1, 33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
        ];
        let prog3 = Program::from(program3);
        let best3 = prog3.find_best_phase_settings(5);
        assert_eq!(best3.0, vec![1, 0, 4, 3, 2]);
        assert_eq!(best3.1, 65210);
    }
}
