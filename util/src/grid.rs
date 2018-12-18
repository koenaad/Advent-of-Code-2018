use num_traits::cast::ToPrimitive;
use std::fmt::{Display, Formatter, Result};

pub struct Grid<T> {
    width: usize,
    height: usize,
    data: Vec<T>,
}

impl<T> Grid<T> {
    /// Populate each cell of a grid of `width` by `height` with `value`.
    pub fn populate<F>(width: usize, height: usize, value: F) -> Grid<T>
        where F: Fn(usize, usize) -> T
    {
        let mut data = Vec::with_capacity(width * height);
        for y in 0..height {
            for x in 0..width {
                data.push(value(x, y));
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

    pub fn iter(&self) -> std::slice::Iter<T> {
        self.data.iter()
    }

    /// Return a list of references to all neighbouring cells.
    pub fn get_neighbours(&self, x: usize, y: usize) -> Vec<&T> {
        let mut neighbours = Vec::new();

        let min_y = if y == 0 { 0 } else { y - 1 };
        let max_y = if y == self.height - 1 { self.height - 1 } else { y + 1 };

        let min_x = if x == 0 { 0 } else { x - 1 };
        let max_x = if x == self.width - 1 { self.width - 1 } else { x + 1 };

        for y1 in min_y..max_y+1 {
            for x1 in min_x..max_x+1 {
                if x == x1 && y == y1 {
                    continue;
                }
                neighbours.push(&self.data[(y1 * self.width) + x1]);
            }
        }
        neighbours
    }

    /// Return the `x` and `y` coordinates of the top-left corner of a rectangle
    /// of `width` by `height` with the highest sum of values.
    pub fn find_max_rect(&self, width: usize, height: usize) -> (usize, usize, isize)
        where T: Clone + ToPrimitive
    {
        if width > self.width || height > self.height {
            panic!("Dimensions of square is larger than grid");
        }

        let mut largest_sum = 0;
        let mut largest_pos = (0, 0);

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
                    largest_pos = (x, y);
                }
            }
        }
        (largest_pos.0, largest_pos.1, largest_sum)
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
        let grid = Grid::populate(3, 3, |x, y| (x + y) as i32);

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
        let grid = Grid::populate(3, 3, |x, y| (x + y) as i32);

        grid.get(0, 4);
    }

    #[test]
    fn test_get_neighbours() {
        let input = "123\n456\n789\n";
        let grid = Grid::convert(&input, |c| c.to_digit(10).unwrap());

        let neighbours1 = grid.get_neighbours(0, 2);
        assert_eq!(neighbours1.len(), 3);
        assert!(neighbours1.contains(&&4));
        assert!(neighbours1.contains(&&5));
        assert!(neighbours1.contains(&&8));

        let neighbours2 = grid.get_neighbours(1, 1);
        assert_eq!(neighbours2.len(), 8);
        assert!(!neighbours2.contains(&&5));    // should contain everything, except 5

        let neighbours3 = grid.get_neighbours(2, 1);
        assert_eq!(neighbours3.len(), 5);
        assert!(neighbours3.contains(&&2));
        assert!(neighbours3.contains(&&3));
        assert!(neighbours3.contains(&&5));
        assert!(neighbours3.contains(&&8));
        assert!(neighbours3.contains(&&9));
    }

    #[test]
    fn test_find_max_rect() {
        let grid = Grid::populate(3, 3, |x, y| (x + y) as i32);

        assert_eq!(grid.find_max_rect(1, 1), (2, 2, 4));
        assert_eq!(grid.find_max_rect(1, 2), (2, 1, 7));
        assert_eq!(grid.find_max_rect(2, 2), (1, 1, 12));
        assert_eq!(grid.find_max_rect(1, 3), (2, 0, 9));
    }
}
