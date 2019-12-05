use crate::day4::get_digits;
use std::convert::TryInto;

#[aoc_generator(day5, part1)]
fn parse_program(input: &str) -> Vec<i32> {
    input
        .split(',')
        .map(|i| i.parse::<i32>().unwrap())
        .collect()
}

#[aoc(day5, part1)]
fn solve_p1(tape: &Vec<i32>) -> i32 {
    let mut tape = tape.clone();
    let mut i = 0;
    loop {
        let instr: u64 = tape[i].try_into().expect("Negative Instruction is Invalid");
        let digits = get_digits(instr);

        let opcode = if digits.len() == 2 {
            [digits[0], digits[1]]
        } else if digits.len()  == 1 {
            [0, digits[0]]
        } else {
            [digits[digits.len() - 2], digits[digits.len() - 1]]
        };

        //println!("{:?}", opcode);

        match opcode {
            [0, 1] => {
                let p1 = digits.get(digits.len() - 3).unwrap_or(&0);
                let p2 = digits.get(digits.len() - 4).unwrap_or(&0);


                let p1 = match p1 {
                    0 => tape[tape[i + 1] as usize],
                    1 => tape[i+1],
                    _ => unreachable!(),
                };
                let p2 = match p2 {
                    0 => tape[tape[i + 2] as usize],
                    1 => tape[i+2],
                    _ => unreachable!(),
                };

                let output = tape[i + 3] as usize;
                tape[output] = p1 + p2;
                i += 4;
            }
            [0, 2] => {
                let p1 = digits.get(digits.len() - 3).unwrap_or(&0);
                let p2 = digits.get(digits.len() - 4).unwrap_or(&0);

                //println!("p1: {} p2: {}", p1, p2);


                let p1 = match p1 {
                    0 => tape[tape[i + 1] as usize],
                    1 => tape[i+1],
                    _ => unreachable!(),
                };
                let p2 = match p2 {
                    0 => tape[tape[i + 2] as usize],
                    1 => tape[i+2],
                    _ => unreachable!(),
                };

                //println!("p1: {} p2: {}", p1, p2);

                let output = tape[i + 3] as usize;
                tape[output] = p1 * p2;
                i += 4;
            }
            [0, 3] => {
                let output = tape[i + 3] as usize;
                tape[output] = 1; // Always submitting input 1
                i += 2;
            }
            [0, 4] => {
                if tape[i + 2] == 99 {
                    // This is the final output
                    break tape[tape[i + 1] as usize]
                } else {
                    println!("{:?}", tape[tape[i + 1] as usize]);
                    //assert_eq!(tape[tape[i + 1] as usize], 0)
                }
                i += 2;
            },
            [9, 9] => {
                break 0
            },
            _ => panic!("Unrecognized opcode: {:?}" , opcode)
        }
        //println!("{:?}", tape);
    }
}


#[aoc_generator(day5, part2)]
fn p2_gen(input: &str) -> Vec<i32> {
    parse_program(input)
}


