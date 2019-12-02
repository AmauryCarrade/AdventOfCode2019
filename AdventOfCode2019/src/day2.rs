use itertools::Itertools;

use crate::input;
use crate::intcode_program::Program;

pub fn run() {
    let source_code_raw = input(2, false).get(0).expect("Input file is empty").clone();
    let mut program = Program::parse(&source_code_raw).unwrap();

    program.patch(1, 12);
    program.patch(2, 2);

    match program.execute() {
        Ok(output) => println!("Position zero: {}", output),
        Err(e) => println!("{:?}", e),
    }

    const MOON_LANDING: usize = 1969_07_20;

    (0..99).tuple_combinations().for_each(|(noun, verb)| {
        let mut program = Program::parse(&source_code_raw).unwrap();

        program.patch(1, noun);
        program.patch(2, verb);

        match program.execute() {
            Ok(output) if output == MOON_LANDING => println!(
                "Found noun = {} and verb = {}; result = {}",
                noun,
                verb,
                100 * noun + verb
            ),
            Ok(_) => (),
            Err(e) => println!("{:?}", e),
        }
    });
}
