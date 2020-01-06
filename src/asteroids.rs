use crate::grid::Coordinate;
use num::{Rational, Zero};
use rayon::prelude::*;
use std::cmp::{Ord, Ordering, PartialOrd};
use std::collections::HashSet;
use std::convert::TryInto;
use std::error::Error;

fn to_cartesian(x: isize, y: isize) -> (isize, isize) {
    (x, -y)
}

impl Coordinate {
    pub fn angle(&self) -> f64 {
        let (x, y) = to_cartesian(self.x, self.y);

        let radians = (x as f64).atan2(y as f64);

        // unnecessary, but it helped with debugging.
        let mut deg = (radians * 180.0) / std::f64::consts::PI;

        if deg < 0.0 {
            deg += 360.0;
        }

        deg
    }
}

impl Ord for Coordinate {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.eq(&other) {
            return Ordering::Equal;
        }
        self.angle().partial_cmp(&other.angle()).unwrap()
    }
}

impl PartialOrd for Coordinate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct InvalidCoordinateError;

impl Error for InvalidCoordinateError {}

impl std::fmt::Display for InvalidCoordinateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "That coordinate is not in the region")
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Slope {
    x: isize,
    y: isize,
}

impl Slope {
    fn new(from: &Coordinate, to: &Coordinate) -> Self {
        let y_diff = to.y - from.y;
        let y_diff_is_neg = y_diff < 0;

        let x_diff = to.x - from.x;
        let x_diff_is_neg = x_diff < 0;

        // (y, x)
        let (mut numer, mut denom) = if to.x - from.x == Zero::zero() {
            (1, 0)
        } else {
            let rational = Rational::new(to.y - from.y, to.x - from.x);

            let numer = *rational.numer();
            let denom = *rational.denom();

            (numer, denom)
        };

        // Rational reduces too much and removes the negation. Correct it.
        // Also negate our raw (1, 0) slope appropriately.
        if y_diff_is_neg && numer > 0 || !y_diff_is_neg && numer < 0 {
            numer = -numer;
        }
        if x_diff_is_neg && denom > 0 || !x_diff_is_neg && denom < 0 {
            denom = -denom;
        }

        Slope { y: numer, x: denom }
    }
}

#[derive(Debug, Default)]
pub struct Obliteration {
    region: Region,
    from: Coordinate,
    visible_from: Vec<Coordinate>,
}

impl Obliteration {
    pub fn new(
        region: &Region,
        from: &Coordinate,
    ) -> Result<Self, InvalidCoordinateError> {
        if !region.contains(from) {
            return Err(InvalidCoordinateError);
        }

        Ok(Obliteration {
            region: region.clone(),
            from: from.clone(),
            ..Default::default()
        })
    }

    fn populate_visible_from(&mut self) -> bool {
        if !self.visible_from.is_empty() {
            return false;
        }

        let mut visible_from = self
            .region
            .visible_from(&self.from)
            .unwrap()
            .into_par_iter()
            .map(|c| Coordinate {
                x: c.x - self.from.x,
                y: c.y - self.from.y,
            })
            .collect::<Vec<_>>();

        visible_from.sort();

        visible_from = visible_from
            .into_par_iter()
            .map(|c| Coordinate {
                x: c.x + self.from.x,
                y: c.y + self.from.y,
            })
            .collect::<Vec<_>>();

        self.visible_from = visible_from;

        true
    }
}

impl Iterator for Obliteration {
    type Item = Coordinate;

    fn next(&mut self) -> Option<Self::Item> {
        if self.visible_from.is_empty() {
            self.populate_visible_from();
        }

        if self.visible_from.is_empty() {
            return None;
        }

        let result = self.visible_from.remove(0);

        self.region.remove(&result);

        Some(result)
    }
}

#[derive(Clone, Debug, Default)]
pub struct Region {
    pub coordinates: HashSet<Coordinate>,
    pub width: usize,
    pub height: usize,
}

impl Region {
    fn is_asteroid(ch: char) -> bool {
        ch == '#'
    }

    pub fn len(&self) -> usize {
        self.coordinates.len()
    }

    pub fn is_empty(&self) -> bool {
        self.coordinates.is_empty()
    }

    pub fn remove(&mut self, coord: &Coordinate) -> bool {
        self.coordinates.remove(coord)
    }

