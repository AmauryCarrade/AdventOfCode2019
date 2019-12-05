use crate::{first_answer, input, second_answer};

pub fn run() {
    let input: Vec<i32> = input(1)
        .iter()
        .map(|mass_str| mass_str.parse::<i32>().expect("Invalid number in input!"))
        .collect();

    let fuel_mass: i32 = input.iter().map(|m| *m).map(compute_fuel_naive).sum();
    let real_fuel_mass: i32 = input
        .iter()
        .map(|mass| {
            std::iter::successors(Some(compute_fuel_naive(*mass)), |mass| {
                Some(compute_fuel_naive(*mass))
            })
            .take_while(|mass| *mass > 0)
            .sum::<i32>()
        })
        .sum();

    first_answer("Naive fuel mass", &fuel_mass);
    second_answer("Real fuel mass", &real_fuel_mass);
}

fn compute_fuel_naive(mass: i32) -> i32 {
    (mass as i32 / 3) - 2
}
