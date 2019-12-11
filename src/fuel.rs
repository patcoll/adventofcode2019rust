#[allow(dead_code)]
pub fn total_fuel_needed_for_mass(mass: i32) -> i32 {
    let mut total: i32 = 0;
    let mut calculated: i32;
    let mut mass_to_calculate: i32 = mass;

    loop {
        calculated = fuel_needed_for_mass(mass_to_calculate);

        if calculated > 0 {
            total += calculated;
        }

        if calculated <= 0 {
            return total;
        } else {
            mass_to_calculate = calculated;
        }
    }
}

#[allow(dead_code)]
pub fn fuel_needed_for_mass(mass: i32) -> i32 {
    mass / 3 - 2
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_total_fuel_needed_for_mass() {
        assert_eq!(total_fuel_needed_for_mass(12), 2);
        assert_eq!(total_fuel_needed_for_mass(14), 2);
        assert_eq!(total_fuel_needed_for_mass(1969), 966);
        assert_eq!(total_fuel_needed_for_mass(100756), 50346);
    }

    #[test]
    fn test_fuel_needed_for_mass() {
        assert_eq!(fuel_needed_for_mass(12), 2);
        assert_eq!(fuel_needed_for_mass(14), 2);
        assert_eq!(fuel_needed_for_mass(1969), 654);
        assert_eq!(fuel_needed_for_mass(100756), 33583);
    }
}
