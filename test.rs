    fn run(&mut self, program: &mut Vec<i64>, input: Option<i64>) -> (Option<usize>, Option<i64>) {
        match self.opcode.number {
            1 => {
                match self.evaluated_values.as_slice() {
                    [Some(first), Some(second), Some(result_index)] => {
                        let result = &self.evaluated_values[0..2].iter().fold(0, |acc, n| acc + n.unwrap());
                        program[*result_index as usize] = first + second;
                    },
                    _ => (),
                }
                // println!("self: {:?}", self.evaluated_values);
                // println!("add values: {:?}", &self.values[0..2]);
                // let result = &self.evaluated_values[0..2].iter().fold(0, |acc, n| acc + n.unwrap());
                // println!("result: {:?}", result);
                // let result_index = self.evaluated_values[2].unwrap() as usize;
                // println!("result_index: {:?}", result_index);

                // program[result_index] = *result;

                // println!("program: {:?}", program);
                // println!("");
                (Some(self.pos + self.opcode.length), None)
            },
            2 => {
                match self.evaluated_values.as_slice() {
                    [Some(first), Some(second), Some(result_index)] => {
                        let result = &self.evaluated_values[0..2].iter().fold(0, |acc, n| acc + n.unwrap());
                        program[*result_index as usize] = first * second;
                    },
                    _ => (),
                }

                // println!("multiply values: {:?}", &self.values[0..2]);
                // let result = &self.evaluated_values[0..2].iter().fold(1, |acc, n| acc * n.unwrap());
                // println!("result: {:?}", result);
                // let result_index = self.evaluated_values[2].unwrap() as usize;
                // println!("result_index: {:?}", result_index);

                // program[result_index] = *result;

                // println!("program: {:?}", program);
                // println!("");
                (Some(self.pos + self.opcode.length), None)
            },
            3 => {
                match self.values.as_slice() {
                    [Some(result_index)] => {
                        program[*result_index as usize] = input.unwrap();
                    },
                    _ => (),
                }

                println!("program: {:?}", program);
                println!("");

                (Some(self.pos + self.opcode.length), None)
            },
            4 => {
                match self.values.as_slice() {
                    [Some(out)] => {
                        println!("[program::out]: {}", out);
                        (Some(self.pos + self.opcode.length), Some(*out))
                    },
                    _ => panic!("No output from output instruction"),
                }
            },
            // jump-if-true
            5 => {
                match self.values.as_slice() {
                    [Some(param), Some(value)] if *param != 0 => {
                        (Some(*value as usize), None)
                    },
                    _ => (Some(self.pos + self.opcode.length), None),
                }
            },
            // jump-if-false
            6 => {
                match self.values.as_slice() {
                    [Some(param), Some(value)] if *param == 0 => {
                        (Some(*value as usize), None)
                    },
                    _ => (Some(self.pos + self.opcode.length), None),
                }
            },
            // less than
            7 => {
                // let values_as_slice = self.values.as_slice();

                match self.evaluated_values.as_slice() {
                    [Some(first), Some(second), Some(store_pos)] => {
                        program[(*store_pos) as usize] =
                            if *first < *second {
                                1
                            } else {
                                0
                            };
                    },
                    _ => (),
                }

                (Some(self.pos + self.opcode.length), None)
            },
            // equals
            8 => {
                // let values_as_slice = self.values.as_slice();
                // println!("values_as_slice: {:?}", values_as_slice);

                match self.evaluated_values.as_slice() {
                    [Some(first), Some(second), Some(store_pos)] => {
                        program[(*store_pos) as usize] =
                            if *first == *second {
                                1
                            } else {
                                0
                            };
                    },
                    _ => (),
                }

                // println!("program: {:?}", program);
                // println!("");

                (Some(self.pos + self.opcode.length), None)
            },
            // 6 => {
            //     let new_pos =
            //         let Some(param) = self.values[0];
            //         if param == 0 {
            //             self.values[1].expect("expected second parameter for jump-if-true");
            //         } else {
            //             Some(*pos + self.opcode.length)
            //         }
            //     (new_pos, None)
            // },
            // 7 => {
            //     let new_pos =
            //         let Some(param) = self.values[0];
            //         if param == 0 {
            //             self.values[1].expect("expected second parameter for jump-if-true");
            //         } else {
            //             Some(*pos + self.opcode.length)
            //         }
            //     (new_pos, None)
            // },
            99 => Default::default(),
            _ => Default::default(),
        }
    }
