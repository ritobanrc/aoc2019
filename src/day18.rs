use crate::day17::DIRECTIONS;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum TileType {
    Player,
    Empty,
    Wall,
    Key(u8),
    Door(u8),
}

#[derive(Clone)]
struct TunnelMap {
    map: HashMap<(usize, usize), TileType>,
    width: usize,
    height: usize,
    player_pos: (usize, usize),
    keys: i32,
}

impl fmt::Debug for TunnelMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(
                    f,
                    "{}",
                    match self.map[&(x, y)] {
                        TileType::Player => '▣',
                        TileType::Empty => ' ',
                        TileType::Wall => '█',
                        TileType::Door(a) => (a + 'A' as u8) as char,
                        TileType::Key(a) => (a + 'a' as u8) as char,
                    }
                )?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[aoc_generator(day18)]
fn parse_map(input: &str) -> TunnelMap {
    let mut map = HashMap::new();
    let mut player_pos = (0, 0);
    let mut keys = 0i32;

    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let tile_type = match c {
                '@' => {
                    player_pos = (x, y);
                    TileType::Player
                }
                '.' => TileType::Empty,
                '#' => TileType::Wall,
                c if c.is_uppercase() => TileType::Door(c as u8 - 'A' as u8),
                c if c.is_lowercase() => {
                    let idx = c as u8 - 'a' as u8;
                    keys += 1 << (idx as u32);
                    TileType::Key(idx)
                }
                c => panic!("Unrecognized {:?}", c),
            };
            map.insert((x, y), tile_type);
        }
    }

    TunnelMap {
        map,
        width,
        height,
        player_pos,
        keys,
    }
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct State {
    missing_keys: i32,
    pos: (usize, usize),
}

impl State {
    fn new(map: &TunnelMap) -> Self {
        State {
            missing_keys: map.keys,
            pos: map.player_pos,
        }
    }
}

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "State {{ missing_keys: {:b}, pos: {:?} }}",
            self.missing_keys, self.pos
        )
    }
}

fn count_steps(map: &TunnelMap) -> usize {
    use std::collections::VecDeque;

    let mut queue = VecDeque::new();
    let mut steps_to = HashMap::new();

    queue.push_back(State::new(&map));
    steps_to.insert(State::new(&map), 0);
    // so we start at
    while queue.len() > 0 {
        let current = queue.pop_front().unwrap();
        if current.missing_keys == 0 {
            return steps_to[&current];
        }
        if !steps_to.contains_key(&current) {
            continue;
        }
        for dir in &DIRECTIONS {
            let new_pos = (
                (current.pos.0 as i64 + dir.dx()) as usize,
                (current.pos.1 as i64 + dir.dy()) as usize,
            );

            let next_state = match map.map.get(&new_pos) {
                Some(TileType::Wall) => continue,
                Some(TileType::Door(a)) => {
                    if (current.missing_keys >> a) & 1 == 0 {
                        State {
                            missing_keys: current.missing_keys,
                            pos: new_pos,
                        }
                    } else {
                        continue;
                    }
                }
                Some(TileType::Key(a)) => State {
                    missing_keys: match (current.missing_keys >> a) & 1 {
                        1 => current.missing_keys ^ (1 << a),
                        _ => current.missing_keys,
                    },
                    pos: new_pos,
                },
                Some(TileType::Player) | Some(TileType::Empty) | None => State {
                    missing_keys: current.missing_keys,
                    pos: new_pos,
                },
            };

            if !steps_to.contains_key(&next_state) {
                let steps = steps_to[&current] + 1;
                steps_to.insert(next_state, steps);
                queue.push_back(next_state);
            }
        }
    }

    0
}

#[aoc(day18, part1)]
fn solve_p1(map: &TunnelMap) -> usize {
    println!("{:?}", map);
    count_steps(map)
}

#[aoc(day18, part2)]
fn solve_p2(map: &TunnelMap) -> usize {
    let mut new_map = map.clone();

    new_map.map.insert(map.player_pos, TileType::Wall);
    new_map
        .map
        .insert((map.player_pos.0, map.player_pos.1 + 1), TileType::Wall);
    new_map
        .map
        .insert((map.player_pos.0, map.player_pos.1 - 1), TileType::Wall);
    new_map
        .map
        .insert((map.player_pos.0 + 1, map.player_pos.1), TileType::Wall);
    new_map
        .map
        .insert((map.player_pos.0 - 1, map.player_pos.1), TileType::Wall);
    new_map.map.insert(
        (map.player_pos.0 + 1, map.player_pos.1 + 1),
        TileType::Player,
    );
    new_map.map.insert(
        (map.player_pos.0 + 1, map.player_pos.1 - 1),
        TileType::Player,
    );
    new_map.map.insert(
        (map.player_pos.0 - 1, map.player_pos.1 + 1),
        TileType::Player,
    );
    new_map.map.insert(
        (map.player_pos.0 - 1, map.player_pos.1 - 1),
        TileType::Player,
    );

    let mut total = 0;
    for quadrant in 0..4 {
        let quadrant_map: HashMap<(usize, usize), TileType> = new_map
            .map
            .clone()
            .into_iter()
            .filter(|&(k, _v)| match quadrant {
                0 => k.0 >= map.player_pos.0 && k.1 >= map.player_pos.1,
                1 => k.0 >= map.player_pos.0 && k.1 <= map.player_pos.1,
                2 => k.0 <= map.player_pos.0 && k.1 >= map.player_pos.1,
                3 => k.0 <= map.player_pos.0 && k.1 <= map.player_pos.1,
                _ => unreachable!(),
            })
            .collect();

        let keys = quadrant_map
            .iter()
            .map(|(_k, v)| if let TileType::Key(a) = v { 1 << a } else { 0 })
            .sum();

        let map1 = TunnelMap {
            map: quadrant_map,
            width: map.width / 2, // these don't actually work because the printing code goes from 0..width
            height: map.height / 2,
            player_pos: match quadrant {
                0 => (map.player_pos.0 + 1, map.player_pos.1 + 1),
                1 => (map.player_pos.0 + 1, map.player_pos.1 - 1),
                2 => (map.player_pos.0 - 1, map.player_pos.1 + 1),
                3 => (map.player_pos.0 - 1, map.player_pos.1 - 1),
                _ => unreachable!(),
            },
            keys,
        };

        let steps = count_steps(&map1);
        total += steps;
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day18_test1() {
        let input = "
########################
#...............b.C.D.f#
#.######################
#.....@.a.B.c.d.A.e.F.g#
########################"
            .trim();
        assert_eq!(132, solve_p1(&parse_map(input)));
    }

    #[test]
    fn day18_test2() {
        let input = "
#################
#i.G..c...e..H.p#
########.########
#j.A..b...f..D.o#
########@########
#k.E..a...g..B.n#
########.########
#l.F..d...h..C.m#
#################"
            .trim();
        assert_eq!(136, solve_p1(&parse_map(input)));
    }

    #[test]
    fn day18_test3() {
        let input = "
########################
#@..............ac.GI.b#
###d#e#f################
###A#B#C################
###g#h#i################
########################"
            .trim();
        assert_eq!(81, solve_p1(&parse_map(input)));
    }
}
