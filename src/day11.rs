use crate::day9::{intcode_computer, parse_program};
use std::collections::HashMap;

#[aoc_generator(day11, part1)]
fn p1_generator(input: &str) -> Vec<i64> {
    parse_program(input)
}

#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn rotate_right(&self) -> Self {
        match *self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    pub fn rotate_left(&self) -> Self {
        match *self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        }
    }

    pub fn dx(&self) -> i64 {
        match *self {
            Direction::Up => 0,
            Direction::Left => -1,
            Direction::Down => 0,
            Direction::Right => 1,
        }
    }

    pub fn dy(&self) -> i64 {
        match *self {
            Direction::Up => -1,
            Direction::Left => 0,
            Direction::Down => 1,
            Direction::Right => 0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Color {
    Black,
    White,
}

impl Into<i64> for Color {
    fn into(self) -> i64 {
        match self {
            Color::Black => 0,
            Color::White => 1,
        }
    }
}

impl From<i64> for Color {
    fn from(num: i64) -> Self {
        match num {
            0 => Color::Black,
            1 => Color::White,
            e => panic!("Unrecognized color {:?}", e),
        }
    }
}

#[aoc(day11, part1)]
fn solve_p1(tape: &[i64]) -> usize {
    let mut robot_x = 0i64;
    let mut robot_y = 0i64;
    let mut robot_dir = Direction::Up;

    let mut panels: HashMap<(i64, i64), Color> = HashMap::new();

    let mut tape = tape.to_owned();

    let mut i = 0;
    let mut relative_base = 0;
    loop {
        let paint = intcode_computer(&mut tape, &mut i, &mut relative_base, || {
            (*panels.get(&(robot_x, robot_y)).unwrap_or(&Color::Black)).into()
        });

        if paint == -1 {
            break;
        }

        panels.insert((robot_x, robot_y), paint.into());

        let dir = intcode_computer(&mut tape, &mut i, &mut relative_base, || 0);

        match dir {
            0 => robot_dir = robot_dir.rotate_left(),
            1 => robot_dir = robot_dir.rotate_right(),
            e => panic!("Unknown direction to turn: {:?}", e),
        }

        robot_x += robot_dir.dx();
        robot_y += robot_dir.dy();
    }

    panels.len()
}

#[aoc_generator(day11, part2)]
fn p2_generator(input: &str) -> Vec<i64> {
    parse_program(input)
}

#[aoc(day11, part2)]
fn solve_p2(tape: &[i64]) -> usize {
    let mut robot_x = 0i64;
    let mut robot_y = 0i64;
    let mut robot_dir = Direction::Up;

    let mut panels: HashMap<(i64, i64), Color> = HashMap::new();

    panels.insert((robot_x, robot_y), Color::White);

    let mut tape = tape.to_owned();

    let mut i = 0;
    let mut relative_base = 0;

    loop {
        let paint = intcode_computer(&mut tape, &mut i, &mut relative_base, || {
            (*panels.get(&(robot_x, robot_y)).unwrap_or(&Color::Black)).into()
        });

        if paint == -1 {
            break;
        }

        panels.insert((robot_x, robot_y), paint.into());

        let dir = intcode_computer(&mut tape, &mut i, &mut relative_base, || 1);

        match dir {
            0 => robot_dir = robot_dir.rotate_left(),
            1 => robot_dir = robot_dir.rotate_right(),
            e => panic!("Unknown direction to turn: {:?}", e),
        }

        robot_x += robot_dir.dx();
        robot_y += robot_dir.dy();
    }

    for y in 0..10 {
        for x in 0..50 {
            let color = *panels.get(&(x, y)).unwrap_or(&Color::Black);
            print!(
                "{}",
                match color {
                    Color::White => "■",
                    Color::Black => " ",
                }
            )
        }
        println!();
    }
    0
}
