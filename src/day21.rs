use crate::day9::{intcode_computer, parse_program};

#[aoc_generator(day21)]
fn day21_gen(input: &str) -> Vec<i64> {
    parse_program(input)
}

#[aoc(day21, part1)]
fn solve_p1(tape: &[i64]) -> i64 {
    // if A is a hole, jump
    // if A and B are holes, jump,
    // if A B and C are holes, jump
    // if D is a hole, DO NOT JUMP

    let program = &"
NOT A J
NOT B T
OR T J
NOT C T
OR T J
AND D J
WALK
"[1..];

    test_program(tape, program).unwrap()
}

#[aoc(day21, part2)]
fn solve_p2(tape: &[i64]) -> i64 {
    // If !A, Jump ---- no matter what, otherwise, you will die.
    // If !B, Jump if D and !E
    // If !C, Jump if D and !F
    // !A || (!B && !E && D) || (!C && D && !F)
    // !A || !(B || E) && D || !(C || F) && D
    // !A || !(B || E) && D
    //       !((B || E) || !Q)

    let program = &"
NOT A J
NOT B T
AND A T
AND C T
OR T J
NOT C T
AND H T
OR T J
AND D J
RUN
"[1..];

    test_program(tape, program).unwrap()
}

fn test_program(tape: &[i64], program: &str) -> Option<i64> {
    let mut program_feed = program.bytes();

    let mut tape = tape.to_owned();
    let mut i = 0;
    let mut rb = 0;

    loop {
        let result = intcode_computer(&mut tape, &mut i, &mut rb, || {
            program_feed.next().unwrap() as i64
        });

        if result == -1 {
            return None;
        }

        if result > 127 {
            return Some(result);
        }
        print!("{}", result as u8 as char);
    }
}
