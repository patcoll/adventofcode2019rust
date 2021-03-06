use crate::code::Digits;
use itertools::Itertools;
use num::cast::ToPrimitive;
use num::Integer;
use rayon::prelude::*;
use std::collections::HashMap;
use std::convert::TryInto;
use std::sync::mpsc::{channel, Receiver, SendError, Sender, TryRecvError};

const POSITION_MODE: u32 = 0;
const IMMEDIATE_MODE: u32 = 1;
const RELATIVE_MODE: u32 = 2;

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
        map.insert(9, 2);
        map.insert(99, 1);
        map
    };
}

#[derive(Debug)]
pub struct Program {
    pub code: Vec<i64>,
    sender: Sender<i64>,
    receiver: Receiver<i64>,
    output: Vec<i64>,
    pos: usize,
    relative_base: isize,
    finished: bool,
}

impl Clone for Program {
    fn clone(&self) -> Self {
        Program {
            code: self.code.to_owned(),
            ..Default::default()
        }
    }
}

impl Default for Program {
    fn default() -> Self {
        let (sender, receiver) = channel();

        Program {
            code: vec![99],
            sender,
            receiver,
            output: vec![],
            pos: 0,
            relative_base: 0,
            finished: false,
        }
    }
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
    fn new(code: &[i64], inputs: &[i64]) -> Program {
        let (sender, receiver) = channel();

        for input in inputs {
            sender.send(input.clone()).unwrap();
        }

        Program {
            code: code.to_owned(),
            sender,
            receiver,
            ..Default::default()
        }
    }

    pub fn finish(&mut self) {
        self.finished = true;
    }

    pub fn is_finished(&self) -> bool {
        self.finished
    }

    fn get(&self, pos: usize) -> Option<i64> {
        match pos {
            p if p >= self.code.len() => Some(0),
            _ => Some(self.code[pos]),
        }
    }

    pub fn set(&mut self, pos: usize, value: i64) {
        if pos >= self.code.len() {
            let mut expand_with = vec![0; pos - self.code.len()];
            expand_with.push(value);
            self.code.append(&mut expand_with);
            return;
        }

        self.code[pos] = value;
    }

    pub fn all_output(&self) -> &[i64] {
        self.output.as_ref()
    }

    pub fn output(&self) -> Option<i64> {
        if self.output.is_empty() {
            return None;
        }
        Some(self.output[self.output.len() - 1])
    }

    pub fn send_output(&mut self, out: i64) {
        self.output.push(out);
    }

    pub fn send_input<T>(&mut self, input: T) -> Result<(), SendError<i64>>
    where
        T: Integer + ToPrimitive,
    {
        let converted = input.to_i64().unwrap();
        self.sender.send(converted)
    }

    pub fn try_recv_input(&self) -> Result<i64, TryRecvError> {
        self.receiver.try_recv()
    }

    pub fn find_best_phase_settings(&self, amplifier_count: usize) -> (Vec<usize>, i64) {
        (0..amplifier_count)
            .permutations(amplifier_count)
            .map(|permutation| {
                let mut input = 0;

                for phase in &permutation {
                    let mut program = self.clone();
                    let _ = program.send_input(*phase);
                    let _ = program.send_input(input);
                    program.run();
                    input = program.output().expect("expected output");
                }

                (permutation, input)
            })
            .max_by_key(|(_, power)| *power)
            .unwrap()
    }

    pub fn find_best_phase_settings_in_feedback_loop_mode(
        &self,
        amplifier_count: usize,
    ) -> (Vec<usize>, i64) {
        let amp_number = |amps: &[Program], n: usize| -> usize { n % amps.len() };

        (5..5 + amplifier_count)
            .permutations(amplifier_count)
            .map(|permutation| {
                let mut amplifiers = (0..amplifier_count)
                    .map(|_| self.clone())
                    .collect::<Vec<_>>();

                // Send phase values.
                for (j, phase) in (&permutation).iter().enumerate() {
                    let n = amp_number(&amplifiers, j);
                    let program = &mut amplifiers[n];
                    let _ = program.send_input(*phase);
                }

                // Send input and get output in a loop until done.
                let mut input = 0;
                let mut finished = 0;
                let mut j = 0;

                while finished < amplifier_count {
                    let n = amp_number(&amplifiers, j);
                    let program = &mut amplifiers[n];

                    let _ = program.send_input(input);

                    program.run();

                    if program.is_finished() {
                        finished += 1;
                    }

                    input = program.output().expect("expected output");

                    j += 1;
                }

                (permutation, input)
            })
            .max_by_key(|(_, power)| *power)
            .unwrap()
    }

