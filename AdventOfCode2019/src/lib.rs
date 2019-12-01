use std::fs::File;
use std::io::{BufRead, BufReader};

pub mod day1;

///
/// Loads the input from the sources directory. Files have to be in
/// /input/day-12-2.txt for day 12 problem 2 (and the same for others).
///
fn input(day: u8, bonus: bool) -> Vec<String> {
    let filename = format!("input/day-{day}-{bonus}.txt", day = day, bonus = match bonus {
        true => 2,
        false => 1
    });

    let file = File::open(&filename)
        .expect(format!("Unable to open input file in {filename}", filename = filename).as_str());

    BufReader::new(file)
        .lines()
        .map(|l| l.expect(format!("Unable to read line in {filename}", filename = filename).as_str()))
        .filter(|l| !l.is_empty())
        .collect()
}
