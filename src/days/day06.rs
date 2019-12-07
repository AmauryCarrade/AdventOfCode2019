use crate::{first_answer, input, second_answer};
use std::collections::binary_heap::BinaryHeap;
use std::collections::HashMap;

/// Our orbits map (graph). Stored as a list of edges connected from one point to another.
#[derive(Debug)]
struct OrbitalMap {
    edges: HashMap<String, Vec<String>>,
}

impl OrbitalMap {
    pub fn new(orbits: Vec<String>) -> Self {
        let mut orbits_map = OrbitalMap {
            edges: HashMap::new(),
        };

        orbits
            .iter()
            .map(|orbit| orbit.split(')').collect::<Vec<&str>>())
            .filter(|orbit| orbit.len() == 2)
            .for_each(|orbit| {
                let from = orbit[0];
                let to = orbit[1];

                orbits_map
                    .edges
                    .entry(from.to_string())
                    .or_insert_with(|| vec![])
                    .push(to.to_string());
                orbits_map
                    .edges
                    .entry(to.to_string())
                    .or_insert_with(|| vec![])
                    .push(from.to_string());
            });

        orbits_map
    }

    /// Computes the distance from `from` to `to` using the
    /// Dijkstra algorithm.
    pub fn distance(&self, from: &str, to: &str) -> Option<usize> {
        let mut distances: HashMap<String, usize> = self
            .edges
            .keys()
            .map(|body| (body.clone(), std::usize::MAX))
            .collect();

        distances.insert(from.to_string(), 0);

        let mut heap = BinaryHeap::new();
        heap.push((from.to_string(), 0));

        while let Some((body, dist)) = heap.pop() {
            if body == to.to_string() {
                return Some(dist);
            }

            if dist > distances.get(&body).unwrap().clone() {
                continue;
            }

            for other_body in self.edges.get(&body).unwrap() {
                if dist + 1 < distances.get(other_body).unwrap().clone() {
                    distances.insert(body.clone(), dist + 1);
                    heap.push((other_body.clone(), dist + 1));
                }
            }
        }

        None
    }

    pub fn checksum(&self) -> usize {
        self.edges
            .keys()
            .filter_map(|body| self.distance(body.as_str(), "COM"))
            .sum()
    }
}

pub fn run() {
    let orbital_map = OrbitalMap::new(input(6));

    first_answer("Orbital map checksum", &orbital_map.checksum());

    // We compute the distance from YOU to SAN with Dijkstra, but we
    // want the number of orbits **transfers**, so we have to remove two
    // hops for the first and last orbits.
    second_answer(
        "How many orbital transfers from us (YOU) to Santa (SAN)",
        &(orbital_map.distance("YOU", "SAN").unwrap() - 2),
    );
}
