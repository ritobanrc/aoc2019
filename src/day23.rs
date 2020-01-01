use crate::day9::{parse_program};
use std::collections::{VecDeque};

#[aoc_generator(day23)]
fn day23_gen(input: &str) -> Vec<i64> {
    parse_program(input)
}


#[aoc(day23, part1)]
fn solve_p1(tape: &[i64]) -> i64 {
    let mut vms = Vec::new();

    for address in 0..50 {
        let mut vm = IntcodeVM::new(tape);
        vm.input(address);
        vms.push(vm);
    }

    let mut current = 0;
    loop {
        let vm = &mut vms[current];
        let output = vm.next();
        match output {
            IntcodeOutput::Done => break -1,
            IntcodeOutput::Out(reciever) => {
                let x = match vm.next()  {
                    IntcodeOutput::Out(o) => o,
                    _ => panic!("Reciever {}, but didn't get anything else", reciever),
                };
                let y = match vm.next()  {
                    IntcodeOutput::Out(o) => o,
                    _ => panic!("Reciever {}, but didn't get anything else", reciever),
                };

                if reciever == 255 {
                    return y
                }

                vms[reciever as usize].input(x);
                vms[reciever as usize].input(y);
            }
            IntcodeOutput::Input => {
                // so the vm is out of input
                // we could provide a -1 here, but I don't think that's necessary
                // we'll just now spend some time running the other vms
                vm.input(-1);
                current = (current + 1) % vms.len();
            }
        }
    }
}


#[aoc(day23, part2)]
fn solve_p2(tape: &[i64]) -> i64 {
    let mut vms = Vec::new();

    for address in 0..50 {
        let mut vm = IntcodeVM::new(tape);
        vm.input(address);
        vms.push(vm);
    }

    let mut nat_x = 0;
    let mut nat_y = 0;

    let mut last_y_delivered = -1;

    let mut current = 0;
    let mut idle = vec![false; 50];

    loop {
        let vm = &mut vms[current];
        let output = vm.next();
        match output {
            IntcodeOutput::Done => break -1,
            IntcodeOutput::Out(reciever) => {
                let x = match vm.next()  {
                    IntcodeOutput::Out(o) => o,
                    _ => panic!("Reciever {}, but didn't get anything else", reciever),
                };
                let y = match vm.next()  {
                    IntcodeOutput::Out(o) => o,
                    _ => panic!("Reciever {}, but didn't get anything else", reciever),
                };

                if reciever == 255 {
                    nat_x = x;
                    nat_y = y;
                    continue
                }

                let reciever = reciever as usize;
                idle[reciever] = false;
                vms[reciever].input(x);
                vms[reciever].input(y);
            }
            IntcodeOutput::Input => {
                // so the vm is out of input
                // we could provide a -1 here, but I don't think that's necessary
                // we'll just now spend some time running the other vms
                vm.input(-1);
                idle[current] = true;
                current = (current + 1) % vms.len();
            }
        }

        if idle.iter().all(|&x| x) {
            if nat_y == last_y_delivered {
                return nat_y;
            }
            idle[0] = false;
            vms[0].input(nat_x);
            vms[0].input(nat_y);

            last_y_delivered = nat_y;
        }
    }
}


struct IntcodeVM {
    tape: Vec<i64>,
    i: usize,
    relative_base: i64,
    input_queue: VecDeque<i64>,
}

enum IntcodeOutput {
    Done,
    Out(i64),
    Input,
}

impl IntcodeVM {
    fn new(tape: &[i64]) -> IntcodeVM {
        IntcodeVM {
            tape: tape.to_owned(),
            i: 0, 
            relative_base: 0,
            input_queue: VecDeque::new()
        }
    }

    fn input(&mut self, input: i64) {
        self.input_queue.push_front(input);
    }

    fn next(&mut self) -> IntcodeOutput {
        use crate::day4::get_digits;
        use std::convert::TryInto;

        //let mut tape = tape.clone();
        loop {
            let instr: u64 = self.tape[self.i]
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
                    0 => self.tape
                        .get(*self.tape.get(self.i + idx).unwrap_or(&0) as usize)
                        .unwrap_or(&0),
                    1 => self.tape.get(self.i + idx).unwrap_or(&0),
                    2 => self.tape
                        .get((self.relative_base + *self.tape.get(self.i + idx).unwrap_or(&0)) as usize)
                        .unwrap_or(&0),
                    e => panic!("Unrecognized parameter_mode: {:?}", e),
                }
            };

            // we can't close over `self`, so instead, close over these copies
            let i = self.i;
            let rb = self.relative_base;

            let write_param = |tape: &mut Vec<i64>, idx: usize, data| {
                let parameter_mode = digits.get(digits.len() - 2 - idx).unwrap();
                match parameter_mode {
                    0 => {
                        let output = *tape.get(i + idx).unwrap_or(&0) as usize;
                        if output >= tape.len() {
                            tape.resize(output + 1, 0);
                        }
                        tape[output] = data;
                    }
                    1 => panic!("Cannot write to immediate mode param"),
                    2 => {
                        let output = (rb + *tape.get(i + idx).unwrap_or(&0)) as usize;
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
                    let p1 = *read_param(1);
                    let p2 = *read_param(2);

                    write_param(&mut self.tape, 3, p1 + p2);
                    self.i += 4;
                }
                [0, 2] => {
                    let p1 = *read_param(1);
                    let p2 = *read_param(2);

                    write_param(&mut self.tape, 3, p1 * p2);
                    self.i += 4;
                }
                [0, 3] => {
                    let input = if let Some(i) = self.input_queue.pop_back() {
                        i
                    } else {
                        break IntcodeOutput::Input
                    };
                    write_param(&mut self.tape, 1, input);
                    self.i += 2;
                }
                [0, 4] => {
                    let out = *read_param(1);
                    self.i += 2;
                    break IntcodeOutput::Out(out);
                }
                [0, 5] => {
                    let p1 = read_param(1);
                    let p2 = read_param(2);

                    if *p1 != 0 {
                        self.i = *p2 as usize;
                    } else {
                        self.i += 3;
                    }
                }
                [0, 6] => {
                    let p1 = read_param(1);
                    let p2 = read_param(2);

                    if *p1 == 0 {
                        self.i = *p2 as usize;
                    } else {
                        self.i += 3;
                    }
                }
                [0, 7] => {
                    let p1 = read_param(1);
                    let p2 = read_param(2);

                    if p1 < p2 {
                        write_param(&mut self.tape, 3, 1);
                    } else {
                        write_param(&mut self.tape, 3, 0)
                    }
                    self.i += 4;
                }
                [0, 8] => {
                    let p1 = read_param(1);
                    let p2 = read_param(2);

                    if p1 == p2 {
                        write_param(&mut self.tape, 3, 1);
                    } else {
                        write_param(&mut self.tape, 3, 0);
                    }
                    self.i += 4;
                }
                [0, 9] => {
                    let p1 = *read_param(1);

                    self.relative_base += p1;

                    self.i += 2
                }
                [9, 9] => break IntcodeOutput::Done,
                _ => panic!("Unrecognized opcode: {:?}", opcode),
            };
        }
    }
}
