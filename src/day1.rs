use std::error;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn part_1() -> Result<i32, Box<dyn error::Error>> {
    let input = File::open("inputs/day1.txt")?;
    let reader = BufReader::new(input);

    let mut result: i32 = 0;

    for line in reader.lines() {
        let module_mass: i32 = line?.parse()?;
        result = result + calculate_fuel(module_mass);
    }

    Ok(result)
}

pub fn part_2() -> Result<i32, Box<dyn error::Error>> {
    let input = File::open("inputs/day1.txt")?;
    let reader = BufReader::new(input);

    let mut result: i32 = 0;

    for line in reader.lines() {
        let module_mass: i32 = line?.parse()?;
        result = result + calculate_fuel_with_residual(module_mass);
    }

    Ok(result)
}

fn calculate_fuel_with_residual(module_mass: i32) -> i32 {
    let mut result: i32 = 0;

    // Calculate initial fuel mass
    let mut fuel_mass = calculate_fuel(module_mass);
    result = result + fuel_mass;

    // Calculate each next mass
    loop {
        fuel_mass = calculate_fuel(fuel_mass);

        if fuel_mass <= 0 {
            break result;
        }

        result = result + fuel_mass;
    }
}

// Fuel required to launch a given module is based on its mass. Specifically, to find the fuel required for a module, take its mass, divide by three, round down, and subtract 2.
fn calculate_fuel(mass: i32) -> i32 {
    (f64::floor(f64::from(mass) / 3f64) as i32) - 2
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_calculate_fuel() {
        assert_eq!(calculate_fuel(12), 2);
        assert_eq!(calculate_fuel(14), 2);
        assert_eq!(calculate_fuel(1969), 654);
        assert_eq!(calculate_fuel(100756), 33583);
    }

    #[test]
    fn test_calculate_fuel_with_residual() {
        assert_eq!(calculate_fuel_with_residual(12), 2);
        assert_eq!(calculate_fuel_with_residual(1969), 966);
        assert_eq!(calculate_fuel_with_residual(100756), 50346);
    }
}
