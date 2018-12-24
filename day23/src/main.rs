mod vec3;
mod nanobot;

use crate::vec3::Vec3;
use crate::nanobot::Nanobot;
use std::ops::Range;

fn parse_input(input: &str) -> Result<Vec<Nanobot>, &'static str> {
    let mut list = Vec::new();

    for line in input.lines() {
        let nanobot = Nanobot::parse(line)?;
        list.push(nanobot);
    }

    Ok(list)
}

fn main() {
    let input = include_str!("../input.txt");
    let bots = parse_input(&input)
        .expect("Could not parse input");

    println!("Puzzle 1: {}", puzzle_1(&bots));
    println!("Puzzle 2: {}", puzzle_2(&bots));
}

/// Finds the nanobot with the largest signal radius, returns how many other nanobots it can reach.
fn puzzle_1(bots: &Vec<Nanobot>) ->  usize {
    let max_bot = bots.iter()
        .max_by_key(|bot| bot.radius)
        .unwrap();

    bots.iter()
        .filter(|bot| max_bot.can_reach(&bot.pos))
        .count()
}

#[derive(Debug)]
struct Bounds {
    x: Range<isize>,
    y: Range<isize>,
    z: Range<isize>,
}

/// Find the bounds of hotspot of size `square` which is most likely to contain most bots within reach.
fn find_most_likely_square(bots: &Vec<Nanobot>, bounds: &Bounds, square: usize) -> Bounds {
    let isquare = square as isize;

    println!("Hotspot {}, given {:?}", square, bounds);

    let mut max_squares = Vec::new();
    let mut max_count = 0;

    for z in bounds.z.clone().step_by(square) {
        for y in bounds.y.clone().step_by(square) {
            for x in bounds.x.clone().step_by(square) {
                let mut square_max = 0;

                let inner_steps = square / 4;

                for z2 in (z..z + isquare).step_by(inner_steps) {
                    for y2 in (y..y + isquare).step_by(inner_steps) {
                        for x2 in (x..x + isquare).step_by(inner_steps) {
                            let count = Nanobot::bots_within_reach_of(&Vec3::new(x2, y2, z2), &bots);

                            if count > square_max {
                                square_max = count;
                            }
                        }
                    }
                }

                if square_max > max_count {
                    max_count = square_max;

                    max_squares.clear();
                    max_squares.push(Vec3::new(x, y, z));
                } else if square_max == max_count {
                    max_squares.push(Vec3::new(x, y, z));
                }
            }
        }
    }

    println!("Found {} squares w count: {}", max_squares.len(), max_count);

    let max_square = max_squares.iter()
        .min_by_key(|sq| Vec3::new(sq.x + isquare/2, sq.y + isquare/2, sq.z + isquare/2).distance_origin())
        .unwrap();

    Bounds {
        x: max_square.x..max_square.x + isquare,
        y: max_square.y..max_square.y + isquare,
        z: max_square.z..max_square.z + isquare,
    }
}

