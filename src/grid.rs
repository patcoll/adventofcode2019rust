use crate::program::Number;

#[derive(Debug, Default, PartialEq)]
pub struct Coordinate {
    x: Number,
    y: Number,
}

type GridContent = Vec<Vec<bool>>;

#[derive(Debug, Default)]
pub struct Grid {
    content: Vec<GridContent>,
}

#[derive(Debug, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl std::default::Default for Direction {
    fn default() -> Self { Direction::Right }
}

#[derive(Debug, Default, PartialEq)]
pub struct Path {
    direction: Direction,
    steps: Number,
}

impl Path {
    pub fn from_text(text: &str) -> Self {
        let direction: Direction = match text.chars().next() {
            Some('U') => Direction::Up,
            Some('D') => Direction::Down,
            Some('L') => Direction::Left,
            Some('R') => Direction::Right,
            _ => Direction::default(),
        };

        let mut steps: Number = 0;

        if text.len() > 0 {
            steps = match (&text[1..]).parse::<Number>() {
                Ok(num) => num,
                _ => 0,
            }
        }

        Path { direction, steps }
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct Route {
    content: Vec<Path>,
}

impl Route {
    pub fn from_text(text: &str) -> Self {
        let paths_text: Vec<&str> = text
            .trim()
            .split(",")
            .map(|path_text| path_text.trim())
            .filter(|path_text| path_text.len() > 0)
            .collect();

        if paths_text.len() == 0 {
            return Route::default();
        }

        let content: Vec<Path> = paths_text
            .iter()
            .map(|path_text| Path::from_text(path_text))
            .collect();

        Route { content }
    }
}

impl Grid {
    pub fn add_route(&self, route: &Route) {}

    pub fn origin(&self) -> Coordinate {
        Default::default()
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
    fn test_origin() {
        assert_eq!(Coordinate::default(), Grid::default().origin());
    }

    #[test]
    fn test_path_from_text() {
        assert_eq!(
            Path::from_text(""),
            Path::default(),
        );

        assert_eq!(
            Path::from_text("L1005"),
            Path {
                direction: Direction::Left,
                steps: 1005
            }
        );
    }

    #[test]
    fn test_route_from_text() {
        assert_eq!(
            Route::from_text(""),
            Route::default(),
        );

        assert_eq!(
            Route::from_text("L2, U5"),
            Route {
                content: vec![
                    Path {
                        direction: Direction::Left,
                        steps: 2
                    },
                    Path {
                        direction: Direction::Up,
                        steps: 5
                    },
                ]
            }
        );
    }
}
