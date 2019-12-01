extern crate lib;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day: u8 = *&args[1].parse::<u8>().expect("Invalid day number");

    match day {
        1 => lib::day1::run(),
        _ => panic!("Nothing for this day")
    };
}
