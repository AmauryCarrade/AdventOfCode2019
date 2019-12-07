#![feature(result_map_or_else)]

extern crate itertools;

use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub mod intcode;

pub mod days;

///
/// Loads the input from the sources directory. Files have to be in
/// /input/day-12-2.txt for day 12 problem 2 (and the same for others).
///
fn input(day: u8) -> Vec<String> {
    let filename = format!("input/day-{day}.txt", day = day);

    let file = File::open(&filename).expect(
        format!(
            "Unable to open input file in {filename}",
            filename = filename
        )
        .as_str(),
    );

    BufReader::new(file)
        .lines()
        .map(|l| {
            l.expect(format!("Unable to read line in {filename}", filename = filename).as_str())
        })
        .filter(|l| !l.is_empty())
        .collect()
}

fn answer(num: usize, label: &str, val: &dyn Display) {
    println!("{} - {}: {}", num, label, val)
}

fn first_answer(label: &str, val: &dyn Display) {
    answer(1, label, val)
}

fn second_answer(label: &str, val: &dyn Display) {
    answer(2, label, val)
}
