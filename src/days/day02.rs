use itertools::Itertools;

use crate::intcode::Program;
use crate::{first_answer, input, second_answer};

pub fn run() {
    let source_code_raw = input(2).get(0).expect("Input file is empty").clone();
    let mut program: Program = source_code_raw.parse().unwrap();

    program.patch(1, 12);
    program.patch(2, 2);

    match program.execute() {
        Ok(_) => first_answer("Program output", &program.get(0).unwrap()),
        Err(e) => println!("{:?}", e),
    }

    const MOON_LANDING: i64 = 1969_07_20;

    (0..99).tuple_combinations().for_each(|(noun, verb)| {
        let mut program: Program = source_code_raw.parse().unwrap();

        program.patch(1, noun);
        program.patch(2, verb);

        match program.execute() {
            Ok(_) if program.get(0).unwrap() == MOON_LANDING => second_answer(
                format!("Found noun = {} and verb = {}, so", noun, verb).as_str(),
                &(100 * noun + verb),
            ),
            Ok(_) => (),
            Err(e) => println!("{:?}", e),
        }
    });
}
