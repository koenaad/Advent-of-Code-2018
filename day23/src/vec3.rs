type Unit = isize;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Vec3 {
    pub x: Unit,
    pub y: Unit,
    pub z: Unit,
}

impl Vec3 {
    pub fn new(x: Unit, y: Unit, z: Unit) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn distance(v1: &Self, v2: &Self) -> usize {
        ((v1.x - v2.x).abs() + (v1.y - v2.y).abs() + (v1.z - v2.z).abs()) as usize
    }

    pub fn distance_origin(&self) -> usize {
        (self.x.abs() + self.y.abs() + self.z.abs()) as usize
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_distance() {
        assert_eq!(Vec3::distance(&Vec3::new(0, 0, 0), &Vec3::new(1, 3, 5)), 9);
        assert_eq!(Vec3::distance(&Vec3::new(3, -10, 7), &Vec3::new(-2, 3, 8)), 19);
    }

    #[test]
    fn test_distance_origin() {
        assert_eq!(Vec3::new(0, 0, 0).distance_origin(), 0);
        assert_eq!(Vec3::new(3, -10, 7).distance_origin(), 20);
    }
}
