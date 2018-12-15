use std::fmt;
use util::vec2::Vec2;
use crate::dir::Dir;

pub type Pos = Vec2<usize>;

#[derive(Debug, Eq, PartialEq)]
pub struct Cart {
    pos: Pos,
    dir: Dir,
}

impl Cart {
    pub fn parse(c: char, pos: Pos) -> Result<Cart, ()> {
        let dir = Dir::parse(c)?;

        Ok(Cart { pos, dir })
    }
}

impl fmt::Display for Cart {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.dir {
            Dir::Up     => write!(f, "^"),
            Dir::Down   => write!(f, "v"),
            Dir::Left   => write!(f, ">"),
            Dir::Right  => write!(f, "<"),
        }
    }
}