    pub fn run(&mut self) {
        let mut opcode: Opcode;

        loop {
            let pos = self.pos;
            let opcode_value = self.get(pos).unwrap();
            opcode = Opcode::from(opcode_value);

            let mut instruction = Instruction::new(self, opcode);
            instruction.init(pos);

            match instruction.run() {
                Some(new_pos) => {
                    self.pos = new_pos;
                }
                None => break,
            }

            // println!("program: {:?}", self.code);
            // println!();
        }
    }
}

pub fn compose_program_with_noun_and_verb(
    original: &[i64],
    noun: i64,
    verb: i64,
) -> Vec<i64> {
    let mut prog = original.to_owned();
    prog[1] = noun;
    prog[2] = verb;
    prog
}

pub fn run_program_and_get_output(original: &[i64]) -> i64 {
    let result = run_program(original);
    result.code[0]
}

pub fn run_program_to_get_output(
    original: &[i64],
    desired_output: i64,
) -> Option<(i64, i64)> {
    let permutations: Vec<(i64, i64)> =
        (0..=99).permutations(2).map(|v| (v[0], v[1])).collect();

    permutations.into_par_iter().find_first(|(i, j)| {
        let composed = compose_program_with_noun_and_verb(original, *i, *j);
        run_program_and_get_output(&composed) == desired_output
    })
}

pub fn run_program(original: &[i64]) -> Program {
    let mut program = Program::from(original);
    program.run();
    program
}

pub fn run_program_with_input(original: &[i64], input: i64) -> Program {
    run_program_with_inputs(original, &[input])
}

pub fn run_program_with_inputs(original: &[i64], inputs: &[i64]) -> Program {
    let mut program = Program::new(original, inputs);
    program.run();
    program
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

struct Instruction<'a> {
    program: &'a mut Program,
    opcode: Opcode,
    indexes: Vec<Option<usize>>,
    values: Vec<Option<i64>>,
    pos: usize,
}

impl<'a> Instruction<'a> {
    fn new(program: &'a mut Program, opcode: Opcode) -> Instruction<'a> {
        Instruction {
            program,
            opcode,
            indexes: vec![],
            values: vec![],
            pos: 0,
        }
    }

    pub fn init(&mut self, pos: usize) -> &Instruction<'a> {
        self.pos = pos;

        let mut indexes = Vec::new();
        let mut values = Vec::new();

        // println!("opcode.number: {:?}", self.opcode.number);
        // println!("opcode.modes: {:?}", self.opcode.modes);

        for (mode_pos, mode) in self.opcode.modes.iter().enumerate() {
            let parameter_number = mode_pos + 1;
            let value_at_pos = self.program.get(self.pos + parameter_number);

            let (index, value) = match *mode {
                POSITION_MODE | RELATIVE_MODE => {
                    let index = if let Some(v) = value_at_pos {
                        if *mode == RELATIVE_MODE {
                            let sum: i64 = self.program.relative_base as i64 + v;
                            Some(sum.try_into().unwrap())
                        } else {
                            Some(v.try_into().unwrap())
                        }
                    } else {
                        None
                    };

                    let value = if let Some(i) = index {
                        self.program.get(i)
                    } else {
                        None
                    };

                    (index, value)
                }
                IMMEDIATE_MODE => (None, value_at_pos),
                _ => break,
            };

            indexes.push(index);
            values.push(value);
        }

        // println!("indexes: {:?}", indexes);
        // println!("values: {:?}", values);
        // println!("::");

        self.indexes = indexes;
        self.values = values;

        self
    }

