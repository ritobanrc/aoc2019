use std::collections::HashSet;

#[derive(Debug)]
enum Step {
    Up(u32),
    Down(u32),
    Left(u32),
    Right(u32),
}

impl Step {
    fn size(&self) -> u32 {
        match *self {
            Step::Up(x) | Step::Down(x) | Step::Left(x) | Step::Right(x) => x,
        }
    }

    fn dir(&self) -> [i32; 2] {
        match *self {
            Step::Up(_) => [0, -1],
            Step::Down(_) => [0, 1],
            Step::Left(_) => [-1, 0],
            Step::Right(_) => [1, 0],
        }
    }
}

#[aoc_generator(day3, part1)]
fn parse_wires(input: &str) -> [Vec<Step>; 2] {
    let mut wires = input.lines().map(|line| {
        let path_text = line.split(',');
        let mut path = Vec::new();

        for step in path_text {
            let step = match step.chars().nth(0).unwrap() {
                'U' => Step::Up(step[1..].parse().unwrap()),
                'D' => Step::Down(step[1..].parse().unwrap()),
                'L' => Step::Left(step[1..].parse().unwrap()),
                'R' => Step::Right(step[1..].parse().unwrap()),
                _ => panic!("Character not recognized"),
            };
            path.push(step);
        }

        path
    });

    [wires.next().unwrap(), wires.next().unwrap()]
}

#[aoc(day3, part1)]
fn solve_p1(wires: &[Vec<Step>; 2]) -> i32 {
    let (wire1_points, _) = points_from_steps(&wires[0]);
    let (wire2_points, _) = points_from_steps(&wires[1]);

    let intersections = wire1_points.intersection(&wire2_points);

    let int = intersections
        .min_by_key(|a| (a[0].abs() + a[1].abs()))
        .unwrap();

    int[0].abs() + int[1].abs()
}

#[aoc_generator(day3, part2)]
fn parse_wires_p2(input: &str) -> [Vec<Step>; 2] {
    parse_wires(input)
}

#[aoc(day3, part2)]
fn solve_p2(wires: &[Vec<Step>; 2]) -> usize {
    let (wire1_points, wire1_path) = points_from_steps(&wires[0]);
    let (wire2_points, wire2_path) = points_from_steps(&wires[1]);

    let intersections = wire1_points.intersection(&wire2_points);

    let int = intersections
        // Start by mapping each intersection to a (key, intersection) tuple
        // Where key is the total number of steps in both wires
        .map(|a| {
            let key = wire1_path
                .iter()
                .position(|x| x[0] == a[0] && x[1] == a[1])
                .unwrap()
                + 1
                + wire2_path
                    .iter()
                    .position(|x| x[0] == a[0] && x[1] == a[1])
                    .unwrap()
                + 1;
            (key, a)
        })
        .min_by_key(|a| a.0)
        .unwrap();

    int.0
}

fn points_from_steps(wire: &[Step]) -> (HashSet<[i32; 2]>, Vec<[i32; 2]>) {
    use std::iter::FromIterator;
    let path = path_from_steps(wire);
    (HashSet::from_iter(path.clone().into_iter()), path)
}

fn path_from_steps(wire: &[Step]) -> Vec<[i32; 2]> {
    let mut pos = [0, 0];
    let mut wire_points = Vec::new();

    for step in wire {
        let dir = step.dir();
        for _ in 0..step.size() {
            pos[0] += dir[0];
            pos[1] += dir[1];
            wire_points.push(pos);
        }
    }
    wire_points
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let wires = "R8,U5,L5,D3\nU7,R6,D4,L4";
        let wires = parse_wires(wires);
        assert_eq!(solve_p2(&wires), 30);
    }

    #[test]
    fn test1() {
        let wires = "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83";
        let wires = parse_wires(wires);
        assert_eq!(solve_p2(&wires), 610);
    }

    #[test]
    fn test2() {
        let wires =
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
        let wires = parse_wires(wires);
        assert_eq!(solve_p2(&wires), 410);
    }
}