    pub fn contains(&self, coord: &Coordinate) -> bool {
        self.coordinates.contains(coord)
    }

    pub fn could_contain(&self, coord: &Coordinate) -> bool {
        coord.x >= 0
            && coord.x < self.width.try_into().unwrap()
            && coord.y >= 0
            && coord.y < self.height.try_into().unwrap()
    }

    pub fn derive_full_path(
        &self,
        from: &Coordinate,
        to: &Coordinate,
    ) -> Result<Vec<Coordinate>, InvalidCoordinateError> {
        if !self.contains(from) || !self.contains(to) {
            return Err(InvalidCoordinateError);
        }

        let slope = Slope::new(from, to);

        let Coordinate { mut x, mut y } = from;

        let mut coordinates = Vec::new();

        while self.could_contain(&Coordinate { x, y }) {
            // println!("    coord: {:?}", Coordinate { x, y });

            coordinates.push(Coordinate { x, y });

            if x == to.x && y == to.y {
                break;
            }

            x += slope.x;
            y += slope.y;
        }

        // println!("    coordinates before: {:?}", coordinates);

        coordinates = coordinates
            .into_iter()
            .filter(|coord| self.contains(&coord))
            .collect::<Vec<_>>();

        // println!("    coordinates after: {:?}", coordinates);

        // TODO: Optimize: See if `to` is reachable at all from `from` given the slope.

        Ok(coordinates)
    }

    pub fn can_see(
        &self,
        from: &Coordinate,
        to: &Coordinate,
    ) -> Result<bool, InvalidCoordinateError> {
        // println!("can_see from: {:?}, to: {:?}", from, to);
        let full_path = self.derive_full_path(from, to)?;
        // println!("    can_see full_path: {:?}", full_path);
        Ok(full_path.len() == 2 && full_path[1] == *to)
    }

    pub fn visible_from(
        &self,
        coord: &Coordinate,
    ) -> Result<Vec<Coordinate>, InvalidCoordinateError> {
        if !self.contains(coord) {
            return Err(InvalidCoordinateError);
        }

        let mut visible = Vec::new();

        for c in &self.coordinates {
            if coord == c {
                continue;
            }
            // println!("  {:?} checking against: {:?}", coord, c);
            if let Ok(_can_see) = self.can_see(&coord, &c) {
                if _can_see {
                    visible.push(c.clone());
                // println!("    CAN SEE!");
                // can_see_for_coord += 1;
                } else {
                    // println!("    can't see.");
                }
            }
        }

        // println!("can_see from: {:?}, to: {:?}", from, to);
        // let full_path = self.derive_full_path(from, to)?;
        // println!("can_see full_path: {:?}", full_path);
        // Ok(full_path.len() == 2 && full_path[1] == *to)

        Ok(visible)
    }

    pub fn max_visible_from_count(&self) -> (Option<&Coordinate>, usize) {
        if self.is_empty() {
            return (None, 0);
        }

        let mut max_coord = None;
        let mut max = 0;

        for coord in &self.coordinates {
            // println!("coord: {:?}", coord);
            let can_see_for_coord = self.visible_from(coord).unwrap().len();

            if can_see_for_coord > max {
                max_coord = Some(coord);
                max = can_see_for_coord;
            }
        }

        (max_coord, max)
    }
}

