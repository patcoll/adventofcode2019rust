pub mod code;
pub mod fuel;
pub mod grid;
pub mod orbits;
pub mod program;

#[macro_use]
extern crate lazy_static;

#[cfg(test)]
mod test {
    use super::*;

    use grid::{Coordinate, Grid, Route};
    use orbits::Universe;

    #[test]
    fn test_day_01() {
        let input = include_str!("../data/d01.txt").trim().split_whitespace();

        let total: i64 = input
            .map(|line| line.parse::<i64>().unwrap())
            .map(fuel::fuel_needed_for_mass)
            .sum();

        assert_eq!(total, 3_318_195);
    }

    #[test]
    fn test_day_01_part_2() {
        let input = include_str!("../data/d01.txt").trim().split_whitespace();

        let total: i64 = input
            .map(|line| line.parse::<i64>().unwrap())
            .map(fuel::total_fuel_needed_for_mass)
            .sum();

        assert_eq!(total, 4_974_428);
    }

    #[test]
    fn test_day_02() {
        let mut program: Vec<i64> = include_str!("../data/d02.txt")
            .trim()
            .split(',')
            .map(|node| node.parse::<i64>().unwrap())
            .collect();

        program[1] = 12;
        program[2] = 2;

        let result = program::run_program(&program);

        assert_eq!(result.code[0], 9_706_670);
    }

    #[test]
    fn test_day_02_with_noun_and_verb() {
        let program: Vec<i64> = include_str!("../data/d02.txt")
            .trim()
            .split(',')
            .map(|node| node.parse::<i64>().unwrap())
            .collect();

        let result = program::run_program_with_noun_and_verb(&program, 12, 2);

        assert_eq!(result.code[0], 9_706_670);
    }

    #[test]
    fn test_day_02_part_2() {
        let program: Vec<i64> = include_str!("../data/d02.txt")
            .trim()
            .split(',')
            .map(|node| node.parse::<i64>().unwrap())
            .collect();

        let (noun, verb) = program::run_program_to_get_output(&program, 19_690_720);

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

        let c1 = test_grid_1
            .closest_to_origin_in_intersection()
            .expect("No closest coordinate found");
        assert_eq!(c1.manhattan_distance(), 159);

        let test_grid_2 = Grid::from(&vec![
            Route::from("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51"),
            Route::from("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"),
        ]);

        let c2 = test_grid_2
            .closest_to_origin_in_intersection()
            .expect("No closest coordinate found");
        assert_eq!(c2.manhattan_distance(), 135);

        // Use data now.
        let routes: Vec<Route> = include_str!("../data/d03.txt")
            .trim()
            .split_whitespace()
            .map(Route::from)
            .collect();

        let grid = Grid::from(&routes);

        let c = grid
            .closest_to_origin_in_intersection()
            .expect("No closest coordinate found");
        assert_eq!(c, Coordinate { x: -369, y: 6 });
        assert_eq!(c.manhattan_distance(), 375);
    }

    #[test]
    fn test_day_03_part_2() {
        let test_grid_1 = Grid::from(&vec![
            Route::from("R75,D30,R83,U83,L12,D49,R71,U7,L72"),
            Route::from("U62,R66,U55,R34,D71,R55,D58,R83"),
        ]);

        assert_eq!(test_grid_1.intersection_shortest_path(), 610);

        let test_grid_2 = Grid::from(&vec![
            Route::from("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51"),
            Route::from("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"),
        ]);

        assert_eq!(test_grid_2.intersection_shortest_path(), 410);

        // Use data now.
        let routes: Vec<Route> = include_str!("../data/d03.txt")
            .trim()
            .split_whitespace()
            .map(Route::from)
            .collect();

        let grid = Grid::from(&routes);

        assert_eq!(grid.intersection_shortest_path(), 14746);
    }

    #[test]
    #[ignore]
    fn test_day_04() {
        assert_eq!(code::find_candidates(273_025..=767_253).len(), 910);
        assert_eq!(code::find_candidates(357_253..=892_942).len(), 530);
    }

    #[test]
    fn test_day_04_alternate() {
        assert_eq!(code::find_candidates2(273_025..=767_253).len(), 910);
        assert_eq!(code::find_candidates2(357_253..=892_942).len(), 530);
    }

    #[test]
    #[ignore]
    fn test_day_04_part_2() {
        assert_eq!(
            code::find_candidates_with_one_dup(273_025..=767_253).len(),
            598
        );
    }

    #[test]
    fn test_day_04_part_2_alternate() {
        assert_eq!(
            code::find_candidates_with_one_dup2(273_025..=767_253).len(),
            598
        );
    }

    #[test]
    fn test_day_05() {
        let program: Vec<i64> = include_str!("../data/d05.txt")
            .trim()
            .split(',')
            .map(|node| node.parse::<i64>().unwrap())
            .collect();

        assert_eq!(program::run_program_with_input(&program, Some(1)).output(), 16_574_641);
    }

    #[test]
    fn test_day_05_part_2() {
        let program: Vec<i64> = include_str!("../data/d05.txt")
            .trim()
            .split(',')
            .map(|node| node.parse::<i64>().unwrap())
            .collect();

        assert_eq!(program::run_program_with_input(&program, Some(5)).output(), 15_163_975);
    }

    #[test]
    #[ignore]
    fn test_day_06() {
        let input_str = include_str!("../data/d06.txt");
        let universe = Universe::from(input_str);

        assert_eq!(universe.count_objects(), 1800);
        assert_eq!(universe.count_direct_orbits(), 1799);
        assert_eq!(universe.count_indirect_orbits(), 315_757);
    }

    #[test]
    fn test_day_06_part_2() {
        let input_str = include_str!("../data/d06.txt");
        let universe = Universe::from(input_str);

        assert_eq!(
            universe.get_minimal_orbital_transfer_count("YOU", "SAN"),
            Some(481)
        );
    }
}
