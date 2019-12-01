fn day_1_p2() -> u64 {
    use std::fs;

    fn calc_fuel(mass: u64) -> u64 {
        let mut additional_fuel = mass / 3 - 2;
        let mut total_fuel = additional_fuel;
        while additional_fuel > 0 {
            if let Some(x) = (additional_fuel/ 3).checked_sub(2) {
                additional_fuel = x;
                total_fuel += x;
            } else {
                break
            }
        }
        total_fuel
    }


    fs::read_to_string("input/1")
        .unwrap()
        .lines()
        .map(|n| n.parse::<u64>().unwrap())
        .map(calc_fuel)
        .sum::<u64>()
}

fn day_1_p1() -> u64 {
    use std::fs;
    fs::read_to_string("input/1")
        .unwrap()
        .lines()
        .map(|n| n.parse::<u64>().unwrap())
        .map(|n| n / 3 - 2)
        .sum()
}

fn main() {
    println!("{:?}", day_1_p2());
}
