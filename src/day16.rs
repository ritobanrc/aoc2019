#[aoc_generator(day16)]
fn get_signal(input: &str) -> Vec<i32> {
    input
        .chars()
        .map(|x| x.to_digit(10).unwrap() as i32)
        .collect()
}

struct FFTPattern {
    position: usize,
    scale: usize,
}

impl FFTPattern {
    fn new(scale: usize) -> FFTPattern {
        FFTPattern {
            position: 0,
            scale: scale,
        }
    }
}

impl Iterator for FFTPattern {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let arr = [0, 1, 0, -1];
        self.position += 1;
        Some(arr[(self.position / self.scale) % 4])
    }
}

fn fft(digits: &[i32], phases: usize) -> Vec<i32> {
    let mut output = digits.to_owned();

    for _ in 0..phases {
        output = output
            .iter()
            .enumerate()
            .map(|(i, _)| {
                let pattern = FFTPattern::new(i + 1);
                output
                    .iter()
                    .zip(pattern)
                    .map(|(d, p)| d * p)
                    .sum::<i32>()
                    .abs()
                    % 10
            })
            .collect();
    }

    output
}

#[aoc(day16, part1)]
fn solve_p1(digits: &[i32]) -> i32 {
    use std::convert::TryInto;
    let output = fft(digits, 100);

    let mut ans = 0;
    for (i, x) in output[..8].iter().rev().enumerate() {
        ans += 10_i32.pow(i.try_into().unwrap()) * x
    }
    ans
}

#[aoc(day16, part2)]
fn solve_p2(digits: &[i32]) -> i32 {
    use std::convert::TryInto;

    let output = &digits
        .iter()
        .cycle()
        .take(digits.len() * 10_000)
        .map(|x| *x)
        .collect::<Vec<_>>();

    let orig_len = output.len();

    // We only care about the second half
    let mut output = output[output.len() / 2..].to_owned();

    for _ in 0..100 {
        let mut next_ouput = Vec::new();
        for x in output.iter().rev() {
            if next_ouput.is_empty() {
                next_ouput.push(*x)
            } else {
                next_ouput.push((*x + next_ouput[next_ouput.len() - 1]).abs() % 10);
            }
        }
        output = next_ouput.iter().rev().map(|x| *x).collect();
    }

    let mut offset = 0;
    for (i, x) in digits[..7].iter().rev().enumerate() {
        offset += 10usize.pow(i.try_into().unwrap()) * *x as usize
    }

    offset -= orig_len / 2;

    let mut ans = 0;
    for (i, x) in output[offset..offset + 8].iter().rev().enumerate() {
        ans += 10i32.pow(i.try_into().unwrap()) * x
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day16_test1() {
        assert_eq!(
            fft(&get_signal("12345678"), 4),
            vec![0, 1, 0, 2, 9, 4, 9, 8]
        );
    }

    #[test]
    fn day16_test2() {
        let output = get_signal("03036732577212944063491565474664");
        assert_eq!(solve_p2(&output), 84462026);
    }
}
