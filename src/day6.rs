use std::collections::{HashMap, VecDeque};

#[aoc_generator(day6, part1)]
fn parse_orbits(input: &str) -> (Vec<(usize, usize)>, usize) {
    let mut satellites = Vec::new();
    (
        input
            .lines()
            .map(|x| {
                let mut iter = x.split(')');
                let orbitee = iter.next().unwrap();
                let orbiter = iter.next().unwrap();
                let orbitee_idx = satellites
                    .iter()
                    .position(|x| *x == orbitee)
                    .unwrap_or_else(|| {
                        satellites.push(orbitee);
                        satellites.len() - 1
                    });
                let orbiter_idx = satellites
                    .iter()
                    .position(|x| *x == orbiter)
                    .unwrap_or_else(|| {
                        satellites.push(orbiter);
                        satellites.len() - 1
                    });
                (orbitee_idx, orbiter_idx)
            })
            .collect::<Vec<_>>(),
        satellites.len(),
    )
}

#[aoc(day6, part1)]
fn solve_p1(orbits: &(Vec<(usize, usize)>, usize)) -> usize {
    let num_satellites = orbits.1;
    let orbits = &orbits.0;
    // Start by finding root node of the graph
    // It orbits nothing (0)
    // Give each of its children 1 orbit
    let mut root = None;
    for sat in 0..num_satellites {
        if orbits.iter().all(|(_orbitee, orbiter)| sat != *orbiter) {
            root = Some(sat);
            break;
        }
    }

    let root = root.expect("Could not find root node");

    let mut stack = Vec::new();
    stack.push((root, 0));
    let mut total_count = 0;
    while stack.len() > 0 {
        let (node, depth) = stack.pop().unwrap();
        for (orbitee, orbiter) in orbits.iter() {
            if *orbitee == node {
                // This is a child of our current node
                stack.push((*orbiter, depth + 1));
            }
        }
        total_count += depth;
    }

    total_count
}
struct Part2Input {
    orbits: Vec<(usize, usize)>,
    you_idx: usize,
    san_idx: usize,
}

#[aoc_generator(day6, part2)]
fn parse_orbits_p2(input: &str) -> Part2Input {
    let mut satellites = Vec::new();
    let mut you_idx = 0;
    let mut san_idx = 0;
    let orbits = input
        .lines()
        .map(|x| {
            let mut iter = x.split(')');
            let orbitee = iter.next().unwrap();
            let orbiter = iter.next().unwrap();
            let orbitee_idx = satellites
                .iter()
                .position(|x| *x == orbitee)
                .unwrap_or_else(|| {
                    satellites.push(orbitee);
                    satellites.len() - 1
                });
            let orbiter_idx = satellites
                .iter()
                .position(|x| *x == orbiter)
                .unwrap_or_else(|| {
                    satellites.push(orbiter);
                    satellites.len() - 1
                });
            if orbiter == "YOU" {
                you_idx = orbiter_idx;
            }
            if orbiter == "SAN" {
                san_idx = orbiter_idx;
            }
            (orbitee_idx, orbiter_idx)
        })
        .collect::<Vec<_>>();

    Part2Input {
        orbits,
        you_idx,
        san_idx,
    }
}

#[aoc(day6, part2)]
fn solve_p2(input: &Part2Input) -> usize {
    let orbits = &input.orbits;

    let mut queue = VecDeque::new();
    let mut visited = HashMap::new();
    queue.push_back(input.you_idx);

    while queue.len() > 0 {
        let node = queue.pop_front().unwrap();
        if node == input.san_idx {
            break;
        }
        for (orbitee, orbiter) in orbits.iter() {
            if *orbitee == node && !visited.contains_key(orbiter) {
                // This is a child of our current node
                visited.insert(orbiter, node);
                queue.push_back(*orbiter);
            } else if *orbiter == node && !visited.contains_key(orbitee) {
                // This is a child of our current node
                visited.insert(orbitee, node);
                queue.push_back(*orbitee);
            }
        }
    }

    let mut total_count = 0;
    let mut current = visited[&input.san_idx];
    loop {
        current = visited[&current];
        if current == input.you_idx {
            break;
        }
        total_count += 1;
    }
    total_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn orbit_test() {
        let input = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN
";
        assert_eq!(solve_p2(&parse_orbits_p2(input)), 4);
    }
}
