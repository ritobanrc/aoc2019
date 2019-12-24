use crate::day17::DIRECTIONS;
use std::collections::{HashMap, VecDeque};

enum Tile {
    Wall,
    Passage,
    Portal(char, char),
}

type Pos = (usize, usize);

struct Maze {
    map: HashMap<Pos, Tile>,
    width: usize,
    height: usize,
    portals: HashMap<(char, char), (Pos, Pos)>,
}

#[aoc_generator(day20)]
fn parse_maze(input: &str) -> Maze {
    let mut map = HashMap::new();
    let mut portals: HashMap<(char, char), (Pos, Pos)> = HashMap::new();

    let width = input.find('\n').unwrap();
    let height = input.lines().count();

    let input = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    println!("{:?} by {:?}", input[0].len(), input.len());

    for y in 1..input.len() - 1 {
        for x in 1..input[0].len() - 1 {
            let c = input[y][x];
            match c {
                '#' => {
                    map.entry((x, y)).or_insert(Tile::Wall);
                }
                '.' => {
                    map.entry((x, y)).or_insert(Tile::Passage);
                }
                c if c.is_uppercase() => {
                    let neighbours = [
                        (input[y - 1][x], y - 1, x),
                        (input[y + 1][x], y + 1, x),
                        (input[y][x - 1], y, x - 1),
                        (input[y][x + 1], y, x + 1),
                    ];
                    if let Some((_empty, passage_y, passage_x)) =
                        neighbours.iter().find(|&(empty, _y, _x)| *empty == '.')
                    {
                        let other_letter = neighbours
                            .iter()
                            .find(|&(other, _y, _x)| other.is_uppercase())
                            .unwrap();
                        let name = if (x == other_letter.2 && y > other_letter.1)
                            || (y == other_letter.1 && x > other_letter.2)
                        {
                            (other_letter.0, c)
                        } else {
                            (c, other_letter.0)
                        };
                        map.insert((*passage_x, *passage_y), Tile::Portal(name.0, name.1));

                        if *passage_x == 2
                            || *passage_y == 2
                            || *passage_x > width - 5
                            || *passage_y > height - 5
                        {
                            portals.entry(name).or_default().1 = (*passage_x, *passage_y);
                        } else {
                            portals.entry(name).or_default().0 = (*passage_x, *passage_y);
                        }
                    };
                    continue;
                }
                _ => continue,
            }
        }
    }

    Maze {
        map,
        width,
        height,
        portals,
    }
}

#[aoc(day20, part1)]
fn solve_p1(maze: &Maze) -> usize {
    for y in 0..maze.height {
        for x in 0..maze.width {
            match maze.map.get(&(x, y)) {
                Some(Tile::Wall) => print!("#"),
                Some(Tile::Passage) => print!("."),
                Some(Tile::Portal(a, _b)) => print!("{}", a),
                None => print!(" "),
            }
        }
        println!();
    }

    let mut queue = VecDeque::new();
    let mut steps_to = HashMap::new();

    let start = maze.portals[&('A', 'A')].1;
    let goal = maze.portals[&('Z', 'Z')].1;

    queue.push_back(start);
    steps_to.insert(start, 0);
    // so we start at
    while queue.len() > 0 {
        let current = queue.pop_front().unwrap();

        if current == goal {
            return steps_to[&current];
        }

        if current != start && !steps_to.contains_key(&current) {
            continue;
        }

        if let Some(Tile::Portal(a, b)) = maze.map.get(&current) {
            let portal = maze.portals.get(&(*a, *b)).unwrap();
            if current == portal.0 {
                let new_pos = portal.1;
                if !steps_to.contains_key(&new_pos) {
                    let steps = steps_to[&current] + 1;
                    steps_to.insert(new_pos, steps);
                    queue.push_back(new_pos);
                }
            }
        }

        for dir in &DIRECTIONS {
            let new_pos = (
                (current.0 as i64 + dir.dx()) as usize,
                (current.1 as i64 + dir.dy()) as usize,
            );

            if new_pos.0 > maze.width || new_pos.1 > maze.height {
                continue;
            }

            if steps_to.contains_key(&new_pos) {
                continue;
            }

            match maze.map.get(&new_pos) {
                None | Some(Tile::Wall) => continue,
                Some(Tile::Passage) | Some(Tile::Portal(_, _)) => {
                    let steps = steps_to[&current] + 1;
                    steps_to.insert(new_pos, steps);
                    queue.push_back(new_pos);
                }
            };
        }
    }

    0
}

