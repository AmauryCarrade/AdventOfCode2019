extern crate lib;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day: u8 = args[1].parse::<u8>().expect("Invalid day number");

    match day {
        1 => lib::days::day01::run(),
        2 => lib::days::day02::run(),
        3 => lib::days::day03::run(),
        4 => lib::days::day04::run(),
        _ => eprintln!("Nothing for this day"),
    };
}
