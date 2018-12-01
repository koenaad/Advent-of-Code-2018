extern crate structopt;

use std::collections::HashSet;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    /// Starting frequency
    #[structopt(short = "s", long = "start", default_value = "0")]
    start: i32,

    /// Input file with frequency changes
    #[structopt(parse(from_os_str))]
    input_file: PathBuf,
}

fn main() {
    let opt = Opt::from_args();

    let start = opt.start;
    let input = parse_file_as_i32(&opt.input_file)
        .expect("Could not process input file");

    // Puzzle 1
    println!("*** Puzzle 1: end result = {}", puzzle_1(start, &input));

    // Puzzle 2
    println!("*** Puzzle 2: first duplicate frequency = {}", puzzle_2(start, &input));
}

/// returns the resulting frequency after all `changes` are applied to `start`
fn puzzle_1(start: i32, changes: &Vec<i32>) -> i32 {
    start + changes.iter().sum::<i32>()
}

/// continuously applies `changes` to `start` and returns first duplicate frequency
fn puzzle_2(start: i32, changes: &Vec<i32>) -> i32 {
    let mut curr = start;
    let mut prev_freqs: HashSet<i32> = HashSet::new();

    prev_freqs.insert(curr);

    loop {
        for change in changes {
            curr = curr + change;

            if prev_freqs.contains(&curr) {
                return curr;
            }

            prev_freqs.insert(curr);
        }
    }
}

fn parse_file_as_i32(file: &PathBuf) -> Result<Vec<i32>, String> {
    let content = std::fs::read_to_string(&file)
        .map_err(|e| e.to_string())?;

    content.lines()
        .map(|line| line.parse::<i32>().map_err(|e| e.to_string()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn puzzle_1_examples() {
        assert_eq!(puzzle_1(0, &vec![ 1,  1,  1]),  3);
        assert_eq!(puzzle_1(0, &vec![ 1,  1, -2]),  0);
        assert_eq!(puzzle_1(0, &vec![-1, -2, -3]), -6);
    }

    #[test]
    fn puzzle_2_examples() {
        assert_eq!(puzzle_2(0, &vec![ 1, -1]),  0);
        assert_eq!(puzzle_2(0, &vec![ 3,  3,  4, -2, -4]), 10);
        assert_eq!(puzzle_2(0, &vec![-6,  3,  8,  5, -6]),  5);
        assert_eq!(puzzle_2(0, &vec![ 7,  7, -2, -7, -4]), 14);
    }

    #[test]
    fn parse_file_valid() {
        assert_eq!(parse_file_as_i32(&PathBuf::from("valid_input.txt")).unwrap(), vec![45, 3, -3, 3]);
    }

    #[test]
    fn parse_file_invalid() {
        assert!(parse_file_as_i32(&PathBuf::from("invalid_input.txt")).is_err());
    }

    #[test]
    fn parse_file_non_existing() {
        assert!(parse_file_as_i32(&PathBuf::from("does_not_exist.txt")).is_err());
    }
}
