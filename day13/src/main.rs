mod cart;
mod dir;
mod track;
mod world;

use crate::world::World;
use crate::cart::Pos;

fn main() {
    let input = include_str!("../input.txt");

    println!("Puzzle 1: {}", puzzle_1(&input));
    println!("Puzzle 2: {}", puzzle_2(&input));
}

fn puzzle_1(input: &str) -> Pos {
    let mut world = World::load(&input);

    world.tick_until_first_collision()
}

fn puzzle_2(input: &str) -> Pos {
    let mut world = World::load(&input);

    world.tick_until_one_cart_left()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_puzzle_1() {
        let example = include_str!("../example_1.txt");

        assert_eq!(puzzle_1(&example), (7, 3));
    }

    #[test]
    fn test_puzzle_2() {
        let example = include_str!("../example_2.txt");

        assert_eq!(puzzle_2(&example), (6, 4));
    }
}
