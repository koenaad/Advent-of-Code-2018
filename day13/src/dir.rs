#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    pub fn parse(c: char) -> Result<Dir, ()> {
        match c {
            '^' => Ok(Dir::Up),
            'v' => Ok(Dir::Down),
            '<' => Ok(Dir::Left),
            '>' => Ok(Dir::Right),
            _   => Err(()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(Dir::parse('^'), Ok(Dir::Up));
        assert_eq!(Dir::parse('v'), Ok(Dir::Down));
        assert_eq!(Dir::parse('<'), Ok(Dir::Left));
        assert_eq!(Dir::parse('>'), Ok(Dir::Right));

        assert!(Dir::parse('-').is_err());
    }
}