#[aoc(day5, part2)]
fn solve_p2(tape: &Vec<i32>) -> i32 {
    let mut tape = tape.clone();
    let mut i = 0;
    loop {
        let instr: u64 = tape[i].try_into().expect("Negative Instruction is Invalid");
        let digits = get_digits(instr);

        let opcode = if digits.len() == 2 {
            [digits[0], digits[1]]
        } else if digits.len()  == 1 {
            [0, digits[0]]
        } else {
            [digits[digits.len() - 2], digits[digits.len() - 1]]
        };

        //println!("{:?}", opcode);

        match opcode {
            [0, 1] => {
                let p1 = digits.get(digits.len() - 3).unwrap_or(&0);
                let p2 = digits.get(digits.len() - 4).unwrap_or(&0);


                let p1 = match p1 {
                    0 => tape[tape[i + 1] as usize],
                    1 => tape[i+1],
                    _ => unreachable!(),
                };
                let p2 = match p2 {
                    0 => tape[tape[i + 2] as usize],
                    1 => tape[i+2],
                    _ => unreachable!(),
                };

                let output = tape[i + 3] as usize;
                tape[output] = p1 + p2;
                i += 4;
            }
            [0, 2] => {
                let p1 = digits.get(digits.len() - 3).unwrap_or(&0);
                let p2 = digits.get(digits.len() - 4).unwrap_or(&0);


                let p1 = match p1 {
                    0 => tape[tape[i + 1] as usize],
                    1 => tape[i+1],
                    _ => unreachable!(),
                };
                let p2 = match p2 {
                    0 => tape[tape[i + 2] as usize],
                    1 => tape[i+2],
                    _ => unreachable!(),
                };

                //println!("p1: {} p2: {}", p1, p2);

                let output = tape[i + 3] as usize;
                tape[output] = p1 * p2;
                i += 4;
            }
            [0, 3] => {
                let output = tape[i + 3] as usize;
                tape[output] = 5; // Always submitting input 1
                i += 2;
            }
            [0, 4] => {
                //if tape[i + 2] == 99 {
                    //// This is the final output
                    //break tape[tape[i + 1] as usize]
                //} else {
                break tape[tape[i + 1] as usize]
                    ////assert_eq!(tape[tape[i + 1] as usize], 0)
                //}
            },
            [0, 5] => {
                let p1 = digits.get(digits.len() - 3).unwrap_or(&0);
                let p2 = digits.get(digits.len() - 4).unwrap_or(&0);


                let p1 = match p1 {
                    0 => tape[tape[i + 1] as usize],
                    1 => tape[i+1],
                    _ => unreachable!(),
                };
                let p2 = match p2 {
                    0 => tape[tape[i + 2] as usize],
                    1 => tape[i+2],
                    _ => unreachable!(),
                };

                if p1 != 0 {
                    i = p2 as usize;
                } else {
                    i += 3;
                }
            },
            [0, 6] => {
                let p1 = digits.get(digits.len() - 3).unwrap_or(&0);
                let p2 = digits.get(digits.len() - 4).unwrap_or(&0);


                let p1 = match p1 {
                    0 => tape[tape[i + 1] as usize],
                    1 => tape[i+1],
                    _ => unreachable!(),
                };
                let p2 = match p2 {
                    0 => tape[tape[i + 2] as usize],
                    1 => tape[i+2],
                    _ => unreachable!(),
                };

                if p1 == 0 {
                    i = p2 as usize;
                } else {
                    i += 3;
                }
            }
            [0, 7] => {
                let p1 = digits.get(digits.len() - 3).unwrap_or(&0);
                let p2 = digits.get(digits.len() - 4).unwrap_or(&0);


                let p1 = match p1 {
                    0 => tape[tape[i + 1] as usize],
                    1 => tape[i+1],
                    _ => unreachable!(),
                };
                let p2 = match p2 {
                    0 => tape[tape[i + 2] as usize],
                    1 => tape[i+2],
                    _ => unreachable!(),
                };

                let output = tape[i + 3] as usize;

                if p1 < p2 {
                    tape[output] = 1;
                } else {
                    tape[output] = 0;
                }
                i += 4;
            }
            [0, 8] => {
                let p1 = digits.get(digits.len() - 3).unwrap_or(&0);
                let p2 = digits.get(digits.len() - 4).unwrap_or(&0);


                let p1 = match p1 {
                    0 => tape[tape[i + 1] as usize],
                    1 => tape[i+1],
                    _ => unreachable!(),
                };
                let p2 = match p2 {
                    0 => tape[tape[i + 2] as usize],
                    1 => tape[i+2],
                    _ => unreachable!(),
                };

                let output = tape[i + 3] as usize;

                if p1 == p2 {
                    tape[output] = 1;
                } else {
                    tape[output] = 0;
                }
                i += 4;
            }
            [9, 9] => {
                break 0
            },
            _ => panic!("Unrecognized opcode: {:?}" , opcode)
        }
        //println!("{:?}", tape);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day5_sample1() {
        println!("{:?}", solve_p1(&vec![1002,4,3,4,33]));
    }
}
