fn day_1_p2() -> i64 {
    use std::fs;

    fn calc_fuel(mass: f64) -> f64 {
        let mut additional_fuel = f64::floor(mass / 3.0) - 2.0;
        let mut total_fuel = additional_fuel;
        while additional_fuel > 0.0 {
            additional_fuel = f64::floor(additional_fuel / 3.0) - 2.0;
            total_fuel += f64::max(additional_fuel, 0.0);
        }
        total_fuel
    }


    fs::read_to_string("input/1")
        .unwrap()
        .lines()
        .map(|n| n.parse::<f64>().unwrap())
        .map(calc_fuel)
        .sum::<f64>() as i64
}

fn day_1_p1() -> i64 {
    use std::fs;
    fs::read_to_string("input/1")
        .unwrap()
        .lines()
        .map(|n| n.parse::<f64>().unwrap())
        .map(|n| f64::floor(n / 3.0) - 2.0)
        .sum::<f64>() as i64
}


fn main() {
    println!("{:?}", day_1_p2());
}