    fn run(&mut self) -> Option<usize> {
        match self.opcode.number {
            // add
            1 => {
                if let [Some(first), Some(second), _] = self.values.as_slice() {
                    if let [_, _, Some(result_index)] = self.indexes.as_slice() {
                        self.program.set(*result_index, first + second);
                    }
                };

                Some(self.pos + self.opcode.length)
            }
            // multiply
            2 => {
                if let [Some(first), Some(second), _] = self.values.as_slice() {
                    if let [_, _, Some(result_index)] = self.indexes.as_slice() {
                        self.program.set(*result_index, first * second);
                    }
                };

                Some(self.pos + self.opcode.length)
            }
            // input
            3 => {
                match self.program.try_recv_input() {
                    Ok(input) => {
                        if let [Some(result_index)] = self.indexes.as_slice() {
                            // println!("Input ({:?}) going to index {:?}. pos: {:?}", input, result_index, self.pos);
                            self.program.set(*result_index, input);
                        };

                        Some(self.pos + self.opcode.length)
                    }
                    Err(TryRecvError::Empty) => {
                        // This is valid. Stop program at this instruction.
                        // println!("Input empty. Stopped program at {:?}", self.pos);

                        self.program.finished = false;

                        // Send `None` so program can stop.
                        None
                    }
                    Err(TryRecvError::Disconnected) => {
                        panic!("Channel unexpectedly closed")
                    }
                }
            }
            // output
            4 => {
                if let [Some(out)] = self.values.as_slice() {
                    // println!("[program::out]: {}", out);

                    self.program.send_output(*out);

                    Some(self.pos + self.opcode.length)
                } else {
                    panic!("No output from output instruction");
                }
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
                        match self.values.as_slice() {
                            [Some(first), Some(second), _] if *first < *second => 1,
                            _ => 0,
                        },
                    );
                }

                Some(self.pos + self.opcode.length)
            }
            // equals
            8 => {
                if let [_, _, Some(store_pos)] = self.indexes.as_slice() {
                    self.program.set(
                        (*store_pos) as usize,
                        match self.values.as_slice() {
                            [Some(first), Some(second), _] if *first == *second => 1,
                            _ => 0,
                        },
                    );
                }

                Some(self.pos + self.opcode.length)
            }
            // adjust-relative-base
            9 => {
                if let [Some(adjust_by)] = self.values.as_slice() {
                    self.program.relative_base += *adjust_by as isize;
                }

                Some(self.pos + self.opcode.length)
            }
            // halt
            99 => {
                self.program.finish();

                None
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    // use ntest::*;

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
        assert_eq!(run_program_with_inputs(&program, &[8]).output().unwrap(), 1);
    }

    #[test]
    fn test_equals() {
        // Consider whether the input is equal to 8; output 1 (if it is) or 0
        // (if it is not).

        // position mode
        let program = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        // immediate mode
        let i_program = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];

        assert_eq!(run_program_with_input(&program, 8).output().unwrap(), 1);

        assert_eq!(run_program_with_input(&i_program, 8).output().unwrap(), 1);

        (0..20).filter(|n| *n != 8).for_each(|n| {
            assert_eq!(run_program_with_input(&program, n).output().unwrap(), 0);

            assert_eq!(run_program_with_input(&i_program, n).output().unwrap(), 0);
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

        assert_eq!(run_program_with_input(&program, 7).output().unwrap(), 1);

        assert_eq!(run_program_with_input(&i_program, 7).output().unwrap(), 1);

        (0..20).filter(|n| *n >= 8).for_each(|n| {
            assert_eq!(run_program_with_input(&program, n).output().unwrap(), 0);

            assert_eq!(run_program_with_input(&i_program, n).output().unwrap(), 0);
        });
    }

    #[test]
    fn test_jump_if_true() {
        let program = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
        let i_program = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];

        assert_eq!(run_program_with_input(&program, 0).output().unwrap(), 0);

        assert_eq!(run_program_with_input(&i_program, 0).output().unwrap(), 0);

        assert_eq!(run_program_with_input(&program, 1).output().unwrap(), 1);

        assert_eq!(run_program_with_input(&i_program, 1).output().unwrap(), 1);
    }

