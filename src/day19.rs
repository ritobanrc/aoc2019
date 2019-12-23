use crate::day9::{intcode_computer, parse_program};

#[aoc_generator(day19)]
fn day19_gen(input: &str) -> Vec<i64> {
    parse_program(input)
}

#[aoc(day19, part1)]
fn solve_p1(tape: &[i64]) -> usize {
    let mut total = 0;
    for y in 0..50 {
        for x in 0..50 {
            match query(tape, x, y) {
                0 => continue,
                1 => total += 1,
                _ => unreachable!(),
            }
        }
    }

    total
}

fn query(tape: &[i64], x: i64, y: i64) -> i64 {
    let mut first = false;
    let mut tape = tape.to_owned();
    intcode_computer(&mut tape, &mut 0, &mut 0, || {
        if first {
            y
        } else {
            first = true;
            x
        }
    })
}

#[aoc(day19, part2)]
fn solve_p2(tape: &[i64]) -> i64 {
    // Find a point (x, y) such that
    //     (x + 100, y) is in the beam
    //     (x, y + 100) is in the beam
    let mut x = 0;
    let mut y = 0;

    let mut highest_y_found = None;

    loop {
        match query(tape, x, y) {
            0 => {
                if let Some(highest_y_found) = highest_y_found {
                    if y == highest_y_found {
                        // this means that we have found visible squares at this
                        // level, but it's now no longer visible
                        // so we can move to the next y
                        x = 0;
                        y += 1;
                        //println!();
                        continue;
                    }
                }
                if x > 100 * y {
                    // alternatively, if the X has gotten really far without
                    // encountering anything, this is probably an empty row
                    x = 0;
                    y += 1;
                    //println!();
                }
                x += 1;
                //print!(" ");
            }
            1 => {
                highest_y_found = Some(y);
                if query(tape, x + 99, y) == 1 && query(tape, x, y + 99) == 1 {
                    break x * 10_000 + y;
                }
                x += 1;
                //print!("#");
            }
            _ => unreachable!(),
        }
    }
}
