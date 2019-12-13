mod fuel;
mod grid;
mod program;

use grid::{Grid, Route, Coordinate};
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

    #[test]
    fn test_day_03() {
        let test_grid_1 = Grid::from(&vec![
            Route::from("R75,D30,R83,U83,L12,D49,R71,U7,L72"),
            Route::from("U62,R66,U55,R34,D71,R55,D58,R83"),
        ]);

        match test_grid_1.closest_to_origin_in_intersection() {
            Some(c) => {
                assert_eq!(c.distance(), 159);
            },
            _ => panic!("No closest coordinate found"),
        }

        let test_grid_2 = Grid::from(&vec![
            Route::from("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51"),
            Route::from("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"),
        ]);

        match test_grid_2.closest_to_origin_in_intersection() {
            Some(c) => {
                assert_eq!(c.distance(), 135);
            },
            _ => panic!("No closest coordinate found"),
        }

        // Use data now.
        let routes: Vec<Route> = include_str!("../data/d03.txt")
            .trim()
            .split_whitespace()
            .map(|line| Route::from(line))
            .collect();

        let grid = Grid::from(&routes);

        match grid.closest_to_origin_in_intersection() {
            Some(c) => {
                assert_eq!(c, Coordinate { x: -369, y: 6 });
                assert_eq!(c.distance(), 375);
            },
            _ => panic!("No closest coordinate found"),
        }
    }
}
