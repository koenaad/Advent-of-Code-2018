mod cart;
mod dir;
mod track;
mod world;

use crate::world::World;
use util::vec2::Vec2;

fn main() {
    let input = include_str!("../input.txt");

    println!("Puzzle 1: {}", puzzle_1(&input));
    println!("Puzzle 2: {}", puzzle_2(&input));
}

fn puzzle_1(input: &str) -> Vec2<usize> {
    let mut world = World::load(&input);

    println!("{}", world);

    world.find_first_crash().0
}

fn puzzle_2(input: &str) -> Vec2<usize> {
    Vec2::new(0, 0)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_puzzle_1() {
        let example = include_str!("../example.txt");

        assert_eq!(puzzle_1(&example), (7, 3));
    }

    #[test]
    fn test_puzzle_2() {
        let example = include_str!("../example.txt");

        assert_eq!(puzzle_2(&example), (0, 0));
    }
}
