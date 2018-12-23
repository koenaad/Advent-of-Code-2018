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

    println!("Positions found: {}, count: {}", positions.len(), max_count);

    let shortest_pos = positions.iter()
        .min_by_key(|pos| pos.distance_origin())
        .unwrap()
        .clone();

    println!("Position: {:?}", shortest_pos);

    shortest_pos.distance_origin()
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
