use crate::vec2::Vec2;

use num_traits::cast::ToPrimitive;
use std::fmt::{Display, Formatter, Result};

pub type Pos = Vec2<usize>;

pub struct Grid<T> {
    width: usize,
    height: usize,
    data: Vec<T>,
}

impl<T> Grid<T> {
    /// Populate each cell of a grid of `width` by `height` with `value`.
    pub fn populate<F>(width: usize, height: usize, value: F) -> Grid<T>
        where F: Fn(Pos) -> T
    {
        let mut data = Vec::with_capacity(width * height);
        for y in 0..height {
            for x in 0..width {
                data.push(value(Pos::new(x, y)));
            }
        }
        Grid { width, height, data }
    }

    /// Convert a string, consisting of multiple lines, to a grid using `parse`.
    pub fn convert<F>(input: &str, parse: F) -> Grid<T>
        where F: Fn(char) -> T
    {
        let lines: Vec<&str> = input.lines().collect();

        if lines.is_empty() {
            panic!("Input contains no lines");
        }
        let height = lines.len();

        let min_width = lines.iter().map(|line| line.len()).min().unwrap();
        let max_width = lines.iter().map(|line| line.len()).max().unwrap();

        if min_width != max_width {
            panic!("Not all lines have the same length");
        }
        let width = max_width;

        let data = lines.iter()
            .flat_map(|line| line.chars())
            .map(|c| parse(c))
            .collect();

        Grid { width, height, data }
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    /// Panics when `x` or `y` is out of bounds.
    pub fn get(&self, x: usize, y: usize) -> &T {
        if x > self.width || y > self.height {
            panic!("Value out of bounds");
        }
        &self.data[(y * self.width) + x]
    }

    /// Panics when `x` or `y` is out of bounds.
    pub fn get_value(&self, x: usize, y: usize) -> T
        where T: Clone
    {
        if x > self.width || y > self.height {
            panic!("Value out of bounds");
        }
        self.data[(y * self.width) + x].clone()
    }

    /// Return the top-left corner of a rectangle of `width` by `height` with the highest sum of values.
    pub fn find_max_rect(&self, width: usize, height: usize) -> (Pos, isize)
        where T: Clone + ToPrimitive
    {
        if width > self.width || height > self.height {
            panic!("Dimensions of square is larger than grid");
        }

        let mut largest_sum = 0;
        let mut largest_pos = Vec2::new(0, 0);

        for y in 0..(self.height + 1 - height) {
            for x in 0..(self.width + 1 - width) {

                let mut sum: isize = 0;

                for y2 in y..(y + height) {
                    for x2 in x..(x+ width) {
                        sum += self.get_value(x2, y2).to_isize().unwrap();
                    }
                }
                
                if sum > largest_sum {
                    largest_sum = sum;
                    largest_pos = Vec2::new(x, y);
                }
            }
        }
        (largest_pos, largest_sum)
    }
}

impl<T> Display for Grid<T>
    where T: Display
{
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "*\n")?;

        for y in 0..self.height {
            write!(f, "| ")?;

            for x in 0..self.width {
                write!(f, "{} ", self.get(x, y))?;
            }
            write!(f, "\n")?;
        }
        write!(f, "*")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_populate() {
        let grid = Grid::populate(3, 3, |pos| (pos.x + pos.y) as i32);

        assert_eq!(*grid.get(0, 0), 0);
        assert_eq!(*grid.get(1, 0), 1);
        assert_eq!(*grid.get(2, 0), 2);
        assert_eq!(*grid.get(0, 1), 1);
        assert_eq!(*grid.get(1, 1), 2);
        assert_eq!(*grid.get(2, 1), 3);
        assert_eq!(*grid.get(0, 2), 2);
        assert_eq!(*grid.get(1, 2), 3);
        assert_eq!(*grid.get(2, 2), 4);
    }

    #[test]
    fn test_convert() {
        let input = "123\n456\n789\n";
        let grid = Grid::convert(&input, |c| c.to_digit(10).unwrap());

        assert_eq!(grid.get_width(), 3);
        assert_eq!(grid.get_height(), 3);
        assert_eq!(*grid.get(0, 0), 1);
        assert_eq!(*grid.get(1, 0), 2);
        assert_eq!(*grid.get(2, 0), 3);
        assert_eq!(*grid.get(0, 1), 4);
        assert_eq!(*grid.get(1, 1), 5);
        assert_eq!(*grid.get(2, 1), 6);
        assert_eq!(*grid.get(0, 2), 7);
        assert_eq!(*grid.get(1, 2), 8);
        assert_eq!(*grid.get(2, 2), 9);
    }

    #[test]
    #[should_panic]
    fn test_get_out_of_bounds() {
        let grid = Grid::populate(3, 3, |pos| (pos.x + pos.y) as i32);

        grid.get(0, 4);
    }

    #[test]
    fn test_find_max_rect() {
        let grid = Grid::populate(3, 3, |pos| (pos.x + pos.y) as i32);

        assert_eq!(grid.find_max_rect(1, 1), (Pos::new(2, 2), 4));
        assert_eq!(grid.find_max_rect(1, 2), (Pos::new(2, 1), 7));
        assert_eq!(grid.find_max_rect(2, 2), (Pos::new(1, 1), 12));
        assert_eq!(grid.find_max_rect(1, 3), (Pos::new(2, 0), 9));
    }
}