/// Finds the position that is in range of the most nanobots and is closest to (0, 0, 0).
fn puzzle_2(bots: &Vec<Nanobot>) -> usize {
    // determine bounds of space to search
    let min_x = bots.iter().map(|bot| bot.pos.x).min().unwrap();
    let max_x = bots.iter().map(|bot| bot.pos.x).max().unwrap();

    let min_y = bots.iter().map(|bot| bot.pos.y).min().unwrap();
    let max_y = bots.iter().map(|bot| bot.pos.y).max().unwrap();

    let min_z = bots.iter().map(|bot| bot.pos.z).min().unwrap();
    let max_z = bots.iter().map(|bot| bot.pos.z).max().unwrap();

    let bounds = Bounds { x: (min_x..max_x), y: (min_y..max_y), z: (min_z..max_z) };
    println!("Total space to search: {:?}", bounds);

    // reduce bounds to -hopefully- interesting hotspot
    let bounds = find_most_likely_square(&bots, &bounds, 134_217_728);
    let bounds = find_most_likely_square(&bots, &bounds, 67_108_864);
    let bounds = find_most_likely_square(&bots, &bounds, 33_554_432);
    let bounds = find_most_likely_square(&bots, &bounds, 16_777_216);
    let bounds = find_most_likely_square(&bots, &bounds, 8_388_608);
    let bounds = find_most_likely_square(&bots, &bounds, 4_194_304);
    let bounds = find_most_likely_square(&bots, &bounds, 2_097_152);
    let bounds = find_most_likely_square(&bots, &bounds, 1_048_576);
    let bounds = find_most_likely_square(&bots, &bounds, 524_288);
    let bounds = find_most_likely_square(&bots, &bounds, 262_144);
    let bounds = find_most_likely_square(&bots, &bounds, 131_072);
    let bounds = find_most_likely_square(&bots, &bounds, 65_536);
    let bounds = find_most_likely_square(&bots, &bounds, 32_768);
    let bounds = find_most_likely_square(&bots, &bounds, 16_384);
    let bounds = find_most_likely_square(&bots, &bounds, 8_192);
    let bounds = find_most_likely_square(&bots, &bounds, 4_096);
    let bounds = find_most_likely_square(&bots, &bounds, 2_048);
    let bounds = find_most_likely_square(&bots, &bounds, 1_024);
    let bounds = find_most_likely_square(&bots, &bounds, 512);
    let bounds = find_most_likely_square(&bots, &bounds, 256);
    let bounds = find_most_likely_square(&bots, &bounds, 128);
    let bounds = find_most_likely_square(&bots, &bounds, 64);
    let bounds = find_most_likely_square(&bots, &bounds, 32);
    let bounds = find_most_likely_square(&bots, &bounds, 16);
    let bounds = find_most_likely_square(&bots, &bounds, 8);

    // naive search attempt
    let mut positions = Vec::new();
    let mut max_count = 0;

    for z in bounds.z.clone() {
        for y in bounds.y.clone() {
            for x in bounds.x.clone() {
                let pos = Vec3::new(x, y, z);

                let count = Nanobot::bots_within_reach_of(&pos, &bots);

                if count > max_count {
                    max_count = count;

                    positions.clear();
                    positions.push(pos);
                } else if count == max_count {
                    positions.push(pos);
                }
            }
        }
    }

    println!("Found {} positions w count: {}", positions.len(), max_count);

    let mut max_pos = positions.iter()
        .min_by_key(|pos| pos.distance_origin())
        .unwrap()
        .clone();

    println!("Position: {:?}, w count: {}", max_pos, max_count);

    // previous result is not great, try to step towards a better solution...
    let mut steps = -1;

    loop {
        steps += 1;

        let new_pos = Vec3::new(max_pos.x, max_pos.y, max_pos.z - 1);
        let new_count = Nanobot::bots_within_reach_of(&new_pos, &bots);
        if new_count >= max_count {
            max_count = new_count;
            max_pos = new_pos;
            continue;
        }

        let new_pos = Vec3::new(max_pos.x, max_pos.y - 1, max_pos.z);
        let new_count = Nanobot::bots_within_reach_of(&new_pos, &bots);
        if new_count >= max_count {
            max_count = new_count;
            max_pos = new_pos;
            continue;
        }

        let new_pos = Vec3::new(max_pos.x - 1, max_pos.y, max_pos.z);
        let new_count = Nanobot::bots_within_reach_of(&new_pos, &bots);
        if new_count >= max_count {
            max_count = new_count;
            max_pos = new_pos;
            continue;
        }

        break;
    }
    println!("Took {} extra steps", steps);

    println!("Position: {:?}, w count: {}", max_pos, max_count);

    max_pos.distance_origin()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_puzzle_1() {
        let example = include_str!("../example_1.txt");
        let bots = parse_input(&example)
            .expect("Could not parse input");

        assert_eq!(puzzle_1(&bots), 7);
    }

    #[test]
    fn test_puzzle_2() {
        let example = include_str!("../example_2.txt");
        let bots = parse_input(&example)
            .expect("Could not parse input");

        assert_eq!(puzzle_2(&bots), 36);
    }
}
