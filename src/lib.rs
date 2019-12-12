mod fuel;
mod grid;
mod program;

use program::Number;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day_01() {
        let input = include_str!("../data/d01.txt").trim().split_whitespace();

        let total: Number = input
            .map(|line| line.parse::<Number>().unwrap())
            .map(|mass| fuel::fuel_needed_for_mass(mass))
            .sum();

        assert_eq!(total, 3318195);
    }

    #[test]
    fn test_day_01_part_2() {
        let input = include_str!("../data/d01.txt").trim().split_whitespace();

        let total: Number = input
            .map(|line| line.parse::<Number>().unwrap())
            .map(|mass| fuel::total_fuel_needed_for_mass(mass))
            .sum();

        assert_eq!(total, 4974428);
    }

    #[test]
    fn test_day_02() {
        let mut program: Vec<Number> = include_str!("../data/d02.txt")
            .trim()
            .split(",")
            .map(|node| node.parse::<Number>().unwrap())
            .collect();

        program[1] = 12;
        program[2] = 2;

        let result = program::run_program(&program);

        assert_eq!(result[0], 9706670);
    }

    #[test]
    fn test_day_02_with_inputs() {
        let program: Vec<Number> = include_str!("../data/d02.txt")
            .trim()
            .split(",")
            .map(|node| node.parse::<Number>().unwrap())
            .collect();

        let result = program::run_program_with_inputs(&program, 12, 2);

        assert_eq!(result[0], 9706670);
    }

    #[test]
    fn test_day_02_part_2() {
        let program: Vec<Number> = include_str!("../data/d02.txt")
            .trim()
            .split(",")
            .map(|node| node.parse::<Number>().unwrap())
            .collect();

        let (noun, verb) = program::run_program_to_get_output(&program, 19690720);

        assert_eq!(noun, 25);
        assert_eq!(verb, 52);
        assert_eq!(100 * noun + verb, 2552);
    }
}
