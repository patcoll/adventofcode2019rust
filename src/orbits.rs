use petgraph::algo::astar;
use petgraph::algo::bellman_ford;
use petgraph::prelude::*;
use rayon::prelude::*;
use std::collections::HashSet;

#[derive(Debug)]
pub struct Universe {
    pub objects: HashSet<Object>,
    pub orbits: HashSet<Orbit>,
}

impl From<&str> for Universe {
    fn from(lines: &str) -> Self {
        let (objects, orbits) = Self::parse_orbits(lines);
        Universe { objects, orbits }
    }
}

impl Universe {
    fn parse_orbits(lines: &str) -> ParsedOrbits {
        let mut objects = HashSet::new();
        let mut orbits = HashSet::new();

        lines
            .trim()
            .split_whitespace()
            .collect::<Vec<_>>()
            .into_iter()
            .map(|orbit_str| Orbit::new(orbit_str))
            .for_each(|(in_objects, in_orbit)| {
                objects.extend(in_objects);
                orbits.insert(in_orbit);
            });

        (objects, orbits)
    }

    pub fn add(&mut self, lines: &str) -> &Self {
        let (objects, orbits) = Self::parse_orbits(lines);
        self.objects.extend(objects);
        self.orbits.extend(orbits);
        self
    }

    pub fn count_objects(&self) -> usize {
        self.objects.len()
    }

    pub fn count_direct_orbits(&self) -> usize {
        self.orbits.len()
    }

    pub fn count_indirect_orbits(&self) -> usize {
        let mut graph = GraphMap::<&str, f64, Directed>::with_capacity(
            self.count_objects(),
            self.count_direct_orbits(),
        );

        for orbit in &self.orbits {
            let orbited_id = orbit.orbited.0.as_str();
            let orbiting_id = orbit.orbiting.0.as_str();
            graph.add_edge(orbited_id, orbiting_id, Default::default());
        }

        self.objects
            .iter()
            .map(|object| object.0.as_str())
            .collect::<Vec<_>>()
            .into_par_iter()
            .map(|object_name| {
                let results = bellman_ford(&graph, object_name);

                match &results {
                    Ok((_, b)) => b.iter().cloned().filter(|x| x.is_some()).count(),
                    Err(_) => 0,
                }
            })
            .sum()
    }

    pub fn get_hop_count(&self, from: &str, to: &str) -> Option<usize> {
        let mut graph = GraphMap::<&str, f64, Undirected>::with_capacity(
            self.count_objects(),
            self.count_direct_orbits(),
        );

        for orbit in &self.orbits {
            let orbited_id = orbit.orbited.0.as_str();
            let orbiting_id = orbit.orbiting.0.as_str();
            graph.add_edge(orbited_id, orbiting_id, Default::default());
        }

        let optional_values = astar(&graph, from, |finish| finish == to, |_| 0, |_| 0);
        match optional_values {
            Some((_, path)) => Some(path.len() - 1),
            _ => None,
        }
    }

    pub fn get_minimal_orbital_transfer_count(
        &self,
        from: &str,
        to: &str,
    ) -> Option<usize> {
        if let Some(c) = self.get_hop_count(from, to) {
            if c > 2 {
                Some(c - 2)
            } else {
                None
            }
        } else {
            None
        }
    }
}

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct Object(pub String);

impl From<&str> for Object {
    fn from(s: &str) -> Self {
        Object(s.to_string())
    }
}

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct Orbit {
    pub orbited: Object,  // The planet that is being orbited.
    pub orbiting: Object, // The planet that is orbiting.
}

type ParsedOrbits = (HashSet<Object>, HashSet<Orbit>);
type ParsedOrbit = (HashSet<Object>, Orbit);

impl Orbit {
    fn new(s: &str) -> ParsedOrbit {
        let split = s.split(')');
        let objects = split
            .map(|name| Object(name.to_string()))
            .collect::<Vec<_>>();
        // println!("objects: {:?}", objects);
        let orbit = Orbit {
            orbited: objects[0].clone(),
            orbiting: objects[1].clone(),
        };
        (objects.into_iter().collect::<HashSet<_>>(), orbit)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_orbit_new() {
        let mut objects = HashSet::new();
        objects.insert(Object::from("A"));
        objects.insert(Object::from("B"));

        // let orbits = HashSet::new();
        let orbit = Orbit {
            orbited: Object::from("A"),
            orbiting: Object::from("B"),
        };

        assert_eq!(Orbit::new("A)B"), (objects, orbit));
    }

    #[test]
    fn test_universe_from() {
        let universe = Universe::from("COM)A  A)B  B)C");
        assert_eq!(universe.count_objects(), 4);
        assert_eq!(universe.count_direct_orbits(), 3);
        assert_eq!(universe.count_indirect_orbits(), 6);
    }

    #[test]
    fn test_count_indirect_objects() {
        let universe = Universe::from("COM)B B)C C)D D)E E)F B)G G)H D)I E)J J)K K)L");
        assert_eq!(universe.count_objects(), 12);
        assert_eq!(universe.count_direct_orbits(), 11);
        assert_eq!(universe.count_indirect_orbits(), 42);
    }

    #[test]
    fn test_get_path() {
        let universe = Universe::from("COM)A  A)B  B)C");
        assert_eq!(universe.get_hop_count("COM", "B"), Some(2));
        assert_eq!(
            universe.get_minimal_orbital_transfer_count("COM", "A"),
            None
        );
        assert_eq!(
            universe.get_minimal_orbital_transfer_count("COM", "B"),
            None
        );
        assert_eq!(
            universe.get_minimal_orbital_transfer_count("COM", "C"),
            Some(1)
        );

        let universe2 =
            Universe::from("COM)B B)C C)D D)E E)F B)G G)H D)I E)J J)K K)L K)YOU I)SAN");
        assert_eq!(universe2.get_hop_count("COM", "B"), Some(1));
        assert_eq!(universe2.get_hop_count("COM", "H"), Some(3));
        assert_eq!(universe2.get_hop_count("YOU", "K"), Some(1));
        assert_eq!(universe2.get_hop_count("YOU", "SAN"), Some(6));
        assert_eq!(
            universe2.get_minimal_orbital_transfer_count("YOU", "SAN"),
            Some(4)
        );
    }
}
