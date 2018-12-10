use crate::vec2::Vec2;

/// A representation of a point with a position and a constant velocity.
#[derive(Debug, Eq, PartialEq)]
pub struct Point<T> {
    pub pos: Vec2<T>,
    vel: Vec2<T>,
}

impl<T> Point<T>
    where T: std::marker::Copy + std::ops::AddAssign + std::ops::SubAssign
{
    pub fn new(px: T, py: T, vx: T, vy: T) -> Point<T> {
        Point { pos: Vec2::new(px, py), vel: Vec2::new(vx, vy) }
    }

    /// Update the position of this point by one 'step'.
    pub fn step(&mut self, steps: i32) {
        if steps > 0 {
            for _ in 0..steps {
                self.pos.add(&self.vel);
            }
        } else {
            for _ in 0..steps.abs() {
                self.pos.sub(&self.vel);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_point_i32_step() {
        let mut p1 = Point::new(0, 0, 1, 5);

        p1.step(1);
        assert_eq!(p1.pos, (1, 5));

        p1.step(1);
        assert_eq!(p1.pos, (2, 10));

        p1.step(3);
        assert_eq!(p1.pos, (5, 25));

        p1.step(-1);
        assert_eq!(p1.pos, (4, 20));
    }
}
