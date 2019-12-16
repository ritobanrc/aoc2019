#[aoc_generator(day9, part1)]
pub fn parse_program(input: &str) -> Vec<i64> {
    input
        .split(',')
        .map(|i| i.parse::<i64>().unwrap())
        .collect()
}

#[aoc(day9, part1)]
fn solve_p1(tape: &Vec<i64>) -> i64 {
    let mut tape = tape.clone();
    intcode_computer(&mut tape, &mut 0, &mut 0, || 1)
}

#[aoc_generator(day9, part2)]
fn p2_generator(input: &str) -> Vec<i64> {
    parse_program(input)
}

#[aoc(day9, part2)]
fn solve_p2(tape: &Vec<i64>) -> i64 {
    let mut tape = tape.clone();
    intcode_computer(&mut tape, &mut 0, &mut 0, || 2)
}

pub fn intcode_computer<F>(
    tape: &mut Vec<i64>,
    i: &mut usize,
    relative_base: &mut i64,
    mut get_input: F,
) -> i64
where
    F: FnMut() -> i64,
{
    use crate::day4::get_digits;
    use std::convert::TryInto;

    //let mut tape = tape.clone();
    let output = loop {
        let instr: u64 = tape[*i]
            .try_into()
            .expect("Negative Instruction is Invalid");
        let mut digits = get_digits(instr);

        while digits.len() < 5 {
            digits.insert(0, 0);
        }

        let opcode = [digits[digits.len() - 2], digits[digits.len() - 1]];

        let read_param = |idx| {
            let parameter_mode = digits.get(digits.len() - 2 - idx).unwrap();
            match parameter_mode {
                0 => *tape.get(*tape.get(*i + idx).unwrap_or(&0) as usize).unwrap_or(&0),
                1 => *tape.get(*i + idx).unwrap_or(&0),
                2 => *tape.get((*relative_base + *tape.get(*i + idx).unwrap_or(&0)) as usize).unwrap_or(&0),
                e => panic!("Unrecognized parameter_mode: {:?}", e),
            }
        };

        let write_param = |tape: &mut Vec<i64>, idx: usize, data| {
            let parameter_mode = digits.get(digits.len() - 2 - idx).unwrap();
            match parameter_mode {
                0 => {
                    let output = *tape.get(*i + idx).unwrap_or(&0) as usize;
                    if output >= tape.len() {
                        tape.resize(output + 1, 0);
                    }
                    tape[output] = data;
                }
                1 => panic!("Cannot write to immediate mode param"),
                2 => {
                    let output = (*relative_base + *tape.get(*i + idx).unwrap_or(&0)) as usize;
                    if output >= tape.len() {
                        tape.resize(output + 1, 0);
                    }
                    tape[output] = data;
                }
                e => panic!("Unrecognized parameter_mode: {:?}", e),
            }
        };

        match opcode {
            [0, 1] => {
                let p1 = read_param(1);
                let p2 = read_param(2);

                write_param(tape, 3, p1 + p2);
                *i += 4;
            }
            [0, 2] => {
                let p1 = read_param(1);
                let p2 = read_param(2);

                write_param(tape, 3, p1 * p2);
                *i += 4;
            }
            [0, 3] => {
                write_param(tape, 1, get_input());
                *i += 2;
            }
            [0, 4] => {
                let out = read_param(1);
                *i += 2;
                break out;
            }
            [0, 5] => {
                let p1 = read_param(1);
                let p2 = read_param(2);

                if p1 != 0 {
                    *i = p2 as usize;
                } else {
                    *i += 3;
                }
            }
            [0, 6] => {
                let p1 = read_param(1);
                let p2 = read_param(2);

                if p1 == 0 {
                    *i = p2 as usize;
                } else {
                    *i += 3;
                }
            }
            [0, 7] => {
                let p1 = read_param(1);
                let p2 = read_param(2);

                if p1 < p2 {
                    write_param(tape, 3, 1);
                } else {
                    write_param(tape, 3, 0)
                }
                *i += 4;
            }
            [0, 8] => {
                let p1 = read_param(1);
                let p2 = read_param(2);

                if p1 == p2 {
                    write_param(tape, 3, 1);
                } else {
                    write_param(tape, 3, 0);
                }
                *i += 4;
            }
            [0, 9] => {
                let p1 = read_param(1);

                *relative_base += p1;

                *i += 2
            }
            [9, 9] => break -1,
            _ => panic!("Unrecognized opcode: {:?}", opcode),
        };
    };

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn copy_input() {
        let mut tape = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];
        //tape.append(&mut vec![0; 10_000]);
        let mut i = 0;
        let mut relative_base = 0;
        loop {
            let output = intcode_computer(&mut tape, &mut i, &mut relative_base, || 0);
            if output == -1 {
                return;
            } else {
                println!("{:?}", output);
            }
        }
    }

    #[test]
    fn large_output() {
        let mut tape = vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0];
        //tape.append(&mut vec![0; 10_000]);
        let mut i = 0;
        let mut relative_base = 0;
        let output = intcode_computer(&mut tape, &mut i, &mut relative_base, || 0);
        println!("{:?}", output);
    }

    #[test]
    fn large_output_2() {
        let mut tape = vec![104, 1125899906842624, 99];
        //tape.append(&mut vec![0; 10_000]);
        let mut i = 0;
        let mut relative_base = 0;
        let output = intcode_computer(&mut tape, &mut i, &mut relative_base, || 0);
        println!("{:?}", output);
        assert_eq!(output, tape[1]);
    }
}
