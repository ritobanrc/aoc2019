use crate::day11::Direction;
use crate::day9::{intcode_computer, parse_program};
use std::collections::{HashMap, HashSet, VecDeque};

#[aoc_generator(day15)]
fn day15_gen(input: &str) -> Vec<i64> {
    parse_program(input)
}

impl From<i64> for Direction {
    fn from(num: i64) -> Self {
        match num {
            1 => Direction::Up,
            2 => Direction::Down,
            3 => Direction::Left,
            4 => Direction::Right,
            e => panic!("Unrecognized direction: {:?}", e),
        }
    }
}

impl Into<i64> for Direction {
    fn into(self) -> i64 {
        match self {
            Direction::Up => 1,
            Direction::Down => 2,
            Direction::Left => 3,
            Direction::Right => 4,
        }
    }
}

#[derive(PartialEq, Eq)]
enum TileType {
    Empty,
    Wall,
    Goal,
    Robot,
}

struct SeachResult(
    HashMap<(i64, i64), TileType>,
    HashMap<(i64, i64), (i64, i64)>,
    (i64, i64),
);

fn intcode_search(
    tape: &[i64],
) -> SeachResult {
    let mut tape = tape.to_owned();
    let mut i = 0;
    let mut rb = 0;

    let mut current = (0, 0);
    let mut goal = (0, 0);

    let mut map = HashMap::new();
    let mut parents = HashMap::new();

    map.insert(current, TileType::Robot);

    loop {
        let mut moved = false;

        for dir in 1..=4 {
            let new_pos = (
                current.0 + Direction::from(dir).dx(),
                current.1 + Direction::from(dir).dy(),
            );
            if map.contains_key(&new_pos) {
                continue;
            }
            let result = intcode_computer(&mut tape, &mut i, &mut rb, || dir);
            match result {
                0 => {
                    map.insert(new_pos, TileType::Wall);
                }
                1 => {
                    parents.insert(new_pos, current);
                    map.insert(current, TileType::Empty);
                    current = new_pos;
                    map.insert(current, TileType::Robot);
                    moved = true;
                    break;
                }
                2 => {
                    parents.insert(new_pos, current);
                    map.insert(current, TileType::Empty);
                    current = new_pos;
                    map.insert(current, TileType::Goal);
                    goal = current;
                }
                e => panic!("Unrecognized response: {:?}", e),
            }
        }

        // if we didn't move, go back to the parent
        if !moved {
            let back = parents.get(&current).unwrap();

            let dx = back.0 - current.0;
            let dy = back.1 - current.1;
            let dir: i64 = match (dx, dy) {
                (1, 0) => Direction::Right,
                (-1, 0) => Direction::Left,
                (0, 1) => Direction::Down,
                (0, -1) => Direction::Up,
                e => panic!("Impossible (dx, dy): {:?}", e),
            }
            .into();
            //println!("Moving back to {:?}, {:?}, {:?}, {:?}", back, dx, dy, dir);
            let result = intcode_computer(&mut tape, &mut i, &mut rb, || dir);

            // if the result isn't 1, something is wrong
            assert_eq!(result, 1);
            // don't overwrite the goal when backtracking
            if map[&current] != TileType::Goal {
                map.insert(current, TileType::Empty);
            }
            current = *back;
            if map[&current] != TileType::Goal {
                map.insert(current, TileType::Robot);
            }
        }

        if current == (0, 0) {
            // we've backtracked all the way back to the start, so we're done
            break;
        }
    }

    SeachResult(map, parents, goal)
}

#[aoc(day15, part1)]
fn solve_p1(tape: &[i64]) -> usize {
    let SeachResult(map, parents, goal) = intcode_search(tape);

    for y in -21..20 {
        for x in -30..25 {
            let tile = map.get(&(x, y)).unwrap_or(&TileType::Empty);
            if y == 0 && x == 0 {
                print!("0");
            } else {
                print!(
                    "{}",
                    match tile {
                        TileType::Empty => " ",
                        TileType::Wall => "█",
                        TileType::Robot => "●",
                        TileType::Goal => "▣",
                    }
                )
            }
        }
        println!();
    }

    let mut current = goal;
    let mut distance = 0;
    while current != (0, 0) {
        current = parents[&current];
        distance += 1;
    }

    distance
}

#[aoc(day15, part2)]
fn solve_p2(tape: &[i64]) -> usize {
    let SeachResult(map, _parents, oxygen) = intcode_search(tape);

    let mut max_depth = 0;
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back(oxygen);
    while !queue.is_empty() {
        // BFS looks at each "distance" from the start at a time
        let mut queue_size = queue.len();
        while queue_size > 0 {
            let current = queue.pop_back().unwrap();
            // add each of it's children to the queue
            for dir in 1..=4 {
                let child = (
                    current.0 + Direction::from(dir).dx(),
                    current.1 + Direction::from(dir).dy(),
                );
                if map[&child] != TileType::Wall && !visited.contains(&child) {
                    visited.insert(child);
                    queue.push_front(child);
                }
            }
            queue_size -= 1;
        }

        max_depth += 1;
    }

    max_depth - 1
}
