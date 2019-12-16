const WIDTH: usize = 25;
const HEIGHT: usize = 6;

#[aoc_generator(day8, part1)]
fn parse_images(input: &str) -> Vec<Vec<u8>> {
    let data: Vec<u8> = input.bytes().map(|c| c - 48).collect();

    let chunks: Vec<_> = data.chunks(WIDTH * HEIGHT).map(|c| c.to_vec()).collect();

    chunks
}

#[aoc(day8, part1)]
fn solve_p1(input: &[Vec<u8>]) -> usize {
    let layer = input
        .iter()
        .min_by_key(|v| bytecount::count(v, 0))
        .unwrap();

    bytecount::count(layer, 1) * bytecount::count(layer, 2)
}

#[aoc_generator(day8, part2)]
fn parse_p2(input: &str) -> Vec<Vec<u8>> {
    parse_images(input)
}

#[aoc(day8, part2)]
fn solve_p2(input: &[Vec<u8>]) -> usize {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let idx = y * WIDTH + x;
            let mut color = 2; // every pixel stars out transparent
                               // going from the top layer down
            for pix in input.iter().map(|layer| layer[idx]) {
                match pix {
                    2 => continue, // ignore transparent layers
                    1 | 0 => {
                        if color == 2 {
                            color = pix;
                        }
                    }
                    _ => panic!("Unrecognized color: {:?}", pix),
                }
            }

            print!(
                "{}",
                match color {
                    0 => " ",
                    1 => "■",
                    _ => panic!("Unrecognized color: {:?}", color),
                }
            );
        }
        println!();
    }
    0
}