#[aoc(day20, part2)]
fn solve_p2(maze: &Maze) -> usize {
    let mut queue = VecDeque::new();
    let mut steps_to = HashMap::new();

    let start = (maze.portals[&('A', 'A')].1, 0);
    let goal = (maze.portals[&('Z', 'Z')].1, 0);

    // we start at level 0
    queue.push_back(start);
    steps_to.insert(start, 0);

    while queue.len() > 0 {
        let current = queue.pop_front().unwrap();

        if current == goal {
            return steps_to[&current];
        }

        if current != start && !steps_to.contains_key(&current) {
            continue;
        }

        let (current_pos, current_level) = current;

        if let Some(Tile::Portal(a, b)) = maze.map.get(&current_pos) {
            let portal = maze.portals.get(&(*a, *b)).unwrap();
            if current_pos == portal.0 {
                // we recurse into a level deeper
                let new_pos = (portal.1, current_level + 1);
                if !steps_to.contains_key(&new_pos) {
                    let steps = steps_to[&current] + 1;
                    steps_to.insert(new_pos, steps);
                    queue.push_back(new_pos);
                }
            } else if current_pos == portal.1 && current_level > 0 {
                let new_pos = (portal.0, current_level - 1);
                if !steps_to.contains_key(&new_pos) {
                    let steps = steps_to[&current] + 1;
                    steps_to.insert(new_pos, steps);
                    queue.push_back(new_pos);
                }
            }
        }

        for dir in &DIRECTIONS {
            let new_pos = (
                (
                    (current_pos.0 as i64 + dir.dx()) as usize,
                    (current_pos.1 as i64 + dir.dy()) as usize,
                ),
                current_level,
            );

            if (new_pos.0).0 > maze.width || (new_pos.0).1 > maze.height {
                continue;
            }

            if steps_to.contains_key(&new_pos) {
                continue;
            }

            match maze.map.get(&new_pos.0) {
                None | Some(Tile::Wall) => continue,
                Some(Tile::Passage) | Some(Tile::Portal(_, _)) => {
                    let steps = steps_to[&current] + 1;
                    steps_to.insert(new_pos, steps);
                    queue.push_back(new_pos);
                }
            };
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day20_test1() {
        let maze = parse_maze(
            &"
         A           
         A           
  #######.#########  
  #######.........#  
  #######.#######.#  
  #######.#######.#  
  #######.#######.#  
  #####  B    ###.#  
BC...##  C    ###.#  
  ##.##       ###.#  
  ##...DE  F  ###.#  
  #####    G  ###.#  
  #########.#####.#  
DE..#######...###.#  
  #.#########.###.#  
FG..#########.....#  
  ###########.#####  
             Z       
             Z       "[1..],
        );
        assert_eq!(23, solve_p1(&maze));
    }

    #[test]
    fn day20_test2() {
        let maze = parse_maze(
            &"
             Z L X W       C                 
             Z P Q B       K                 
  ###########.#.#.#.#######.###############  
  #...#.......#.#.......#.#.......#.#.#...#  
  ###.#.#.#.#.#.#.#.###.#.#.#######.#.#.###  
  #.#...#.#.#...#.#.#...#...#...#.#.......#  
  #.###.#######.###.###.#.###.###.#.#######  
  #...#.......#.#...#...#.............#...#  
  #.#########.#######.#.#######.#######.###  
  #...#.#    F       R I       Z    #.#.#.#  
  #.###.#    D       E C       H    #.#.#.#  
  #.#...#                           #...#.#  
  #.###.#                           #.###.#  
  #.#....OA                       WB..#.#..ZH
  #.###.#                           #.#.#.#  
CJ......#                           #.....#  
  #######                           #######  
  #.#....CK                         #......IC
  #.###.#                           #.###.#  
  #.....#                           #...#.#  
  ###.###                           #.#.#.#  
XF....#.#                         RF..#.#.#  
  #####.#                           #######  
  #......CJ                       NM..#...#  
  ###.#.#                           #.###.#  
RE....#.#                           #......RF
  ###.###        X   X       L      #.#.#.#  
  #.....#        F   Q       P      #.#.#.#  
  ###.###########.###.#######.#########.###  
  #.....#...#.....#.......#...#.....#.#...#  
  #####.#.###.#######.#######.###.###.#.#.#  
  #.......#.......#.#.#.#.#...#...#...#.#.#  
  #####.###.#####.#.#.#.#.###.###.#.###.###  
  #.......#.....#.#...#...............#...#  
  #############.#.#.###.###################  
               A O F   N                     
               A A D   M                     "[1..],
        );
        println!("{:?}", solve_p2(&maze));
    }
}
