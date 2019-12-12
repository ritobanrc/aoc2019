use itertools::Itertools;
use regex::Regex;

#[derive(Debug, Clone)]
struct Moon {
    position: [i32; 3],
    velocity: [i32; 3],
}

#[aoc_generator(day12, part1)]
fn load_moons(input: &str) -> Vec<Moon> {
    let re = Regex::new(r"<x=(-?\d+), y=(-?\d+), z=(-?\d+)>").unwrap();

    re.captures_iter(input).map(|cap| {
        Moon {
            position: [cap[1].parse::<i32>().unwrap(),
                       cap[2].parse::<i32>().unwrap(),
                       cap[3].parse::<i32>().unwrap()] ,
            velocity: [0, 0, 0]
        }
    }).collect()
}

#[aoc(day12, part1)]
fn solve_p1(moons: &Vec<Moon>) -> i32 {
    // Repeat for 1000 steps
    let mut moons = moons.clone();
    for _ in 0..1000 {
        let mut next_moons = moons.clone();
        for pair in moons.iter().enumerate().combinations(2) {
            for (i, (a, b)) in pair[0].1.position.iter().zip(&pair[1].1.position).enumerate() {
                if a > b {
                    next_moons[pair[0].0].velocity[i] -= 1;
                    next_moons[pair[1].0].velocity[i] += 1;
                } else if a < b {
                    next_moons[pair[0].0].velocity[i] += 1;
                    next_moons[pair[1].0].velocity[i] -= 1;
                } 
            }
        }

        for moon in next_moons.iter_mut() {
            for (pos, vel) in moon.position.iter_mut().zip(&moon.velocity) {
                *pos += *vel
            }
        }

        moons = next_moons;
        //println!("{:?}", moons);
    }

    moons.iter().map(|moon| {
        let pot: i32 = moon.position.iter().map(|a| a.abs()).sum();
        let kin: i32 = moon.velocity.iter().map(|a| a.abs()).sum();

        pot * kin
    }).sum()
}
