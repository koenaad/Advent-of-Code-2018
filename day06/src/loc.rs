use crate::pos::Pos;

pub type CoordId = i32;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Loc {
    pub pos: Pos,
    pub state: State,
    pub is_edge: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum State {
    Uninitialized,
    NearestTo(CoordId, i32),
    EqualDistance(i32),
    Coord(CoordId),
}

impl Loc {
    pub fn new(pos: Pos) -> Loc {
        Loc { pos, state: State::Uninitialized, is_edge: false, }
    }

    pub fn new_coord(pos: Pos, id: CoordId) -> Loc {
        Loc { pos, state: State::Coord(id), is_edge: false, }
    }

    pub fn new_edge(pos: Pos) -> Loc {
        Loc { pos, state: State::Uninitialized, is_edge: true, }
    }

    /// Calculate the manhattan distance between these two Locs.
    pub fn dist_to(&self, other: &Loc) -> i32 {
        self.pos.dist_to(&other.pos)
    }

    /// Iterate through `coords` and update the state.
    pub fn update_state(&mut self, coords: &Vec<Loc>) {
        // TODO: return immediately if self is of type Coord

        for coord in coords {
            self.update_state_with(&coord);
        }
    }

    fn update_state_with(&mut self, coord: &Loc) {
        let coord_id = match coord.state {
            State::Coord(id) => id,
            _               => panic!("update_state_with is not a Coord"),
        };
        let coord_dist = self.dist_to(coord);

        if coord_dist == 0 {
            self.state = State::Coord(coord_id);    // Look at me, I'm the Coord now
            return;
        }

        match self.state {
            State::Uninitialized         => {
                self.state = State::NearestTo(coord_id, coord_dist);
            }
            State::NearestTo(_, dist)   => {
                if coord_dist < dist {
                    self.state = State::NearestTo(coord_id, coord_dist);
                } else if coord_dist == dist {
                    self.state = State::EqualDistance(coord_dist);
                }
            },
            State::EqualDistance(dist)   => {
                if coord_dist < dist {
                    self.state = State::NearestTo(coord_id, coord_dist);
                }
            }
            State::Coord(_)             => {},
        }
    }

    /// Return the `CoordId` this Loc is nearest to or is, returns None if at equal distance to multiple.
    pub fn part_of(&self) -> Option<CoordId> {
        match self.state {
            State::Coord(id)         => Some(id),
            State::NearestTo(id, _)  => Some(id),
            State::EqualDistance(_)  => None,
            State::Uninitialized     => panic!("Loc is still uninitialized"),
        }
    }

    /// Returns whether this Loc is nearest to or is `CoordId`.
    pub fn is_part_of(&self, coord_id: CoordId) -> bool {
        self.part_of() == Some(coord_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let loc1 = Loc::new(Pos::new(1, 3));

        assert_eq!(loc1.pos, Pos::new(1, 3));
        assert_eq!(loc1.state, State::Uninitialized);
        assert_eq!(loc1.is_edge, false);

        let loc2 = Loc::new_coord(Pos::new(-1, 4), 5);

        assert_eq!(loc2.pos, Pos::new(-1, 4));
        assert_eq!(loc2.state, State::Coord(5));
        assert_eq!(loc2.is_edge, false);

        let loc3 = Loc::new_edge(Pos::new(3, 3));

        assert_eq!(loc3.pos, Pos::new(3, 3));
        assert_eq!(loc3.state, State::Uninitialized);
        assert_eq!(loc3.is_edge, true);
    }

    #[test]
    fn test_update_state_with() {
        let mut loc1 = Loc::new(Pos::new(0, 0));
        assert_eq!(&loc1.state, &State::Uninitialized);

        loc1.update_state_with(&Loc::new_coord(Pos::new(5, 5), 5));
        assert_eq!(loc1.state, State::NearestTo(5, 10));

        loc1.update_state_with(&Loc::new_coord(Pos::new(3, 7), 6));
        assert_eq!(loc1.state, State::EqualDistance(10));

        loc1.update_state_with(&Loc::new_coord(Pos::new(3, 3), 7));
        assert_eq!(loc1.state, State::NearestTo(7, 6));

        loc1.update_state_with(&Loc::new_coord(Pos::new(0, 0), 7));
        assert_eq!(loc1.state, State::Coord(7));
    }

    #[test]
    #[should_panic]
    fn test_update_state_with_not_coord() {
        let mut loc1 = Loc::new(Pos::new(0, 0));
        let loc2 = Loc { pos: Pos::new(5, 5), state: State::NearestTo(1, 10), is_edge: false };

        loc1.update_state_with(&loc2);
    }

    #[test]
    fn test_update_state_with_on_coord() {
        let mut loc1 = Loc::new_coord(Pos::new(0, 0), 5);

        // nothing should happen, self is a Coord
        loc1.update_state_with(&Loc::new_coord(Pos::new(5, 5), 10));
        assert_eq!(loc1.state, State::Coord(5));
    }

    #[test]
    fn test_part_of() {
        let loc1 = Loc { pos: Pos::new(0, 0), state: State::Coord(1), is_edge: false };
        let loc2 = Loc { pos: Pos::new(0, 0), state: State::NearestTo(2, 10), is_edge: false };
        let loc3 = Loc { pos: Pos::new(0, 0), state: State::EqualDistance(10), is_edge: false };
        let loc4 = Loc { pos: Pos::new(0, 0), state: State::NearestTo(3, 5), is_edge: true };

        assert_eq!(loc1.part_of(), Some(1));
        assert_eq!(loc2.part_of(), Some(2));
        assert_eq!(loc3.part_of(), None);
        assert_eq!(loc4.part_of(), Some(3));
    }

    #[test]
    #[should_panic]
    fn test_part_of_uninitialized() {
        let loc1 = Loc::new(Pos::new(0, 0));

        loc1.part_of();
    }

    #[test]
    fn test_is_part_of() {
        let loc1 = Loc { pos: Pos::new(0, 0), state: State::Coord(1), is_edge: false };
        let loc2 = Loc { pos: Pos::new(0, 0), state: State::NearestTo(2, 10), is_edge: false };
        let loc3 = Loc { pos: Pos::new(0, 0), state: State::EqualDistance(10), is_edge: false };
        let loc4 = Loc { pos: Pos::new(0, 0), state: State::NearestTo(3, 5), is_edge: true };

        assert_eq!(loc1.is_part_of(1), true);
        assert_eq!(loc2.is_part_of(1), false);
        assert_eq!(loc3.is_part_of(1), false);
        assert_eq!(loc4.is_part_of(3), true);
    }

    #[test]
    #[should_panic]
    fn test_is_part_of_uninitialized() {
        let loc1 = Loc::new(Pos::new(0, 0));

        loc1.is_part_of(1);
    }
}
