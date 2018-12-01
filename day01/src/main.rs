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
    let input = parse_file_as_i32(&opt.input_file);

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

/// panics if there is an issue reading or parsing `file`
fn parse_file_as_i32(file: &PathBuf) -> Vec<i32> {
    let content = std::fs::read_to_string(&file)
        .expect("Could not read file");

    content.lines()
        .map(|line| {
            line.parse()
                .expect("Could not parse line")
        })
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
        assert_eq!(parse_file_as_i32(&PathBuf::from("valid_input.txt")), vec![45, 3, -3, 3]);
    }

    #[test]
    #[should_panic]
    fn parse_file_invalid() {
        parse_file_as_i32(&PathBuf::from("invalid_input.txt"));
    }

    #[test]
    #[should_panic]
    fn parse_file_non_existing() {
        parse_file_as_i32(&PathBuf::from("does_not_exist.txt"));
    }
}
