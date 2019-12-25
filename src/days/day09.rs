use crate::intcode::Program;
use crate::{first_answer, input, second_answer};

pub fn run() {
    let source_code = input(9).get(0).expect("Invalid input").clone();

    let mut test_boost_program: Program = source_code.parse().expect("Invalid BOOST program");
    test_boost_program.set_input(move |_| Ok(1));

    let mut sensor_boost_program: Program = source_code.parse().expect("Invalid BOOST program");
    sensor_boost_program.set_input(move |_| Ok(2));

    first_answer(
        "BOOST keycode",
        &test_boost_program
            .execute()
            .expect("Error while running BOOST program in test mode")
            .get(0)
            .unwrap(),
    );

    second_answer(
        "Coordinates of the distress signal",
        &sensor_boost_program
            .execute()
            .expect("Error while running BOOST program in sensor mode")
            .get(0)
            .unwrap(),
    )
}
