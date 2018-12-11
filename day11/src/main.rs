use std::time::Instant;
use rayon::prelude::*;
use util::grid::Grid;

const INPUT: isize = 8141;

fn main() {
    println!("Puzzle 1: {:?}", puzzle_1(INPUT));
    println!("Puzzle 2: {:?}", puzzle_2(INPUT));
}

fn create_power_grid(serial_number: isize) -> Grid<isize> {
    Grid::populate(300, 300, |pos| {
        let rack_id = (pos.x as isize) + 10;
        let mut power_level = rack_id * (pos.y as isize);
        power_level += serial_number;
        power_level *= rack_id;
        power_level = power_level % 1000 / 100;
        power_level -= 5;
        power_level
    })
}

fn puzzle_1(serial_number: isize) -> (usize, usize) {
    let grid = create_power_grid(serial_number);

    let max = grid.find_max_rect(3, 3).0;

    (max.x, max.y)
}

fn puzzle_2(serial_number: isize) -> (usize, usize, usize) {
    let grid = create_power_grid(serial_number);

    let start = Instant::now();

    let (pos, _, size) = (1..300usize)
        .into_par_iter()
        // .inspect(|i| println!("{}: start {}", start.elapsed().as_secs(), i))
        .map(|size| {
            let (pos, total_power) = grid.find_max_rect(size, size);
            (pos, total_power, size)
        })
        // .inspect(|(_, _, size)| println!("{}: {} done", start.elapsed().as_secs(), size))
        .max_by_key(|(_, total_power, _)| *total_power)
        .unwrap();

    println!("Computation time: {} secs", start.elapsed().as_secs());

    (pos.x, pos.y, size)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle_1() {
        assert_eq!(puzzle_1(18), (33, 45));
        assert_eq!(puzzle_1(42), (21, 61));
    }

    #[test]
    fn test_puzzle_2() {
        assert_eq!(puzzle_2(18), ( 90, 269, 16));
        assert_eq!(puzzle_2(42), (232, 251, 12));
    }
}
