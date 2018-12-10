use crate::point::Point;
use crate::vec2::Vec2;

use std::fmt::{Display, Formatter, Result};
use nom::*;
use nom::types::CompleteStr;
use util::nom_utils::parse_i64;

/// A grid holding a collection of points. Supports 'nice' visuals and 'advanced' zooming.
pub struct Grid {
    data: Vec<Point<i64>>
}

impl Grid {
    pub fn from_vec(input: &Vec<&str>) -> Grid {
        let mut data = Vec::new();

        for d in input {
            let point = do_parse!(CompleteStr(d),
                tag!("position=<")      >>
                px: parse_i64           >>
                tag!(",")               >>
                py: parse_i64           >>
                tag!("> velocity=<")    >>
                vx: parse_i64           >>
                tag!(",")               >>
                vy: parse_i64           >>
                tag!(">")               >>

                (Point::new(px, py, vx, vy))
            ).expect("Could not parse Point").1;

            data.push(point);
        }
        Grid { data }
    }

    pub fn step(&mut self, steps: i32) -> i32 {
        for point in &mut self.data {
            point.step(steps);
        }
        steps
    }

    pub fn step_into_visibility(&mut self) -> i32 {
        let mut steps_taken = 0;

        while !self.is_viewable() {
            self.step(1);
            steps_taken += 1;
        }
        steps_taken
    }

    fn lower_bound(&self) -> Vec2<i64> {
        let x = self.data.iter().map(|p| p.pos.x).min().unwrap_or(0);
        let y = self.data.iter().map(|p| p.pos.y).min().unwrap_or(0);

        Vec2::new(x, y)
    }

    fn higher_bound(&self) -> Vec2<i64> {
        let x = self.data.iter().map(|p| p.pos.x).max().unwrap_or(0);
        let y = self.data.iter().map(|p| p.pos.y).max().unwrap_or(0);

        Vec2::new(x, y)
    }

    fn is_viewable(&self) -> bool {
        let low = self.lower_bound();
        let high = self.higher_bound();

        high.x - low.x < 250 && high.y - low.y < 250
    }

    fn contains_point(&self, x: i64, y: i64) -> bool {
        self.data.iter()
            .map(|p| &p.pos)
            .find(|p| **p == (x, y))
            .is_some()
    }

}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "*\n")?;

        if !self.is_viewable() {
            write!(f, "| ... to big to display ...\n")?;
        } else {
            let low = self.lower_bound();
            let high = self.higher_bound();

            for y in low.y..high.y+1 {
                write!(f, "| ")?;

                for x in low.x..high.x+1 {
                    if self.contains_point(x, y) {
                        write!(f, "#")?;
                    } else {
                        write!(f, ".")?;
                    }
                }
                write!(f, "\n")?;
            }
        }
        write!(f, "*")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from_vec() {
        let grid = Grid::from_vec(&vec![
            "position=< 9,  1> velocity=< 0,  2>",
            "position=< 7,  0> velocity=<-1,  0>",
        ]);

        assert_eq!(
            grid.data,
            vec!(Point::new(9, 1, 0, 2), Point::new(7, 0, -1, 0))
        );
    }
}