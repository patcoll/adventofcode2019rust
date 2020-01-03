use std::collections::HashSet;
use std::convert::TryInto;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
pub struct Coordinate {
    pub x: isize,
    pub y: isize,
}

impl Coordinate {
    pub fn manhattan_distance(&self) -> usize {
        (self.x.abs() + self.y.abs()).try_into().unwrap()
    }
}

#[derive(Debug, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Default for Direction {
    fn default() -> Self {
        Direction::Right
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct Path {
    direction: Direction,
    steps: usize,
}

impl From<&str> for Path {
    fn from(text: &str) -> Self {
        let direction: Direction = match text.chars().next() {
            Some('U') => Direction::Up,
            Some('D') => Direction::Down,
            Some('L') => Direction::Left,
            Some('R') => Direction::Right,
            _ => Direction::default(),
        };

        let mut steps: usize = 0;

        if !text.is_empty() {
            steps = match (&text[1..]).parse::<usize>() {
                Ok(num) => num,
                _ => 0,
            }
        }

        Path { direction, steps }
    }
}

impl Path {
    pub fn coordinates(start_at: &Coordinate, path: &Path) -> Vec<Coordinate> {
        let mut coordinates: Vec<Coordinate> = Vec::with_capacity(path.steps);

        let multiplier: i64 =
            if [Direction::Right, Direction::Down].contains(&path.direction) {
                1
            } else {
                -1
            };

        if [Direction::Up, Direction::Down].contains(&path.direction) {
            // y direction
            for i in 0..=path.steps {
                coordinates.push(Coordinate {
                    y: ((start_at.y as i64) + (multiplier * i as i64)) as isize,
                    ..*start_at
                });
            }
        } else {
            // x direction
            for i in 0..=path.steps {
                coordinates.push(Coordinate {
                    x: ((start_at.x as i64) + (multiplier * i as i64)) as isize,
                    ..*start_at
                });
            }
        }

        coordinates
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct Route(Vec<Path>);

impl From<&str> for Route {
    fn from(text: &str) -> Self {
        let paths_text: Vec<&str> = text
            .trim()
            .split(',')
            .map(|path_text| path_text.trim())
            .filter(|path_text| !path_text.is_empty())
            .collect();

        if paths_text.is_empty() {
            return Route::default();
        }

        let content: Vec<Path> = paths_text
            .iter()
            .map(|path_text| Path::from(&path_text[..]))
            .collect();

        Route(content)
    }
}

type CoordinateList = Vec<Coordinate>;

#[derive(Debug, Default)]
pub struct Grid {
    coordinates: Vec<CoordinateList>,
    // coordinate_sets: Vec<HashSet<Coordinate>>,
}

impl From<&Vec<Route>> for Grid {
    fn from(routes: &Vec<Route>) -> Self {
        let content = Vec::with_capacity(routes.len());

        let mut grid = Grid {
            coordinates: content,
        };

        for route in routes.iter() {
            grid.add_route(&route);
        }

        grid
    }
}

impl Grid {
    pub fn count(&self) -> usize {
        self.coordinates.len()
    }

    // Returns index of newly created route.
    pub fn add_route(&mut self, route: &Route) -> usize {
        // Initialize new HashSet.
        let index = self.coordinates.len();
        self.coordinates.push(vec![]);

        // println!("route: {:?}", route);

        let mut coordinates: Vec<Coordinate>;
        let mut coords: &Vec<Coordinate>;

        let origin = self.origin();

        let mut start_at: &Coordinate = &origin;

        self.coordinates[index].push(origin);

        let mut path_count = 0;

        for path in route.0.iter() {
            coordinates = Path::coordinates(start_at, path);

            if coordinates.len() < 2 {
                break;
            }

            coords = &coordinates;

            // Skip first coordinate since it will already be there.
            for c in coords[1..].to_owned() {
                path_count += 1;
                self.coordinates[index].push(c);
            }

            let last_coordinate = match coords.iter().last() {
                Some(c) => c,
                _ => break,
            };

            start_at = last_coordinate;
        }

        // The number of coordinates should always be one more than the number of paths.
        assert_eq!(path_count, self.coordinates[index].len() - 1);

        // println!("self.coordinates[{:?}]: {:?}", index, &self.coordinates[index][0..10]);
        // println!("self.coordinates[{:?}].len(): {:?}", index, &self.coordinates[index].len());

        index
    }

    pub fn intersection(&self) -> HashSet<Coordinate> {
        let content = self.coordinates.clone();

        let mut sets = content
            .into_iter()
            .map(|coordinates| coordinates.into_iter().collect::<HashSet<Coordinate>>())
            .collect::<Vec<HashSet<Coordinate>>>()
            .into_iter();

        let mut result = sets
            .next()
            .map(|set| {
                sets.fold(set, |set1, set2| {
                    set1.intersection(&set2).cloned().collect()
                })
            })
            .expect("No HashSet found");

        // Remove origin.
        result.remove(&self.origin());

        result
    }

    pub fn origin(&self) -> Coordinate {
        Default::default()
    }

    pub fn closest_to_origin_in_intersection(&self) -> Option<Coordinate> {
        let intersection = &self.intersection();

        intersection
            .iter()
            .min_by(|c1, c2| c1.manhattan_distance().cmp(&c2.manhattan_distance()))
            .cloned()
    }

    pub fn intersection_shortest_path(&self) -> usize {
        // Get all intersections.
        let intersection = self.intersection();
        let intersection_vec = &intersection.into_iter().collect::<Vec<Coordinate>>();

        intersection_vec
            .iter()
            .map(|&coordinate| {
                // println!("coordinate: {:?}", coordinate);

                self.coordinates
                    .iter()
                    .map(|coordinate_list| {
                        coordinate_list
                            .iter()
                            .position(|&c| c == coordinate)
                            .expect("Should find coordinate")
                    })
                    .sum::<usize>()
            })
            .min()
            .expect("Expect a usize path length")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_default_coordinate() {
        assert_eq!(Coordinate::default(), Coordinate { x: 0, y: 0 });
    }

    #[test]
    fn test_path_from_text() {
        assert_eq!(Path::from(""), Path::default());

        assert_eq!(
            Path::from("L1005"),
            Path {
                direction: Direction::Left,
                steps: 1005
            }
        );
    }

    #[test]
    fn test_route_from_text() {
        assert_eq!(Route::from(""), Route::default());

        assert_eq!(
            Route::from("L2, U5"),
            Route(vec![
                Path {
                    direction: Direction::Left,
                    steps: 2
                },
                Path {
                    direction: Direction::Up,
                    steps: 5
                },
            ])
        );
    }

    #[test]
    fn test_grid_origin() {
        assert_eq!(Coordinate::default(), Grid::default().origin());
    }

    #[test]
    fn test_grid_intersection() {
        let routes = vec![
            Route::from("L2, U10"),
            Route::from("U2, L3, U2, R3, U2, L3, D200, R500"),
        ];

        let grid = Grid::from(&routes);

        assert_eq!(grid.count(), 2);

        let intersection = grid.intersection();

        assert_eq!(intersection.len(), 3);

        assert!(intersection.contains(&Coordinate { x: -2, y: -2 }));
        assert!(intersection.contains(&Coordinate { x: -2, y: -4 }));
        assert!(intersection.contains(&Coordinate { x: -2, y: -6 }));

        let c = grid
            .closest_to_origin_in_intersection()
            .expect("No closest coordinate found");

        assert_eq!(c, Coordinate { x: -2, y: -2 });
        assert_eq!(c.manhattan_distance(), 4);
    }

    #[test]
    fn test_grid_intersection_shortest_path() {
        let routes = vec![
            Route::from("L2, U10"),
            Route::from("U2, L3, U2, R3, U2, L3, D200, R500"),
        ];

        let grid = Grid::from(&routes);

        assert_eq!(grid.intersection_shortest_path(), 8);
    }
}
