#[allow(dead_code)]
pub fn run_program_with_inputs(original: &Vec<i32>, noun: i32, verb: i32) -> Vec<i32> {
    let mut program = original.clone();
    program[1] = noun;
    program[2] = verb;
    run_program(&program)
}

#[allow(dead_code)]
pub fn run_program_with_inputs_and_get_output(original: &Vec<i32>, noun: i32, verb: i32) -> i32 {
    let result = run_program_with_inputs(original, noun, verb);
    result[0]
}

#[allow(dead_code)]
pub fn run_program_to_get_output(original: &Vec<i32>, desired_output: i32) -> (i32, i32) {
    for i in 0..=99 {
        for j in 0..=99 {
            if run_program_with_inputs_and_get_output(original, i, j) == desired_output {
                return (i, j);
            }
        }
    }
    (0, 0)
}

#[allow(dead_code)]
pub fn run_program(original: &Vec<i32>) -> Vec<i32> {
    let mut program = original.clone();

    let mut pos = 0;
    let mut opcode;

    loop {
        opcode = match pos {
            p if p >= program.len() => break,
            _ => program[pos] as usize,
        };

        let first_index: usize = match pos {
            p if p + 1 >= program.len() => break,
            _ => program[pos + 1] as usize,
        };

        let second_index: usize = match pos {
            p if p + 2 >= program.len() => break,
            _ => program[pos + 2] as usize,
        };

        let result_index: usize = match pos {
            p if p + 3 >= program.len() => break,
            _ => program[pos + 3] as usize,
        };

        // NOTE: Careful, these might panic if index is out of bounds.
        // println!("opcode: {:?}", opcode);
        // println!("first_index: {:?}", first_index);
        // println!("first value: {:?}", program[first_index]);
        // println!("second_index: {:?}", second_index);
        // println!("second value: {:?}", program[second_index]);
        // println!("result_index: {:?}", result_index);

        match opcode {
            1 => {
                program[result_index] = program[first_index] + program[second_index];
                pos += 4;
            },
            2 => {
                program[result_index] = program[first_index] * program[second_index];
                pos += 4;
            },
            99 => break,
            _ => break,
        }
    }

    program
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_run_program() {
        assert_eq!(run_program(&vec![1,9,10,3,2,3,11,0,99,30,40,50]), vec![3500,9,10,70,2,3,11,0,99,30,40,50]);
        assert_eq!(run_program(&vec![1,0,0,0,99]), vec![2,0,0,0,99]);
        assert_eq!(run_program(&vec![2,3,0,3,99]), vec![2,3,0,6,99]);
        assert_eq!(run_program(&vec![2,4,4,5,99,0]), vec![2,4,4,5,99,9801]);
        assert_eq!(run_program(&vec![1,1,1,4,99,5,6,0,99]), vec![30,1,1,4,2,5,6,0,99]);
    }
}
