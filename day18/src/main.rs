use util::grid::Grid;
use std::time::Instant;

const OPEN_GROUND: char = '.';
const TREES: char = '|';
const LUMBERYARD: char = '#';

fn parse_input(input: &str) -> Grid<char> {
    Grid::convert(&input, |c| c)
}

fn next_spawn(curr: &char, neighbours: Vec<&char>) -> char {
    match curr {
        &OPEN_GROUND    => {
            if neighbours.iter().filter(|acre| ***acre == TREES).count() >= 3 {
                TREES
            } else {
                OPEN_GROUND
            }
        }
        &TREES          => {
            if neighbours.iter().filter(|acre| ***acre == LUMBERYARD).count() >= 3 {
                LUMBERYARD
            } else {
                TREES
            }
        },
        &LUMBERYARD     => {
            if neighbours.contains(&&TREES) && neighbours.contains(&&LUMBERYARD) {
                LUMBERYARD
            } else {
                OPEN_GROUND
            }
        }
        _               => {
            panic!("Not a valid acre type");
        }
    }
}

fn evolve(grid: &Grid<char>) -> Grid<char> {
    let populate_fn =  move |x, y| {
        next_spawn(grid.get(x, y), grid.get_neighbours(x, y))
    };

    Grid::populate(grid.get_width(), grid.get_height(), populate_fn)
}

fn evolve_times(grid: &Grid<char>, times: usize) -> Grid<char> {
    let mut new_grid = evolve(&grid);

    for _ in 0..times - 1 {
        new_grid = evolve(&new_grid);
    }

    new_grid
}

fn resource_value(grid: &Grid<char>) -> usize {
    let num_trees = grid.iter()
        .filter(|acre| **acre == TREES)
        .count();

    let num_lumberyard = grid.iter()
        .filter(|acre| **acre == LUMBERYARD)
        .count();

    num_trees * num_lumberyard
}

fn main() {
    let input = include_str!("../input.txt");

    let start = Instant::now();

    let mut grid = parse_input(&input);
    let mut steps = 0;

    grid = evolve_times(&grid, 10);
    steps += 10;

    println!("Puzzle 1: after 10 minutes: {}", resource_value(&grid));

    // align to 10.000
    grid = evolve_times(&grid, 10_000 - steps);
    steps = 10_000;

    while steps < 1_000_000_000usize {
        grid = evolve_times(&grid, 10_000);
        steps += 10_000;

        println!("{} seconds, {} steps", start.elapsed().as_secs(), steps);
    }

    println!("Puzzle 2: after 1000s years: {}", resource_value(&grid));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle_1() {
        let example = include_str!("../example.txt");
        let grid = parse_input(&example);

        let grid_10_minutes = evolve_times(&grid, 10);

        assert_eq!(resource_value(&grid_10_minutes), 1147);
    }

    #[test]
    #[ignore]
    fn step() {
        let input = include_str!("../input.txt");
        let grid = parse_input(&input);

        let mut new_grid = grid;
        let mut steps = 0;

        println!("Minute: {}", steps);
        println!("{}", new_grid);
        println!("");

        loop {
            new_grid = evolve(&new_grid);
            steps += 1;

            println!("Minute: {} - value: {}", steps, resource_value(&new_grid));
            println!("{}", new_grid);
            println!("");

            let mut input = String::new();

            std::io::stdin().read_line(&mut input)
                .expect("Could not read input");

            if input.contains("q") {
                println!("Bye");
                break;
            }
        }        
    }
}
