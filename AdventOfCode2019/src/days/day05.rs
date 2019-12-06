use crate::intcode::Program;
use crate::{answer, input};

pub fn run() {
    let source_code_raw = input(5).get(0).expect("Input file is empty").clone();

    vec![1, 5]
        .into_iter()
        .enumerate()
        .for_each(|(answer_num, input)| {
            let mut program: Program = source_code_raw.parse().unwrap();

            program.set_input(move |_| Ok(input.clone()));

            match program.execute() {
                Ok(output) => answer(
                    answer_num + 1,
                    format!("Diagnostic code for system ID {}", input).as_str(),
                    &output
                        .iter()
                        .filter(|i| i != &&0)
                        .map(|i| i.to_string())
                        .collect::<String>(),
                ),
                Err(e) => println!("{:?}", e),
            };
        });
}
