#[aoc_generator(day2, part1)]
fn parse_program(input: &str) -> Vec<usize> {
    input.split(',').map(|i| i.parse::<usize>().unwrap()).collect()
}

fn run_tape(mut tape: Vec<usize>) -> Vec<usize> {
    let mut i = 0;
    loop {
        match tape[i] {
            1 => {
                let out = tape[i+3];
                tape[out] = tape[tape[i+2]] + tape[tape[i+1]]
            },
            2 => {
                let out = tape[i+3];
                tape[out] = tape[tape[i+2]] * tape[tape[i+1]]
            },
            99 => {
                break
            },
            _ => {}
        }
        i += 4;
    }
    tape
}

#[aoc(day2, part1)]
fn solve_p1(tape: &Vec<usize>) -> usize {
    let mut tape = tape.clone();
    tape[1] = 12;
    tape[2] = 2;
    run_tape(tape)[0]
}

#[aoc_generator(day2, part2)]
fn p2_generator(input: &str) -> Vec<usize> {
    parse_program(input)
}

#[aoc(day2, part2)]
fn solve_p2(tape: &Vec<usize>) -> usize {
    for noun in 1..99 {
        for verb in 1..99 {
            let mut tape = tape.clone();
            tape[1] = noun;
            tape[2] = verb;
            if run_tape(tape)[0] == 19690720 {
                return noun * 100 + verb
            }
        }
    }
    println!("Failed to solve Day 2 Part 2");
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        assert_eq!(run_tape(vec![1,0,0,0,99]), vec![2, 0, 0, 0, 99]);
    }

    #[test]
    fn sample2() {
        assert_eq!(run_tape(vec![2,3,0,3,99]), vec![2,3,0,6,99]);
    }

    #[test]
    fn sample3() {
        assert_eq!(run_tape(vec![2,4,4,5,99,0]), vec![2,4,4,5,99,9801]);
    }

    #[test]
    fn sample4() {
        assert_eq!(run_tape(vec![1,1,1,4,99,5,6,0,99]), vec![30,1,1,4,2,5,6,0,99]);
    }
}