    #[test]
    fn test_jump_if_false() {
        let program = vec![3, 12, 5, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
        let i_program = vec![3, 3, 1106, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];

        assert_eq!(run_program_with_input(&program, 0).output().unwrap(), 1);

        assert_eq!(run_program_with_input(&i_program, 0).output().unwrap(), 1);

        assert_eq!(run_program_with_input(&program, 1).output().unwrap(), 0);

        assert_eq!(run_program_with_input(&i_program, 1).output().unwrap(), 0);
    }

    #[test]
    fn test_new_instructions_integration() {
        let program = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0,
            36, 98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46,
            1101, 1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99,
        ];

        assert_eq!(run_program_with_input(&program, 7).output().unwrap(), 999);

        assert_eq!(run_program_with_input(&program, 8).output().unwrap(), 1000);

        assert_eq!(run_program_with_input(&program, 9).output().unwrap(), 1001);
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
            run_program_with_inputs(&program, &[4, 0]).output().unwrap(),
            4
        );

        assert_eq!(
            run_program_with_inputs(&program, &[3, 4]).output().unwrap(),
            43
        );

        assert_eq!(
            run_program_with_inputs(&program, &[2, 43])
                .output()
                .unwrap(),
            432
        );

        assert_eq!(
            run_program_with_inputs(&program, &[1, 432])
                .output()
                .unwrap(),
            4321
        );

        assert_eq!(
            run_program_with_inputs(&program, &[0, 4321])
                .output()
                .unwrap(),
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

        let program3: &[i64] = &[
            3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33,
            7, 33, 1, 33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
        ];
        let prog3 = Program::from(program3);
        let best3 = prog3.find_best_phase_settings(5);
        assert_eq!(best3.0, vec![1, 0, 4, 3, 2]);
        assert_eq!(best3.1, 65210);
    }

    #[test]
    fn test_find_best_phase_settings_in_feedback_loop_mode() {
        let program: &[i64] = &[
            3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001,
            28, -1, 28, 1005, 28, 6, 99, 0, 0, 5,
        ];
        let prog = Program::from(program);
        let best = prog.find_best_phase_settings_in_feedback_loop_mode(5);
        assert_eq!(best.0, vec![9, 8, 7, 6, 5]);
        assert_eq!(best.1, 139_629_729);

        let program2: &[i64] = &[
            3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26,
            1001, 54, -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1,
            55, 2, 53, 55, 53, 4, 53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10,
        ];
        let prog2 = Program::from(program2);
        let best2 = prog2.find_best_phase_settings_in_feedback_loop_mode(5);
        assert_eq!(best2.0, vec![9, 7, 8, 5, 6]);
        assert_eq!(best2.1, 18216);
    }

    #[test]
    fn test_large_numbers() {
        let program: &[i64] = &[1102, 34_915_192, 34_915_192, 7, 4, 7, 99, 0];
        let mut prog = Program::from(program);
        prog.run();

        assert_eq!(prog.output().unwrap(), 1_219_070_632_396_864);
    }

    #[test]
    fn test_large_numbers_2() {
        let program: &[i64] = &[104, 1_125_899_906_842_624, 99];
        let mut prog = Program::from(program);
        prog.run();

        assert_eq!(prog.output().unwrap(), 1_125_899_906_842_624);
    }

    #[test]
    fn test_expandable_memory() {
        let program: &[i64] = &[3, 10, 99];
        let mut prog = Program::new(program, &[111]);
        prog.run();

        assert_eq!(prog.code, &[3, 10, 99, 0, 0, 0, 0, 0, 0, 0, 111]);
    }

    #[test]
    fn test_adjust_relative_base() {
        let program: &[i64] = &[109, 19, 99];

        let mut prog = Program::new(program, &[111]);
        prog.relative_base = 2000;
        prog.run();

        assert_eq!(prog.relative_base, 2019);
    }

    #[test]
    fn test_relative_mode() {
        let program: &[i64] = &[
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];

        let mut prog = Program::from(program);
        prog.run();

        assert_eq!(prog.all_output(), program);
    }
}
