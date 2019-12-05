
#[aoc(day4, part1)]
fn count_passwords_p1(input: &str) -> usize {
    let range: Vec<_> =  input.split('-').collect();
    let lower: u64 = range[0].parse().unwrap();
    let upper: u64 = range[1].parse().unwrap();
    let mut count = 0;
    for i in lower..upper {
        let digits = get_digits(i);
        let mut adjacent = false;
        let mut increasing = true;
        for (i, d) in digits.iter().enumerate() {
            if i != 0 && *d < digits[i - 1] {
                increasing = false;
            }
            else if i != 0 && *d == digits[i - 1] {
                adjacent = true;
            }
        }

        if adjacent && increasing {
            count += 1;
        }
    }
    count
}

#[aoc(day4, part2)]
fn count_passwords_p2(input: &str) -> usize {
    let range: Vec<_> =  input.split('-').collect();
    let lower: u64 = range[0].parse().unwrap();
    let upper: u64 = range[1].parse().unwrap();
    let mut count = 0;
    for i in lower..upper {
        let digits = get_digits(i);
        let mut adjacent = false;
        let mut increasing = true;
        for (i, d) in digits.iter().enumerate() {
            if i != 0 && *d < digits[i - 1] {
                increasing = false;
            }
            else if i > 0 && *d == digits[i - 1] {
                if i > 1 && *d == digits[i-2] {
                    continue
                }
                if i < digits.len() - 1 && *d == digits[i + 1] {
                    continue
                }
                adjacent = true;
            }
        }

        if adjacent && increasing {
            count += 1;
        }
    }
    count
}


pub fn get_digits(n: u64) -> Vec<u64> {
    fn next_digit(n: u64, digits: &mut Vec<u64>) {
        if n >= 10 {
            next_digit(n / 10, digits);
        }
        digits.push(n % 10);
    }

    let mut digits = Vec::new();
    next_digit(n, &mut digits);
    digits
}
