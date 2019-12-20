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

        self
            .objects
            .clone()
            .into_par_iter()
            .map(|object| {
                let results = bellman_ford(&graph, object.0.as_str());

                match &results {
                    Ok((_, b)) => b.iter().cloned().filter(|x| x.is_some()).count(),
                    Err(_) => 0,
                }
            })
            .sum()
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
}
