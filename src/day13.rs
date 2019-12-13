use crate::day9::{parse_program, intcode_computer};
use std::collections::HashMap;

#[aoc_generator(day13)]
fn day13_gen(input: &str) -> Vec<i64> {
    parse_program(input)
}

#[aoc(day13, part1)]
fn solve_p1(tape: &Vec<i64>) -> usize {
    const TAPE_SIZE: usize = 10_000;
    let mut tape = tape.clone();
    tape.append(&mut vec![0; TAPE_SIZE]);

    let mut screen = HashMap::new();

    let mut i = 0;
    let mut relative_base = 0;
    loop {
        let x = intcode_computer(&mut tape, &mut i, &mut relative_base, || { 0 });

        if x == -1 {
            break
        }

        let y = intcode_computer(&mut tape, &mut i, &mut relative_base, || { 0 });
        let tile_type = intcode_computer(&mut tape, &mut i, &mut relative_base, || { 0 });

        screen.entry(tile_type).or_insert(Vec::new()).push((x, y));
    }

    screen.get(&2).unwrap().len()
}



#[aoc(day13, part2)]
fn solve_p2(tape: &Vec<i64>) -> i64 {
    const TAPE_SIZE: usize = 10_000;
    let mut tape = tape.clone();
    tape.append(&mut vec![0; TAPE_SIZE]);

    tape[0] = 2;

    let mut screen = HashMap::new();

    fn get_move(screen: &HashMap<(i64, i64), i64>) -> i64 {
        let mut paddle_pos = 0;
        let mut ball_pos = 0;
        // We only need to print out the screen when it asks for input, because that's when it's finished drawing
        // Clear the terminal
        //print!("{}[2J", 27 as char);
        for y in 0..20 {
            for x in 0..50 {
                let tile_type = *screen.get(&(x, y)).unwrap_or(&0);
                let c = match tile_type {
                    0 => " ",
                    1 => "█",
                    2 => "▣",
                    3 => { paddle_pos = x; "⊔" },
                    4 => { ball_pos = x; "●" },
                    _ => panic!("Unrecognized tiletype"),
                };
                //print!("{}", c);
            }
            //println!("");
        }
        (ball_pos - paddle_pos).signum()
    }

    let mut i = 0;
    let mut relative_base = 0;

    let mut answer = 0;

    loop {
        let x = intcode_computer(&mut tape, &mut i, &mut relative_base, || get_move(&screen));
        let y = intcode_computer(&mut tape, &mut i, &mut relative_base, || get_move(&screen));
        let tile_type = intcode_computer(&mut tape, &mut i, &mut relative_base, || get_move(&screen));

        if x == -1 && y == 0 {
            answer = tile_type;
        }

        if tile_type == -1 {
            break
        }

        screen.insert((x, y), tile_type);


    }

    answer
}

