pub fn total_fuel_needed_for_mass(mass: i64) -> i64 {
    let mut total: i64 = 0;
    let mut calculated: i64;
    let mut mass_to_calculate: i64 = mass;

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

pub fn fuel_needed_for_mass(mass: i64) -> i64 {
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
        assert_eq!(total_fuel_needed_for_mass(100_756), 50346);
    }

    #[test]
    fn test_fuel_needed_for_mass() {
        assert_eq!(fuel_needed_for_mass(12), 2);
        assert_eq!(fuel_needed_for_mass(14), 2);
        assert_eq!(fuel_needed_for_mass(1969), 654);
        assert_eq!(fuel_needed_for_mass(100_756), 33583);
    }
}