impl From<&str> for Region {
    fn from(lines: &str) -> Self {
        let mut coordinates: HashSet<Coordinate> = HashSet::new();

        let rows = lines.trim().split_whitespace().collect::<Vec<_>>();
        let height = rows.len();
        let width = rows[0].len();

        for (y, line) in rows.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                if Self::is_asteroid(ch) {
                    let coordinate = Coordinate {
                        x: x.try_into().unwrap(),
                        y: y.try_into().unwrap(),
                    };
                    coordinates.insert(coordinate);
                }
            }
        }

        Region {
            width,
            height,
            coordinates,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_slope() {
        let slope1 = Slope::new(&Coordinate { x: 2, y: 2 }, &Coordinate { x: 3, y: 3 });
        assert_eq!(slope1, Slope { x: 1, y: 1 });

        let slope1 = Slope::new(&Coordinate { x: 2, y: 2 }, &Coordinate { x: 2, y: 3 });
        assert_eq!(slope1, Slope { x: 0, y: 1 });

        let slope1 = Slope::new(&Coordinate { x: 2, y: 2 }, &Coordinate { x: 1, y: 3 });
        assert_eq!(slope1, Slope { x: -1, y: 1 });

        let slope1 = Slope::new(&Coordinate { x: 2, y: 2 }, &Coordinate { x: 1, y: 2 });
        assert_eq!(slope1, Slope { x: -1, y: 0 });

        let slope1 = Slope::new(&Coordinate { x: 2, y: 2 }, &Coordinate { x: 1, y: 1 });
        assert_eq!(slope1, Slope { x: -1, y: -1 });

        let slope1 = Slope::new(&Coordinate { x: 2, y: 2 }, &Coordinate { x: 2, y: 1 });
        assert_eq!(slope1, Slope { x: 0, y: -1 });

        let slope1 = Slope::new(&Coordinate { x: 2, y: 2 }, &Coordinate { x: 3, y: 1 });
        assert_eq!(slope1, Slope { x: 1, y: -1 });
    }

    #[test]
    fn test_to_cartesian() {
        assert_eq!(to_cartesian(1, -1), (1, 1));
    }

    #[test]
    fn test_region() {
        let simple_map = "
            .....
            .....
            .....
        ";
        let simple_region = Region::from(simple_map);
        assert_eq!(simple_region.len(), 0);
        assert_eq!(simple_region.is_empty(), true);
        assert_eq!(simple_region.width, 5);
        assert_eq!(simple_region.height, 3);

        let map = "
            .#..#
            .....
            #####
            ....#
            ...##
        ";

        let region = Region::from(map);

        assert_eq!(region.len(), 10);
        assert_eq!(region.is_empty(), false);
        assert_eq!(region.width, 5);
        assert_eq!(region.height, 5);

        let from = Coordinate { x: 1, y: 0 };
        let to = Coordinate { x: 3, y: 4 };

        assert_eq!(region.contains(&from), true);
        assert_eq!(region.contains(&to), true);
        assert_eq!(region.contains(&Coordinate { x: 0, y: 0 }), false);
    }

    #[test]
    fn test_derive_full_path() {
        let map = "
            .#...
            .....
            ..#..
            .....
            ...#.
        ";

        let region = Region::from(map);

        let from = Coordinate { x: 1, y: 0 };
        let to = Coordinate { x: 3, y: 4 };

        let full_path = region.derive_full_path(&from, &to).unwrap();

        assert_eq!(full_path.len(), 3);
        assert_eq!(full_path.contains(&Coordinate { x: 2, y: 2 }), true);
    }

    #[test]
    fn test_derive_full_path_div_zero() {
        let map = "
            #.
            #.
        ";

        let region = Region::from(map);

        let from = Coordinate { x: 0, y: 0 };
        let to = Coordinate { x: 0, y: 1 };

        let full_path = region.derive_full_path(&from, &to).unwrap();

        assert_eq!(full_path.len(), 2);
        assert_eq!(full_path.contains(&Coordinate { x: 0, y: 0 }), true);
        assert_eq!(full_path.contains(&Coordinate { x: 0, y: 1 }), true);
    }

    #[test]
    fn test_derive_full_path_with_gaps() {
        let map = "
            #....
            .....
            ..#..
            .....
            ....#
        ";

        let region = Region::from(map);

        let from = Coordinate { x: 4, y: 4 };
        let to = Coordinate { x: 0, y: 0 };

        let full_path = region.derive_full_path(&from, &to).unwrap();

        assert_eq!(full_path.len(), 3);
        assert_eq!(full_path.contains(&Coordinate { x: 4, y: 4 }), true);
        assert_eq!(full_path.contains(&Coordinate { x: 2, y: 2 }), true);
        assert_eq!(full_path.contains(&Coordinate { x: 0, y: 0 }), true);
    }

    #[test]
    fn test_derive_full_path_with_test_data() {
        let map = "
            .#..#
            .....
            #####
            ....#
            ...##
        ";

        let region = Region::from(map);

        let from = Coordinate { x: 3, y: 4 };
        let to = Coordinate { x: 4, y: 0 };

        let full_path = region.derive_full_path(&from, &to).unwrap();

        assert_eq!(full_path.len(), 2);
        assert_eq!(full_path.contains(&Coordinate { x: 3, y: 4 }), true);
        assert_eq!(full_path.contains(&Coordinate { x: 4, y: 0 }), true);
    }

    #[test]
    fn test_derive_full_path_invalid_coordinate() {
        let map = "
            ..
            ..
        ";

        let region = Region::from(map);

        let from = Coordinate { x: 0, y: 0 };
        let to = Coordinate { x: 1, y: 1 };

        let full_path = region.derive_full_path(&from, &to);

        assert_eq!(full_path.is_err(), true);
        assert_eq!(full_path.unwrap_err(), InvalidCoordinateError);
    }

    #[test]
    fn test_can_see() {
        let map = "
            .#..#
            .....
            #####
            ....#
            ...##
        ";

        let region = Region::from(map);

        let from = Coordinate { x: 1, y: 0 };
        let to = Coordinate { x: 3, y: 4 };

        assert_eq!(region.can_see(&from, &to).unwrap(), false);

        let to2 = Coordinate { x: 2, y: 2 };

        assert_eq!(region.can_see(&from, &to2).unwrap(), true);
    }

    #[test]
    fn test_can_see_upwards() {
        let map = "
            .#..#
            .....
            #####
            ....#
            ...##
        ";

        let region = Region::from(map);

        let from = Coordinate { x: 3, y: 4 };
        let to = Coordinate { x: 1, y: 0 };

        assert_eq!(region.can_see(&from, &to).unwrap(), false);

        let to2 = Coordinate { x: 2, y: 2 };

        assert_eq!(region.can_see(&from, &to2).unwrap(), true);
    }

    #[test]
    fn test_visible_from() {
        let map = "
            .#..#
            .....
            #####
            ....#
            ...##
        ";

        let region = Region::from(map);

        let from = Coordinate { x: 3, y: 4 };

        let visible = region.visible_from(&from).unwrap();

        assert_eq!(visible.len(), 8);

        assert_eq!(visible.contains(&Coordinate { x: 4, y: 0 }), true);
        assert_eq!(visible.contains(&Coordinate { x: 0, y: 2 }), true);
        assert_eq!(visible.contains(&Coordinate { x: 1, y: 2 }), true);
        assert_eq!(visible.contains(&Coordinate { x: 2, y: 2 }), true);
        assert_eq!(visible.contains(&Coordinate { x: 3, y: 2 }), true);
        assert_eq!(visible.contains(&Coordinate { x: 4, y: 2 }), true);
        assert_eq!(visible.contains(&Coordinate { x: 4, y: 3 }), true);
        assert_eq!(visible.contains(&Coordinate { x: 4, y: 4 }), true);

        assert_eq!(
            region
                .visible_from(&Coordinate { x: 1, y: 0 })
                .unwrap()
                .len(),
            7
        );

        assert_eq!(
            region
                .visible_from(&Coordinate { x: 4, y: 0 })
                .unwrap()
                .len(),
            7
        );
        assert_eq!(
            region
                .visible_from(&Coordinate { x: 0, y: 2 })
                .unwrap()
                .len(),
            6
        );
        assert_eq!(
            region
                .visible_from(&Coordinate { x: 1, y: 2 })
                .unwrap()
                .len(),
            7
        );
        assert_eq!(
            region
                .visible_from(&Coordinate { x: 2, y: 2 })
                .unwrap()
                .len(),
            7
        );
        assert_eq!(
            region
                .visible_from(&Coordinate { x: 3, y: 2 })
                .unwrap()
                .len(),
            7
        );
        assert_eq!(
            region
                .visible_from(&Coordinate { x: 4, y: 2 })
                .unwrap()
                .len(),
            5
        );
        assert_eq!(
            region
                .visible_from(&Coordinate { x: 4, y: 3 })
                .unwrap()
                .len(),
            7
        );
        assert_eq!(
            region
                .visible_from(&Coordinate { x: 3, y: 4 })
                .unwrap()
                .len(),
            8
        );
        assert_eq!(
            region
                .visible_from(&Coordinate { x: 4, y: 4 })
                .unwrap()
                .len(),
            7
        );
    }

    #[test]
    fn test_max_visible_from_count() {
        let map = "
            .#..#
            .....
            #####
            ....#
            ...##
        ";

        let region = Region::from(map);

        assert_eq!(
            region.max_visible_from_count(),
            (Some(&Coordinate { x: 3, y: 4 }), 8)
        );
    }

    #[test]
    fn test_max_visible_from_count_bigger_1() {
        let map = "
        ......#.#.
        #..#.#....
        ..#######.
        .#.#.###..
        .#..#.....
        ..#....#.#
        #..#....#.
        .##.#..###
        ##...#..#.
        .#....####
        ";

        let region = Region::from(map);

        assert_eq!(
            region.max_visible_from_count(),
            (Some(&Coordinate { x: 5, y: 8 }), 33)
        );
    }

    #[test]
    fn test_max_visible_from_count_bigger_2() {
        let map = "
        #.#...#.#.
        .###....#.
        .#....#...
        ##.#.#.#.#
        ....#.#.#.
        .##..###.#
        ..#...##..
        ..##....##
        ......#...
        .####.###.
        ";

        let region = Region::from(map);

        assert_eq!(
            region.max_visible_from_count(),
            (Some(&Coordinate { x: 1, y: 2 }), 35)
        );
    }

    #[test]
    fn test_max_visible_from_count_bigger_3() {
        let map = "
        .#..#..###
        ####.###.#
        ....###.#.
        ..###.##.#
        ##.##.#.#.
        ....###..#
        ..#.#..#.#
        #..#.#.###
        .##...##.#
        .....#.#..
        ";

        let region = Region::from(map);

        assert_eq!(
            region.max_visible_from_count(),
            (Some(&Coordinate { x: 6, y: 3 }), 41)
        );
    }

    #[test]
    fn test_max_visible_from_count_biggest() {
        let map = "
        .#..##.###...#######
        ##.############..##.
        .#.######.########.#
        .###.#######.####.#.
        #####.##.#.##.###.##
        ..#####..#.#########
        ####################
        #.####....###.#.#.##
        ##.#################
        #####.##.###..####..
        ..######..##.#######
        ####.##.####...##..#
        .#####..#.######.###
        ##...#.##########...
        #.##########.#######
        .####.#.###.###.#.##
        ....##.##.###..#####
        .#.#.###########.###
        #.#.#.#####.####.###
        ###.##.####.##.#..##
        ";

        let region = Region::from(map);

        assert_eq!(
            region.max_visible_from_count(),
            (Some(&Coordinate { x: 11, y: 13 }), 210)
        );
    }

    #[test]
    fn test_obliteration_order_biggest_200th() {
        let map = "
        .#..##.###...#######
        ##.############..##.
        .#.######.########.#
        .###.#######.####.#.
        #####.##.#.##.###.##
        ..#####..#.#########
        ####################
        #.####....###.#.#.##
        ##.#################
        #####.##.###..####..
        ..######..##.#######
        ####.##.####...##..#
        .#####..#.######.###
        ##...#.##########...
        #.##########.#######
        .####.#.###.###.#.##
        ....##.##.###..#####
        .#.#.###########.###
        #.#.#.#####.####.###
        ###.##.####.##.#..##
        ";

        let region = Region::from(map);
        let (max_coord, _) = region.max_visible_from_count();

        let mut ob = Obliteration::new(&region, &max_coord.unwrap()).unwrap();
        assert_eq!(ob.nth(199), Some(Coordinate { x: 8, y: 2 }));
    }

    #[test]
    #[ignore]
    fn test_obliteration_order_biggest_count() {
        let map = "
        .#..##.###...#######
        ##.############..##.
        .#.######.########.#
        .###.#######.####.#.
        #####.##.#.##.###.##
        ..#####..#.#########
        ####################
        #.####....###.#.#.##
        ##.#################
        #####.##.###..####..
        ..######..##.#######
        ####.##.####...##..#
        .#####..#.######.###
        ##...#.##########...
        #.##########.#######
        .####.#.###.###.#.##
        ....##.##.###..#####
        .#.#.###########.###
        #.#.#.#####.####.###
        ###.##.####.##.#..##
        ";

        let region = Region::from(map);
        let (max_coord, _) = region.max_visible_from_count();

        let ob = Obliteration::new(&region, &max_coord.unwrap()).unwrap();

        // all asteroids obliterated except the origin
        assert_eq!(ob.count(), region.len() - 1);
    }
}
