use crate::input;

pub fn run() {
    let input: Vec<u32> = input(1, false).iter()
        .map(|mass_str| mass_str.parse::<u32>().expect("Invalid number in input!"))
        .collect();

    let fuel_mass: u32 = input.iter().map(|m| *m).map(compute_fuel_naive).sum();
    let real_fuel_mass: u32 = input.iter().map(|m| *m).map(compute_fuel_total).sum();

    println!("[1] Naive fuel mass: {mass}", mass = fuel_mass);
    println!("[2] Real fuel mass: {mass}", mass = real_fuel_mass);
}

fn compute_fuel_naive(mass: u32) -> u32 {
    let fuel = ((mass as f32) / 3.0).floor() as i32 - 2;
    return if fuel < 0 { 0 } else { fuel as u32 }
}

fn compute_fuel_total(mass: u32) -> u32 {
    let mut total_fuel = 0;
    let mut current_fuel = compute_fuel_naive(mass);

    loop {
        total_fuel += current_fuel;
        current_fuel = compute_fuel_naive(current_fuel);

        if current_fuel == 0 {
            break;
        }
    }

    total_fuel
}
