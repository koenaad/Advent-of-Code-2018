mod grid;
mod point;
mod vec2;

use std::io;
use crate::grid::Grid;

use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    /// Input file with the points.
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

fn main() {
    let opt = Opt::from_args();

    let content = std::fs::read_to_string(&opt.input)
        .expect("Could not read input file");

    let input = content.lines()
        .filter(|line| line.len() > 0)
        .collect();

    let mut grid = Grid::from_vec(&input);
    let mut steps = 0;

    loop {
        println!("");
        println!("{}", grid);
        println!("");
        println!("{} steps from initial state", steps);
        println!("");
        println!("- #\tStep #");
        println!("- s\tStep into visibility");
        println!("- q\tQuit");
        println!("");

        let mut input = String::new();

        io::stdin().read_line(&mut input)
            .expect("Could not read input");

        if let Ok(n) = input.trim().parse::<i32>() {
            println!("Stepping {}", n);
            steps += grid.step(n);
            continue;
        }
        if input.contains("s") {
            println!("ENHANCE!");
            steps += grid.step_into_visibility();
            continue;
        }
        if input.contains("q") {
            println!("Bye");
            break;
        }
    }
}
