use std::fmt;

#[derive(Debug, Eq, PartialEq)]
pub enum Track {
    Hor,
    Vert,
    Cross,
    DiaTL_BR,
    DiaTR_BL,
    Empty,
    ToDo,   // can only be set manually
}

impl Track {
    pub fn parse(c: char) -> Result<Track, ()> {
        match c {
            '-' => Ok(Track::Hor),
            '|' => Ok(Track::Vert),
            '+' => Ok(Track::Cross),
            '\\'=> Ok(Track::DiaTL_BR),
            '/' => Ok(Track::DiaTR_BL),
            ' ' => Ok(Track::Empty),
            _   => Err(()),
        }
    }

    pub fn has_connection_top(&self) -> bool {        
        match self {
            Track::Hor      => false,
            Track::Vert     => true,
            Track::Cross    => true,
            Track::DiaTL_BR => true,
            Track::DiaTR_BL => true,
            Track::Empty    => false,
            Track::ToDo     => panic!("AAA"),
        }
    }

    pub fn has_connection_bottom(&self) -> bool {        
        match self {
            Track::Hor      => false,
            Track::Vert     => true,
            Track::Cross    => true,
            Track::DiaTL_BR => true,
            Track::DiaTR_BL => true,
            Track::Empty    => false,
            Track::ToDo     => panic!("AAA"),
        }
    }

    pub fn has_connection_left(&self) -> bool {        
        match self {
            Track::Hor      => true,
            Track::Vert     => false,
            Track::Cross    => true,
            Track::DiaTL_BR => true,
            Track::DiaTR_BL => true,
            Track::Empty    => false,
            Track::ToDo     => panic!("AAA"),
        }
    }

    pub fn has_connection_right(&self) -> bool {        
        match self {
            Track::Hor      => true,
            Track::Vert     => false,
            Track::Cross    => true,
            Track::DiaTL_BR => true,
            Track::DiaTR_BL => true,
            Track::Empty    => false,
            Track::ToDo     => panic!("AAA"),
        }
    }
}

impl fmt::Display for Track {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Track::Hor      => write!(f, "-"),
            Track::Vert     => write!(f, "|"),
            Track::Cross    => write!(f, "+"),
            Track::DiaTL_BR => write!(f, "\\"),
            Track::DiaTR_BL => write!(f, "/"),
            Track::Empty    => write!(f, " "),
            Track::ToDo     => write!(f, "?"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(Track::parse('-'), Ok(Track::Hor));
        assert_eq!(Track::parse('|'), Ok(Track::Vert));
        assert_eq!(Track::parse('+'), Ok(Track::Cross));
        assert_eq!(Track::parse('\\'), Ok(Track::DiaTL_BR));
        assert_eq!(Track::parse('/'), Ok(Track::DiaTR_BL));
        assert_eq!(Track::parse(' '), Ok(Track::Empty));

        assert!(Track::parse('v').is_err());
    }
}
