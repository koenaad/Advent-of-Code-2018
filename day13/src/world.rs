use std::fmt;
use util::vec2::Vec2;

use crate::cart::*;
use crate::track::*;

pub struct World {
    grid: Vec<Vec<Track>>,
    carts: Vec<Cart>,
    ticks: i32,
}

impl World {
    pub fn load(input: &str) -> World {
        let mut grid = Vec::new();
        let mut carts = Vec::new();

        for (y, l) in input.lines().enumerate() {
            let mut row = Vec::new();

            for (x, c) in l.chars().enumerate() {
                if let Ok(cart) = Cart::parse(c, Pos::new(x, y)) {
                    carts.push(cart);

                    // can't determine underlying track at this moment
                    row.push(Track::ToDo);
                } else {
                    row.push(Track::parse(c).expect("Could not parse Track"));
                }
            }
            grid.push(row);
        }

        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                if grid[y][x] != Track::ToDo {
                    continue;
                }

                let mut conn_top = false;
                if y > 0 {
                    conn_top = grid[y-1][x].has_connection_bottom();
                }

                let mut conn_bot = false;
                if y < grid.len() - 1 {
                    conn_bot = grid[y+1][x].has_connection_top();
                }

                let mut conn_left = false;
                if x > 0 {
                    conn_left = grid[y][x-1].has_connection_left();
                }

                let mut conn_right = false;
                if x < grid[y].len() - 1 {
                    conn_right = grid[y][x+1].has_connection_right();
                }

                // hmm... 🤔
                if !conn_top && !conn_bot && conn_left && conn_right {
                    grid[y][x] = Track::Hor;
                } else if conn_top && conn_bot && !conn_left && !conn_right {
                    grid[y][x] = Track::Vert;
                } else if conn_top && conn_bot && conn_left && conn_right {
                    grid[y][x] = Track::Cross;
                } else if (conn_top && !conn_bot && !conn_left && conn_right)
                    || (!conn_top && conn_bot && conn_left && !conn_right) {
                    grid[y][x] = Track::DiaTL_BR;
                } else if (conn_top && !conn_bot && conn_left && !conn_right)
                    || (!conn_top && conn_bot && conn_left && !conn_right) {
                    grid[y][x] = Track::DiaTR_BL;
                }
            }
        }

        World { grid, carts, ticks: 0 }
    }

    pub fn tick(&mut self) {
    }

    pub fn find_first_crash(&mut self) -> (Vec2<usize>, i32) {
        (Vec2::new(0, 0), 0)
    }
}

impl fmt::Display for World {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\n")?;
        for row in &self.grid {
            for t in row {
                write!(f, "{}", t)?;
            }
            write!(f, "\n")?;
        }
        write!(f, "\n")?;
        write!(f, "Carts: {:?}", self.carts)?;
        write!(f, "\n")
    }
}
