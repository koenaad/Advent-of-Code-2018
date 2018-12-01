extern crate structopt;

use std::collections::HashSet;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    /// Starting frequency
    start: i32,

    /// Input file with changes
    #[structopt(parse(from_os_str))]
    input_file: std::path::PathBuf,
}

fn main() {
    let opt = Opt::from_args();

    let start = opt.start;
    let input = parse_file(&opt.input_file);

    // Puzzle 1
    let result = start + input.iter().sum::<i32>();

    println!("*** Puzzle 1: end result = {}", result);

    // Puzzle 2
    let mut curr = opt.start;
    let mut prev_freqs: HashSet<i32> = HashSet::new();
    let mut found = false;

    prev_freqs.insert(curr);

    while !found {
        for num in &input {
            curr = curr + num;

            if prev_freqs.contains(&curr) {
                found = true;
                break;
            }

            prev_freqs.insert(curr);
        }
    }
    println!("*** Puzzle 2: first duplicate frequency = {}", curr);
}

/// panics if there is an issue reading or parsing `input_file`
fn parse_file(input_file: &std::path::PathBuf) -> Vec<i32> {
    let content = std::fs::read_to_string(&input_file)
        .expect("Could not read file");

    content.lines()
        .map(|line| {
            line.parse()
                .expect("Could not parse line")
        })
        .collect()
}
