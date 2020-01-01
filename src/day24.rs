use std::ops::Index;
use std::fmt;
use std::iter::FromIterator;
use std::collections::{HashSet, VecDeque};

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
enum Tile {
    Bug,
    Empty
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct ErisMap {
    map: Vec<Tile>,
    width: usize,
    height: usize,
}


struct ErisMapIter<'a> {
    map: &'a ErisMap,
    index: usize,
}

impl<'a> Iterator for ErisMapIter<'a> {
    type Item = (usize, usize, &'a Tile);

    fn next(&mut self) -> Option<Self::Item> {
        let x = self.index % self.map.width;
        let y = self.index / self.map.width;
        let result = Some((x, y, self.map.map.get(self.index)?));
        self.index += 1;
        result
    }
}


impl ErisMap {
    fn count_neighbors(&self, pos: (usize, usize)) -> usize {
        use crate::day17::DIRECTIONS;

        let mut count = 0;
        for dir in &DIRECTIONS {
            let x = pos.0 as i64 + dir.dx();
            let y = pos.1 as i64 + dir.dy();

            if x >= 0 && x < self.width as i64 && y >= 0 && y < self.height as i64 {
                match self[(x as usize, y as usize)] {
                    Tile::Bug => count += 1,
                    Tile::Empty => { },
                }
            }
        }

        count
    }

    fn iter(&self) -> ErisMapIter {
        ErisMapIter {
            map: self,
            index: 0,
        }
    }

    fn step(&self) -> ErisMap {
        // NOTE: I could make this an iterator, but 2 iterators might be confusing
        self.iter().map(|(x, y, tile)| {
            match tile {
                Tile::Bug => {
                    if self.count_neighbors((x, y)) == 1 {
                        Tile::Bug
                    } else {
                        Tile::Empty
                    }
                },
                Tile::Empty => {
                    let neighbors = self.count_neighbors((x, y));
                    if neighbors == 1 || neighbors == 2 {
                        Tile::Bug
                    } else {
                        Tile::Empty
                    }
                }
            }
        }).collect()
    }
}

impl Index<(usize, usize)> for ErisMap {
    type Output = Tile;

    fn index(&self, pos: (usize, usize)) -> &Self::Output {
        &self.map[pos.1 * self.width + pos.0]
    }
}

/// Creates an ErisMap from an ErisMapIter
/// ASSUMES `width` and `height` are 5
impl FromIterator<Tile> for ErisMap {
    fn from_iter<I: IntoIterator<Item=Tile>>(iter: I) -> Self {
        ErisMap {
            map: iter.into_iter().collect(),
            width: 5,  // Actually handle this correctly
            height: 5,
        }
    }
}

impl fmt::Debug for ErisMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(
                    f,
                    "{}",
                    match self[(x, y)] {
                        Tile::Bug => '#',
                        Tile::Empty => ' ',
                    }
                )?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}



#[aoc_generator(day24)]
fn parse_map(input: &str) -> ErisMap {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();

    let mut map = Vec::new();

    for line in input.lines() {
        for c in line.chars() {
            let tile_type = match c {
                '#' => Tile::Bug,
                '.' => Tile::Empty,
                c => panic!("Unrecognized tile: {:?}", c),
            };
            map.push(tile_type);
        }
    }

    ErisMap {
        map: map,
        width: width,
        height: height,
    }

}

#[aoc(day24, part1)]
fn solve_p1(map: &ErisMap) -> usize {
    let mut past = HashSet::new();
    past.insert(map.clone());
    let mut map = map.clone();
    loop {
        let new_map = map.step();
        if past.contains(&new_map) {
            return new_map.iter().map(|(x, y, tile)| {
                let i = y * map.width + x;
                match tile {
                    Tile::Empty => 0,
                    Tile::Bug => 2usize.pow(i as u32),
                }
            }).sum();
        }
        map = new_map;
        past.insert(map.clone());
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct RecursiveErisMap {
    map: VecDeque<Vec<Tile>>,
    width: usize,
    height: usize,
}

impl fmt::Debug for RecursiveErisMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for depth in self.map.len() {
            println!("Depth {}", depth);
            for y in 0..self.height {
                for x in 0..self.width {
                    write!(
                        f,
                        "{}",
                        match self[(depth, x, y)] {
                            Tile::Bug => '#',
                            Tile::Empty => ' ',
                        }
                    )?;
                }
                writeln!(f)?;
            }
        }
        Ok(())
    }
}


/// Indexes the RecursiveErisMap using tuple (depth, x, y)
impl Index<(usize, usize, usize)> for RecursiveErisMap {
    type Output = Tile;

    fn index(&self, pos: (usize, usize, usize)) -> &Self::Output {
        &self.map[pos.0][pos.2 * self.width + pos.1]
    }
}



impl RecursiveErisMap {
    fn count_neighbors(&self, pos: (usize, usize, usize)) -> usize {
        use crate::day17::DIRECTIONS;

        let mut count = 0;
        for dir in &DIRECTIONS {
            let x = pos.1 as i64 + dir.dx();
            let y = pos.2 as i64 + dir.dy();

            if x >= 0 && x < self.width as i64 && y >= 0 && y < self.height as i64 {
                match self[(pos.0, x as usize, y as usize)] {
                    Tile::Bug => count += 1,
                    Tile::Empty => { },
                }
            } else {
                // we need to check the (depth - 1) layer
            }
        }

        count
    }
}


#[aoc(day24, part2)]
fn solve_p2(map: &ErisMap) -> usize {
    0
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day24_test1() {
        let input = &"
....#
#..#.
#..##
..#..
#...."[1..];
        assert_eq!(2129920, solve_p1(&parse_map(input)));
    }
}
