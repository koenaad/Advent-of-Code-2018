mod node;

use crate::node::Node;

fn main() {
    let input = include_str!("../input.txt")
        .trim();

    println!("Puzzle 1: {}", puzzle_1(&input));
    println!("Puzzle 2: {}", puzzle_2(&input));
}

fn puzzle_1(input: &str) -> i32 {
    let node = Node::parse(input);

    node.metadata_sum()
}

fn puzzle_2(input: &str) -> i32 {
    let node = Node::parse(input);

    node.value()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle_1() {
        let example = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";

        assert_eq!(puzzle_1(&example), 138);
    }

    #[test]
    fn test_puzzle_2() {
        let example = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";

        assert_eq!(puzzle_2(&example), 66);
    }
}
