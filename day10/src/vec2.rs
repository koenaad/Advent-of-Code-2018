use std::fmt::{Display, Formatter, Result};

/// A 2 dimensional vector.
#[derive(Debug, Eq, PartialEq)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vec2<T>
    where T: std::marker::Copy + std::ops::AddAssign + std::ops::SubAssign
{
    pub fn new(x: T, y: T) -> Vec2<T> {
        Vec2{ x, y }
    }

    pub fn add(&mut self, other: &Vec2<T>) {
        self.x += other.x;
        self.y += other.y;
    }

    pub fn sub(&mut self, other: &Vec2<T>) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl<T> PartialEq<(T, T)> for Vec2<T> 
    where T: PartialEq
{
    fn eq(&self, other: &(T, T)) -> bool {
        self.x == other.0 && self.y == other.1
    }
}

impl<T> Display for Vec2<T>
    where T: Display
{
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "[{}, {}]", self.x, self.y)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_vec2_i32() {
        let mut v1 = Vec2::new(0, 0);
        let mut v2 = Vec2::new(5, 3);

        v1.add(&v2);
        assert_eq!(v1, (5, 3));

        v1.add(&v2);
        assert_eq!(v1, (10, 6));

        v2.sub(&v1);
        assert_eq!(v2, (-5, -3));

        v2.sub(&v1);
        assert_eq!(v2, (-15, -9));
    }
}
