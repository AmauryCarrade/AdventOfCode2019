use crate::{first_answer, input, second_answer};

use std::collections::HashSet;
use std::ops;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Vector {
    x: i32,
    y: i32,
}

impl Vector {
    fn manhattan_distance(&self, other: &Vector) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

impl ops::Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Self::Output {
        Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::Mul<i32> for Vector {
    type Output = Vector;

    fn mul(self, rhs: i32) -> Self::Output {
        Vector {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl ops::Mul<Vector> for i32 {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        Vector {
            x: rhs.x * self,
            y: rhs.y * self,
        }
    }
}

const CENTER: Vector = Vector { x: 0, y: 0 };

#[derive(Debug)]
struct WirePart {
    direction: Vector,
    length: u32,
}

impl FromStr for WirePart {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(WirePart {
            direction: match s.chars().nth(0).unwrap() {
                'U' => Vector { x: 0, y: 1 },
                'D' => Vector { x: 0, y: -1 },
                'L' => Vector { x: -1, y: 0 },
                'R' => Vector { x: 1, y: 0 },
                _ => panic!("Invalid wire part: unknown direction"),
            },
            length: s[1..].parse::<u32>().expect("Invalid wire part: NaN"),
        })
    }
}

#[derive(Debug)]
struct Wire {
    parts: Vec<WirePart>,
    points: Vec<Vector>,
}

impl FromStr for Wire {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut wire = Wire {
            parts: s
                .split(',')
                .map(|instruction| instruction.parse().unwrap())
                .collect(),
            points: vec![],
        };

        wire.compute_points();
        Ok(wire)
    }
}

impl Wire {
    fn compute_points(&mut self) {
        self.points = self
            .parts
            .iter()
            .fold(
                (
                    CENTER,
                    Vec::with_capacity(self.parts.iter().map(|p| p.length).sum::<u32>() as usize),
                ),
                |(current_point, mut points), part| {
                    let new_points: Vec<Vector> =
                        (1i32..(part.length + 1) as i32) // +1 for the initial step
                            .map(|i| current_point + part.direction * i)
                            .collect();
                    points.extend(new_points.iter());
                    (*new_points.last().unwrap(), points)
                },
            )
            .1;
    }

    fn points_set(&self) -> HashSet<Vector> {
        self.points.iter().cloned().collect()
    }

    fn intersect_all(wires: &Vec<Wire>) -> HashSet<Vector> {
        wires
            .iter()
            .skip(1)
            .fold(wires.first().unwrap().points_set(), |acc, wire| {
                acc.intersection(&wire.points_set()).cloned().collect()
            })
    }

    fn signal_delay_to(&self, point: &Vector) -> u32 {
        self.points.iter().take_while(|p| p != &point).count() as u32 + 1 // +1 for the initial step
    }
}

pub fn run() {
    let wires = input(3, false)
        .iter()
        .map(|wire_data| wire_data.parse().expect("Invalid wire data"))
        .collect();
    let intersections = Wire::intersect_all(&wires);

    let min_manhattan = intersections
        .iter()
        .map(|p| p.manhattan_distance(&CENTER))
        .min()
        .unwrap();

    let min_signal: u32 = intersections
        .iter()
        .map(|intersection| {
            wires
                .iter()
                .map(|wire| wire.signal_delay_to(intersection))
                .sum()
        })
        .min()
        .unwrap();

    first_answer("Distance to the closest intersection", &min_manhattan);
    second_answer("Minimal combined steps (signal)", &min_signal);
}
