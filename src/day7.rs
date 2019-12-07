use crate::day5::parse_program;
use itertools::Itertools;

#[aoc_generator(day7, part1)]
fn p1_generator(input: &str) -> Vec<i32> {
    parse_program(input)
}

#[aoc(day7, part1)]
fn solve_p1(tape: &Vec<i32>) -> i32 {
    let sequenecs = (0..=4).permutations(5);

    let mut max_signal = 0;
    for seq in sequenecs {
        let mut output = 0;
        for phase in seq {
            let mut phase_passed = false;
            output = intcode_computer(tape, || { 
                if phase_passed {
                    output
                } else {
                    phase_passed = true;
                    phase
                }
            });
        }
        if output > max_signal {
            max_signal = output;
        }
    }

    max_signal
}


fn intcode_computer<F>(tape: &Vec<i32>, mut get_input: F) -> i32 
    where F: FnMut() -> i32 {
    use std::convert::TryInto;
    use crate::day4::get_digits;
    let mut tape = tape.clone();
    let mut i = 0;
    loop {
        let instr: u64 = tape[i].try_into().expect("Negative Instruction is Invalid");
        let digits = get_digits(instr);

        let opcode = if digits.len() == 2 {
            [digits[0], digits[1]]
        } else if digits.len() == 1 {
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
                    1 => tape[i + 1],
                    _ => unreachable!(),
                };
                let p2 = match p2 {
                    0 => tape[tape[i + 2] as usize],
                    1 => tape[i + 2],
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
                    1 => tape[i + 1],
                    _ => unreachable!(),
                };
                let p2 = match p2 {
                    0 => tape[tape[i + 2] as usize],
                    1 => tape[i + 2],
                    _ => unreachable!(),
                };

                //println!("p1: {} p2: {}", p1, p2);

                let output = tape[i + 3] as usize;
                tape[output] = p1 * p2;
                i += 4;
            }
            [0, 3] => {
                let output = tape[i + 3] as usize;
                tape[output] = get_input();
                i += 2;
            }
            [0, 4] => {
                return tape[tape[i + 1] as usize]
            }
            [0, 5] => {
                let p1 = digits.get(digits.len() - 3).unwrap_or(&0);
                let p2 = digits.get(digits.len() - 4).unwrap_or(&0);

                let p1 = match p1 {
                    0 => tape[tape[i + 1] as usize],
                    1 => tape[i + 1],
                    _ => unreachable!(),
                };
                let p2 = match p2 {
                    0 => tape[tape[i + 2] as usize],
                    1 => tape[i + 2],
                    _ => unreachable!(),
                };

                if p1 != 0 {
                    i = p2 as usize;
                } else {
                    i += 3;
                }
            }
            [0, 6] => {
                let p1 = digits.get(digits.len() - 3).unwrap_or(&0);
                let p2 = digits.get(digits.len() - 4).unwrap_or(&0);

                let p1 = match p1 {
                    0 => tape[tape[i + 1] as usize],
                    1 => tape[i + 1],
                    _ => unreachable!(),
                };
                let p2 = match p2 {
                    0 => tape[tape[i + 2] as usize],
                    1 => tape[i + 2],
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
                    1 => tape[i + 1],
                    _ => unreachable!(),
                };
                let p2 = match p2 {
                    0 => tape[tape[i + 2] as usize],
                    1 => tape[i + 2],
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
                    1 => tape[i + 1],
                    _ => unreachable!(),
                };
                let p2 = match p2 {
                    0 => tape[tape[i + 2] as usize],
                    1 => tape[i + 2],
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
            [9, 9] => break 0,
            _ => panic!("Unrecognized opcode: {:?}", opcode),
        }
    }
}




#[aoc_generator(day7, part2)]
fn p2_generator(input: &str) -> Vec<i32> {
    parse_program(input)
}

#[aoc(day7, part2)]
fn solve_p2(tape: &Vec<i32>) -> i32 {
    let sequences = (5..=9).permutations(5);

    let mut max_signal = 0;
    for seq in sequences {
        let mut output = 0;
        let mut tapes = vec![tape.clone(); seq.len()];
        let mut ips = vec![0; seq.len()];
        let mut phases_passed = vec![false; seq.len()];
        'feedback: loop {
            for (i, phase) in seq.iter().enumerate() {
                let state = intcode_computer_p2(&mut tapes[i], &mut ips[i], || {
                    if !phases_passed[i] {
                        phases_passed[i] = true;
                        //println!("Phase passed for {:?}: {:?}", i, phase);
                        *phase
                    } else {
                        output
                    }
                });


                //println!("{:?}", state);

                if state == -1 {
                    break 'feedback
                }

                output = state;
            }
        }
        if output > max_signal {
            max_signal = output;
        }

        println!("Seq: {:?} Gives {:?}", seq, output);
    }

    max_signal
}


