use std::fmt;
use std::cmp::Ordering;
use util::vec2::Vec2;
use crate::dir::Dir;
use crate::track::Track;

pub type Pos = Vec2<usize>;

#[derive(Debug)]
pub struct Cart {
    pos: Pos,
    dir: Dir,
    has_collided: bool,
    turn_rotaton: usize,
}

impl Cart {
    pub fn parse(c: char, pos: Pos) -> Result<Cart, ()> {
        let dir = Dir::parse(c)?;

        Ok(Cart { pos, dir, has_collided: false, turn_rotaton: 0 })
    }

    pub fn pos(&self) -> Pos {
        self.pos.clone()
    }

    pub fn has_collided(&self) -> bool {
        self.has_collided
    }

    pub fn set_collided(&mut self, collided: bool) {
        self.has_collided = collided;
    }

    pub fn step(&mut self, grid: &Vec<Vec<Track>>) {
        if self.has_collided {
            return;
        }

        // move forward
        match self.dir {
            Dir::Up     => self.pos.y -= 1,
            Dir::Down   => self.pos.y += 1,
            Dir::Left   => self.pos.x -= 1,
            Dir::Right  => self.pos.x += 1,
        }

        // determine new direction
        let opt_dirs = &grid[self.pos.y][self.pos.x].next_dir(&self.dir);

        if opt_dirs.len() == 1 {
            self.dir = opt_dirs[0].clone();
        } else {
            self.dir = opt_dirs[self.turn_rotaton].clone();
            self.turn_rotaton = (self.turn_rotaton + 1) % 3;
        }
    }
}

impl Ord for Cart {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.pos.y, self.pos.x, &self.dir).cmp(&(other.pos.y, other.pos.x, &other.dir))
    }
}

impl PartialOrd for Cart {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Eq for Cart {
}

impl PartialEq for Cart {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos && self.dir == other.dir
    }
}

impl fmt::Display for Cart {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.has_collided() {
            write!(f, "X")
        } else {
            match self.dir {
                Dir::Up     => write!(f, "^"),
                Dir::Down   => write!(f, "v"),
                Dir::Left   => write!(f, ">"),
                Dir::Right  => write!(f, "<"),
            }
        }
    }
}
