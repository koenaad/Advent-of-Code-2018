type Unit = isize;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Vec4 {
    pub x: Unit,
    pub y: Unit,
    pub z: Unit,
    pub t: Unit,
}

impl Vec4 {
    pub fn new(x: Unit, y: Unit, z: Unit, t: Unit) -> Self {
        Self { x, y, z, t }
    }

    pub fn parse(input: &str) -> Result<Self, &'static str> {
        let mut values = Vec::new();

        for value in input.split(",") {
            if let Ok(value) = value.parse() {
                values.push(value);
            } else {
                return Err("could not parse value");
            }
        }

        if values.len() != 4 {
            return Err("expected exactly 4 values");
        }

        Ok(Self { x: values[0], y: values[1], z: values[2], t: values[3] })
    }

    pub fn distance(v1: &Self, v2: &Self) -> usize {
        ((v1.x - v2.x).abs() + (v1.y - v2.y).abs() + (v1.z - v2.z).abs() + (v1.t - v2.t).abs()) as usize
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(Vec4::parse("100,-103,0,13"), Ok(Vec4::new(100, -103, 0, 13)));

        assert!(Vec4::parse("-1,1,0").is_err());
        assert!(Vec4::parse("-1,1,0,5,3").is_err());
    }

    #[test]
    fn test_distance() {
        assert_eq!(Vec4::distance(&Vec4::new(0, 0, 0, 0), &Vec4::new(1, 3, 5, 7)), 16);
        assert_eq!(Vec4::distance(&Vec4::new(3, -10, 7, 2), &Vec4::new(-2, 3, 8, 1)), 20);
    }

}