fn intcode_computer_p2<F>(tape: &mut Vec<i32>, i: &mut usize, mut get_input: F) -> i32
    where F: FnMut() -> i32 {
    use std::convert::TryInto;
    use crate::day4::get_digits;



    //let mut tape = tape.clone();
    let output = loop {
        let instr: u64 = tape[*i].try_into().expect("Negative Instruction is Invalid");
        let mut digits = get_digits(instr);

        while digits.len() < 4 {
            digits.insert(0, 0);
        }

        let opcode = if digits.len() == 2 {
            [digits[0], digits[1]]
        } else if digits.len() == 1 {
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
                    0 => tape[tape[*i + 1] as usize],
                    1 => tape[*i + 1],
                    _ => unreachable!(),
                };
                let p2 = match p2 {
                    0 => tape[tape[*i + 2] as usize],
                    1 => tape[*i + 2],
                    _ => unreachable!(),
                };

                let output = tape[*i + 3] as usize;
                tape[output] = p1 + p2;
                *i += 4;
            }
            [0, 2] => {
                let p1 = digits.get(digits.len() - 3).unwrap_or(&0);
                let p2 = digits.get(digits.len() - 4).unwrap_or(&0);

                //println!("p1: {} p2: {}", p1, p2);

                let p1 = match p1 {
                    0 => tape[tape[*i + 1] as usize],
                    1 => tape[*i + 1],
                    _ => unreachable!(),
                };
                let p2 = match p2 {
                    0 => tape[tape[*i + 2] as usize],
                    1 => tape[*i + 2],
                    _ => unreachable!(),
                };

                //println!("p1: {} p2: {}", p1, p2);

                let output = tape[*i + 3] as usize;
                tape[output] = p1 * p2;
                *i += 4;
            }
            [0, 3] => {
                let output = tape[*i + 1] as usize;
                tape[output] = get_input();
                *i += 2;
            }
            [0, 4] => {
                *i += 2;
                break tape[tape[*i - 1] as usize]
            }
            [0, 5] => {
                let p1 = digits.get(digits.len() - 3).unwrap_or(&0);
                let p2 = digits.get(digits.len() - 4).unwrap_or(&0);

                let p1 = match p1 {
                    0 => tape[tape[*i + 1] as usize],
                    1 => tape[*i + 1],
                    _ => unreachable!(),
                };
                let p2 = match p2 {
                    0 => tape[tape[*i + 2] as usize],
                    1 => tape[*i + 2],
                    _ => unreachable!(),
                };

                if p1 != 0 {
                    *i = p2 as usize;
                } else {
                    *i += 3;
                }
            }
            [0, 6] => {
                let p1 = digits.get(digits.len() - 3).unwrap_or(&0);
                let p2 = digits.get(digits.len() - 4).unwrap_or(&0);

                let p1 = match p1 {
                    0 => tape[tape[*i + 1] as usize],
                    1 => tape[*i + 1],
                    _ => unreachable!(),
                };
                let p2 = match p2 {
                    0 => tape[tape[*i + 2] as usize],
                    1 => tape[*i + 2],
                    _ => unreachable!(),
                };

                if p1 == 0 {
                    *i = p2 as usize;
                } else {
                    *i += 3;
                }
            }
            [0, 7] => {
                let p1 = digits.get(digits.len() - 3).unwrap_or(&0);
                let p2 = digits.get(digits.len() - 4).unwrap_or(&0);

                let p1 = match p1 {
                    0 => tape[tape[*i + 1] as usize],
                    1 => tape[*i + 1],
                    _ => unreachable!(),
                };
                let p2 = match p2 {
                    0 => tape[tape[*i + 2] as usize],
                    1 => tape[*i + 2],
                    _ => unreachable!(),
                };

                let output = tape[*i + 3] as usize;

                if p1 < p2 {
                    tape[output] = 1;
                } else {
                    tape[output] = 0;
                }
                *i += 4;
            }
            [0, 8] => {
                let p1 = digits.get(digits.len() - 3).unwrap_or(&0);
                let p2 = digits.get(digits.len() - 4).unwrap_or(&0);

                let p1 = match p1 {
                    0 => tape[tape[*i + 1] as usize],
                    1 => tape[*i + 1],
                    _ => unreachable!(),
                };
                let p2 = match p2 {
                    0 => tape[tape[*i + 2] as usize],
                    1 => tape[*i + 2],
                    _ => unreachable!(),
                };

                let output = tape[*i + 3] as usize;

                if p1 == p2 {
                    tape[output] = 1;
                } else {
                    tape[output] = 0;
                }
                *i += 4;
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
    fn day7_sample1() {
        let tape = vec![3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5];

        let output = solve_p2(&tape);
        assert_eq!(output, 139629729);
    }


    #[test]
    fn day7_sample2() {
        let tape = vec![3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54, -5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4, 53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10];
        //let seq = vec![9, 7, 8, 5, 6];

        let output = solve_p2(&tape);
        assert_eq!(output, 18216);

    }
}
