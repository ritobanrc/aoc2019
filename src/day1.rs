#[aoc(day1, part1)]
pub fn day_1_p1(input: &str) -> u64 {
    input
        .lines()
        .map(|n| n.parse::<u64>().unwrap())
        .map(|n| n / 3 - 2)
        .sum()
}


#[aoc(day1, part2)]
pub fn day_1_p2(input: &str) -> u64 {
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

    input
        .lines()
        .map(|n| n.parse::<u64>().unwrap())
        .map(calc_fuel)
        .sum::<u64>()
}
