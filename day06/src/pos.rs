#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
}

impl Pos {
    pub fn new(x: i32, y: i32) -> Pos {
        Pos { x, y }
    }

    pub fn parse(string: &str) -> Pos {
        let xy: Vec<i32> = string.to_string()
            .split(", ")
            .map(|s| s.parse()
                .expect("Could not parse Pos")
            )
            .collect();

        let x = *xy.get(0).unwrap();
        let y = *xy.get(1).unwrap();

        Pos::new(x, y)
    }

    /// Calculate the manhattan distance between these two positions.
    pub fn dist_to(&self, other: &Pos) -> i32 {
        let diff_x = self.x - other.x;
        let diff_y = self.y - other.y;

        diff_x.abs() + diff_y.abs()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(Pos::parse("12, 31"), Pos::new(12, 31));
        assert_eq!(Pos::parse("-5, -1"), Pos::new(-5, -1));
        assert_eq!(Pos::parse("103, -7"), Pos::new(103, -7));
    }

    #[test]
    fn test_dist_to() {
        let pos1 = Pos::new(0, 0);
        let pos2 = Pos::new(5, 5);
        let pos3 = Pos::new(3, -1);

        assert_eq!(pos1.dist_to(&pos2), 10);
        assert_eq!(pos2.dist_to(&pos1), 10);
        assert_eq!(pos1.dist_to(&pos3), 4);
        assert_eq!(pos2.dist_to(&pos3), 8);
    }
}
