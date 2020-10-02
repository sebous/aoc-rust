use crate::input;

pub fn day01() {
    let input_string = input::load_input("inputs/01");
    let input_formatted: Vec<u32> = input_string.lines().map(|s| s.parse().unwrap()).collect();

    let mass_per_module: Vec<u32> = input_formatted
        .iter()
        .map(|n| {
            let result = (((n / 3) as f32).floor() as u32) - 2;
            return result;
        })
        .collect();

    let total_fuel: u32 = mass_per_module.iter().sum();

    println!("Part 1:");
    println!("{:?}", total_fuel);

    pub fn calculate_fuel(mass: &u32) -> u32 {
        let fuel_needed_for_mass = (((mass / 3) as f32).floor() as i32) - 2;

        let mut fuel_needed: u32 = if fuel_needed_for_mass < 0 {
            0
        } else {
            fuel_needed_for_mass as u32
        };

        let fuel_for_fuel_mass = match fuel_needed {
            0 => 0,
            1 => 1,
            2 => 2,
            _ => calculate_fuel(&fuel_needed),
        };

        fuel_needed += fuel_for_fuel_mass;
        return fuel_needed;
    }

    let total_fuel_per_module: Vec<u32> = input_formatted
        .iter()
        .map(|mass| calculate_fuel(mass))
        .collect();
    let total_fuel_sum: u32 = total_fuel_per_module.iter().sum();

    println!("Part 2:");
    println!("{}", total_fuel_sum);
}
