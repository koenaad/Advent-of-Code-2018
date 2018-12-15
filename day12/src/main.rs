mod rule;
mod row;

use std::time::Instant;
use crate::row::Row;
use crate::rule::Rule;

fn parse_input(input: &str) -> (Row, Vec<Rule>) {
    let lines: Vec<&str> = input.lines().collect();

    let initial_state = lines.get(0)
        .expect("Could not parse initial state")
        .get(15..)
        .expect("Could not parse initial state");

    let row = Row::new(initial_state);

    let rules = lines.iter()
        .skip(2)
        .map(|line| Rule::parse(line))
        .filter(|rule| rule.result == true) // filter out no-op rules
        .collect();

    (row, rules)
}

fn main() {
    let input = include_str!("../input.txt")
        .trim();

    println!("Puzzle 1: {}", puzzle_1(&input));
    println!("Puzzle 2: {}", puzzle_2(&input));
}

fn puzzle_1(input: &str) -> isize {
    let (mut row, rules) = parse_input(&input);

    for i in 1..20+1 {
        row = row.next_generation(&rules);
    }
    row.sum_of_pots_alive()
}

fn puzzle_2(input: &str) -> isize {
    let (mut row, rules) = parse_input(&input);

    let start = Instant::now();

    for i in 0..50_000_000_000usize {
        row = row.next_generation(&rules);

        if i % 10_000 == 0 {
            println!("{} sec: at {} generations", start.elapsed().as_secs(), i);
        }
    }

    println!("Computation time: {} secs", start.elapsed().as_secs());

    row.sum_of_pots_alive()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle_1() {
        let example = include_str!("../example.txt")
            .trim();

        assert_eq!(puzzle_1(&example), 325);
    }
}
