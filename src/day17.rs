use crate::day11::Direction;
use crate::day9::{intcode_computer, parse_program};
use std::collections::{HashMap, HashSet};
use std::char;

const DIRECTIONS: [Direction; 4] = [Direction::Up, Direction::Down, Direction::Left, Direction::Right];

#[aoc_generator(day17)]
fn day17_gen(input: &str) -> Vec<i64> {
    parse_program(input)
}

#[derive(PartialEq, Eq)]
enum TileType {
    Empty,
    Scaffold,
}

#[aoc(day17, part1)]
fn solve_p1(tape: &[i64]) -> usize {
    let mut tape = tape.to_owned();
    let mut i = 0;
    let mut rb = 0;

    let mut map = HashMap::new();
    let mut current = (0usize, 0usize);
    let mut total_alignment = 0;

    loop {
        let output = intcode_computer(&mut tape, &mut i, &mut rb, || { 0 });
        if output == -1 {
            break;
        }
        let output = output as u8 as char;
        match output{
            '.' => {
                map.insert(current, TileType::Empty);
                current.0  += 1;
            },
            '#' | '^' | 'v' | '<' | '>'  => {
                map.insert(current, TileType::Scaffold);
                current.0  += 1;
            },
            '\n' => {
                current.0 = 0;
                current.1 += 1;
            },
            e => panic!("Unknown char: {:?}", e)
        };
        print!("{}", output);
    }

    let width = map.keys().max_by_key(|(x, _)| x).unwrap().0 + 1;
    let height = map.keys().max_by_key(|(_, y)| y).unwrap().1 + 1;

    //println!("{:?} by {:?}", width, height);

    for x in 0..width {
        for y in 0..height {
            if x > 0 && x < width - 1 && y > 0 && y < height - 1 &&
                map[&(x, y)] == TileType::Scaffold &&
                map[&(x + 1, y)] == TileType::Scaffold &&
                map[&(x - 1, y)] == TileType::Scaffold &&
                map[&(x, y + 1)] == TileType::Scaffold &&
                map[&(x, y - 1)] == TileType::Scaffold {
                    total_alignment += x * y
            }
        }
    }

    total_alignment
}



#[aoc(day17, part2)]
fn solve_p2(tape: &[i64]) -> i64 {
    let mut tape = tape.to_owned();
    tape[0] = 2;
    let mut i = 0;
    let mut rb = 0;

    let mut map = HashMap::new();
    let mut robot_loc = (0, 0);
    let mut robot_dir = Direction::Up;

    { // get map
        let mut current = (0usize, 0usize);

        loop {
            let output = intcode_computer(&mut tape, &mut i, &mut rb, || { 0 });
            if output == -1 {
                break;
            }
            let output = output as u8 as char;
            match output{
                '.' => {
                    map.insert(current, TileType::Empty);
                    current.0  += 1;
                },
                '#' => {
                    map.insert(current, TileType::Scaffold);
                    current.0  += 1;
                },
                '^' | 'v' | '<' | '>'  => {
                    map.insert(current, TileType::Scaffold);
                    robot_loc = current;
                    robot_dir = match output {
                        '^' => Direction::Up,
                        'v' => Direction::Down,
                        '>' => Direction::Right,
                        '<' => Direction::Left,
                        _ => unreachable!(),
                    };
                    current.0  += 1;
                },
                '\n' => {
                    current.0 = 0;
                    current.1 += 1;
                },
                'M' => { // This means that it's asking for "Main"
                    break
                }
                e => eprintln!("Unknown char: {:?}", e)
            };
        }
    }



    let width = map.keys().max_by_key(|(x, _)| x).unwrap().0 + 1;
    let height = map.keys().max_by_key(|(_, y)| y).unwrap().1 + 1;

    let intersections = {
        let mut intersections = HashSet::new();
        for x in 0..width {
            for y in 0..height {
                if x > 0 && x < width - 1 && y > 0 && y < height - 1 &&
                    map[&(x, y)] == TileType::Scaffold &&
                    map[&(x + 1, y)] == TileType::Scaffold &&
                    map[&(x - 1, y)] == TileType::Scaffold &&
                    map[&(x, y + 1)] == TileType::Scaffold &&
                    map[&(x, y - 1)] == TileType::Scaffold {
                        intersections.insert((x, y));
                }
            }
        }
        intersections
    };

    let mut path = Vec::new();
    path.push(robot_loc);

    { // get path
        loop {
            let mut neighbors = Vec::new();
            for dir in &DIRECTIONS {
                let new_pos = ((robot_loc.0 as i64 + dir.dx()),
                               (robot_loc.1 as i64 + dir.dy()));
                if new_pos.0 >= 0 && new_pos.1 >= 0 {
                    let new_pos = (new_pos.0 as usize, new_pos.1 as usize);
                    if new_pos.0 < width && new_pos.1 < height {
                        if map[&new_pos] == TileType::Scaffold && (!path.contains(&new_pos) || intersections.contains(&new_pos)) {
                            neighbors.push(new_pos);
                        }
                    }
                }
            }

            if neighbors.len() == 0 {
                break
            }
            if neighbors.len() == 1 {
                path.push(neighbors[0]);
                robot_loc = neighbors[0];
            } else {
                // we have more than 1 neighbor
                // choose to continue in the direction we have already been going
                let prev = path[path.len() - 2];
                let dx = robot_loc.0 as i64 - prev.0 as i64;
                let dy = robot_loc.1 as i64 - prev.1 as i64;
                let new_pos = ((robot_loc.0 as i64 + dx) as usize,
                               (robot_loc.1 as i64 + dy) as usize);
                assert!(neighbors.contains(&new_pos));
                path.push(new_pos);
                robot_loc = new_pos;
            }
        }
    }

    #[derive(Debug)]
    enum Instruction {
        Left, Right, Forward(usize)
    }

    let mut instructions = Vec::new();

    let mut in_this_dir = 1;

    for (i, pos) in path[..path.len() - 1].iter().enumerate() {
        let dx = path[i + 1].0 as i64 - pos.0 as i64;
        let dy = path[i + 1].1 as i64 - pos.1 as i64;
        // if the direction changed...
        if dx != robot_dir.dx() || dy != robot_dir.dy() {
            // the first instruction won't be a forward
            if instructions.len() > 0 {
                instructions.push(Instruction::Forward(in_this_dir));
            }
            in_this_dir = 1;
            if dx == robot_dir.rotate_left().dx() && dy == robot_dir.rotate_left().dy() {
                instructions.push(Instruction::Left);
                robot_dir = robot_dir.rotate_left();
            } else {
                instructions.push(Instruction::Right);
                robot_dir = robot_dir.rotate_right();
            }
            continue
        }
        in_this_dir += 1;
    }

    println!("{:?}", instructions);

    /* 
    let main = vec![65, 44, 66, 44, 67, 44, 66, 44, 65, 44, 67, 10];
    let A = vec![82, 44, 56, 44, 82, 44, 56, 10];
    let B = vec![82, 44, 52, 44, 82, 44, 52, 44, 82, 44, 56, 10];
    let C = vec![76, 44, 54, 44, 76, 44, 50, 10];

    use itertools::concat;

    let vec = concat(vec![main, A, B, C, vec!['y' as u8 as i64, 10]]);
    let mut iter = vec.iter();

    loop {
        let result = intcode_computer(&mut tape, &mut i, &mut rb, || { *iter.next().unwrap() }) as u8 as char;

        print!("{}", result);

        if result == 'ÿ' {
            break
        }
    }
    */

    0
}
