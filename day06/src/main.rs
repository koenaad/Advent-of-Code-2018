mod pos;
mod loc;
mod grid;

use crate::pos::Pos;
use crate::loc::{Loc, State};
use crate::grid::Grid;

fn get_input() -> Vec<&'static str> {
    include_str!("../input.txt")
        .split("\n")
        .filter(|s| s.len() > 0)
        .collect()
}

fn main() {
    let input = get_input();

    println!("Puzzle 1: {}", puzzle_1(&input));
    println!("Puzzle 2: {}", puzzle_2(&input, 10_000));
}

fn get_coords(input: &Vec<&str>) -> Vec<Loc> {
    input.iter()
        .enumerate()
        .map(|(i, s)| Loc::new_coord(Pos::parse(s), i as i32))
        .collect()
}

fn puzzle_1(input: &Vec<&str>) -> i32 {
    let coords = get_coords(&input);

    let grid = Grid::new_with(&coords);

    coords.iter()
        .filter_map(|loc| {
            match loc.state {
                State::Coord(id) => Some(id),
                _ => None,
            }
        })
        .map(|id| grid.get_locs_part_of(id))
        .filter(|locs| {
            locs.iter()
                .find(|loc| loc.is_edge)
                .is_none()
        })
        .map(|locs| locs.len())
        .max()
        .unwrap() as i32
}

fn puzzle_2(input: &Vec<&str>, max_dist: i32) -> i32 {
    let coords = get_coords(&input);

    let grid = Grid::new_with(&coords);

    grid.locations()
        .map(|loc| {
            coords.iter()
                .map(|coord| loc.dist_to(&coord))
                .sum()
        })
        .filter(|total_dist: &i32| *total_dist < max_dist)
        .count() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle_1() {
        let mut example = Vec::new();
        example.push("1, 1");
        example.push("1, 6");
        example.push("8, 3");
        example.push("3, 4");
        example.push("5, 5");
        example.push("8, 9");

        assert_eq!(puzzle_1(&example), 17);
    }

    #[test]
    fn test_puzzle_2() {
        let mut example = Vec::new();
        example.push("1, 1");
        example.push("1, 6");
        example.push("8, 3");
        example.push("3, 4");
        example.push("5, 5");
        example.push("8, 9");

        assert_eq!(puzzle_2(&example, 32), 16);
    }
}
