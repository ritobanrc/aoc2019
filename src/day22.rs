#[derive(Debug)]
enum Instruction {
    Reverse,
    Interleave(usize),
    Cut(isize),
}

#[aoc_generator(day22)]
fn parse_shuffle(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            if line == "deal into new stack" {
                return Instruction::Reverse;
            } else if line.starts_with("deal with increment") {
                Instruction::Interleave(line.split(' ').last().unwrap().parse().unwrap())
            } else if line.starts_with("cut") {
                Instruction::Cut(line.split(' ').last().unwrap().parse().unwrap())
            } else {
                panic!("Unknown line: {:?}", line);
            }
        })
        .collect()
}

#[aoc(day22, part1)]
fn solve_p1(shuffle: &[Instruction]) -> usize {
    let mut deck = (0..10007).collect::<Vec<usize>>();
    for ins in shuffle {
        match ins {
            Instruction::Reverse => {
                deck[..].reverse();
            }
            Instruction::Cut(amount) => {
                // 4674
                // 3460 + (10007 - 4674)
                let n = if *amount > 0 {
                    *amount as usize
                } else {
                    (deck.len() as isize - amount.abs()) as usize
                };
                deck = [&deck[n..], &deck[..n]].concat();
            }
            Instruction::Interleave(skip) => {
                let mut table = vec![0; deck.len()];
                for (i, x) in deck.iter().enumerate() {
                    table[(i * skip) % deck.len()] = *x;
                }
                deck = table;
            }
        }
    }

    deck.iter().position(|x| *x == 2019).unwrap()
}

#[aoc(day22, part2)]
fn solve_p2(shuffle: &[Instruction]) -> u64 {
    let mut idx = 2020;
    let deck_length = 119315717514047u64;
    //let deck_length = 10007;

    //for _ in 0..101741582076661u64 {
    for _ in 0..100u64 {
        for ins in shuffle.iter().rev() {
            match ins {
                Instruction::Reverse => {
                    idx = deck_length - idx - 1;
                }
                Instruction::Cut(amount) => {
                    let n = if *amount > 0 {
                        *amount as u64
                    } else {
                        (deck_length as isize - amount.abs()) as u64
                    };

                    if idx <= (deck_length - n) {
                        idx += n;
                    } else {
                        idx = idx - (deck_length - n);
                    }
                }
                Instruction::Interleave(skip) => {
                    // curr_idx = prev_idx * skip (mod deck_length)
                    idx = (idx * mod_inv(*skip as u64, deck_length)) % deck_length;
                }
            }
        }
        println!("{:?}", idx);
    }

    // so this gives us a sequence of numbers
    // we now need to reverse the LCG

    idx
}

/// Precondition: a is coprie to m
fn mod_inv(a: u64, m: u64) -> u64 {
    power(a, m - 2, m)
}

// To compute x^y under modulo m
fn power(x: u64, y: u64, m: u64) -> u64 {
    if y == 0 {
        return 1;
    }

    let mut p = power(x, y / 2, m) % m;
    p = (p * p) % m;

    if y % 2 == 0 {
        p
    } else {
        (x * p) % m
    }
}
// Example
//
//
// 0 1 2 3 4 5 6 7 8 9
// deal with increment 7
// 0 3     2 5   1 4
// deal into new stack
// deal into new stack
// Result: 0 3 6 9 2 5 8 1 4 7
