use std::collections::{BTreeMap, HashSet};
use std::ops::Index;

struct AsteroidMap {
    map: Vec<MapValue>,
    width: usize,
    _height: usize,
}

struct AsteroidMapIter<'a> {
    map: &'a AsteroidMap,
    index: usize,
}

impl AsteroidMap {
    fn iter(&self) -> AsteroidMapIter {
        AsteroidMapIter {
            map: self,
            index: 0,
        }
    }
}

impl<'a> Iterator for AsteroidMapIter<'a> {
    type Item = (usize, usize, &'a MapValue);

    fn next(&mut self) -> Option<Self::Item> {
        let x = self.index % self.map.width;
        let y = self.index / self.map.width;
        let result = Some((x, y, self.map.map.get(self.index)?));
        self.index += 1;
        result
    }
}

impl Index<(usize, usize)> for AsteroidMap {
    type Output = MapValue;

    fn index(&self, pos: (usize, usize)) -> &Self::Output {
        &self.map[pos.1 * self.width + pos.0]
    }
}

#[derive(PartialEq, Eq, Debug)]
enum MapValue {
    Empty,
    Asteroid,
}

#[aoc_generator(day10, part1)]
fn parse_asteroid_map(input: &str) -> AsteroidMap {
    AsteroidMap {
        map: input
            .lines()
            .flat_map(|line| {
                line.chars()
                    .map(|c| match c {
                        '.' => MapValue::Empty,
                        '#' => MapValue::Asteroid,
                        e => panic!("Unrecognized map value {:?}", e),
                    })
                    .collect::<Vec<_>>()
            })
            .collect(),
        width: input.lines().next().unwrap().len(),
        _height: input.lines().count(),
    }
}

#[aoc(day10, part1)]
fn solve_p1(map: &AsteroidMap) -> usize {
    let mut max_visible = 0usize;

    for (station_x, station_y, station_value) in map.iter() {
        if *station_value == MapValue::Empty {
            continue;
        }
        let mut angles = HashSet::new();
        'a: for (other_x, other_y, other_value) in map.iter() {
            if *other_value == MapValue::Empty {
                continue 'a;
            }
            if station_x == other_x && station_y == other_y {
                continue;
            }

            let dx = other_x as isize - station_x as isize;
            let dy = other_y as isize - station_y as isize;

            angles.insert((f64::atan2(dy as f64, dx as f64) * 1000.0) as i64);
        }

        if angles.len() > max_visible {
            max_visible = angles.len();
        }
    }

    max_visible
}

#[aoc_generator(day10, part2)]
fn p2_generator(input: &str) -> AsteroidMap {
    parse_asteroid_map(input)
}

#[aoc(day10, part2)]
fn solve_p2(map: &AsteroidMap) -> usize {
    let mut best_angles = BTreeMap::new();

    for (station_x, station_y, station_value) in map.iter() {
        if *station_value == MapValue::Empty {
            continue;
        }
        let mut angles = BTreeMap::new();
        'a: for (other_x, other_y, other_value) in map.iter() {
            if *other_value == MapValue::Empty {
                continue 'a;
            }
            if station_x == other_x && station_y == other_y {
                continue;
            }

            let dx = other_x as isize - station_x as isize;
            let dy = other_y as isize - station_y as isize;

            // We're converting to i64 (after multiplying by 10) because f64 doesn't implement
            let angle = (f64::atan2(dy as f64, dx as f64).to_degrees() * 10.0) as i64;

            // We want to start at 90 degrees, and go clockwise, not counterclockwise

            angles
                .entry(angle)
                .or_insert(Vec::new())
                .push((other_x, other_y));
        }

        if angles.len() > best_angles.len() {
            best_angles = angles;
        }
    }

    // So now, start at -900, and increase it
    let mut counter = 0;
    let mut vec = best_angles.into_iter().collect::<Vec<_>>();

    // Start at -90 degrees
    let mut i = vec.iter().position(|(angle, _)| *angle >= -900).unwrap();
    // We already got t
    while counter < 199 {
        vec[i].1.remove(0);
        i += 1;
        i %= vec.len();
        counter += 1;
    }

    let pos = vec[i].1[0];

    pos.0 * 100 + pos.1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day10_sample1() {
        let input = "
......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####
"
        .trim();
        assert_eq!(solve_p1(&parse_asteroid_map(input)), 33);
    }

    #[test]
    fn day10_sample2() {
        let input = "#.#...#.#.
.###....#.
.#....#...
##.#.#.#.#
....#.#.#.
.##..###.#
..#...##..
..##....##
......#...
.####.###.
";
        assert_eq!(solve_p1(&parse_asteroid_map(input)), 35);
    }
}
