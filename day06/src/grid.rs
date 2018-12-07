use crate::pos::Pos;
use crate::loc::{Loc, CoordId};
use std::collections::HashMap;
use std::collections::hash_map::Values;

pub struct Grid {
    grid: HashMap<Pos, Loc>,
}

impl Grid {
    fn insert(&mut self, loc: Loc) {
        // TODO: the clone on pos could probably be avoided by doing some fancy lifetime stuff
        self.grid.insert(loc.pos.clone(), loc);
    }

    fn insert_if_empty(&mut self, loc: Loc) {
        // TODO: the clone on pos could probably be avoided by doing some fancy lifetime stuff
        self.grid.entry(loc.pos.clone()).or_insert(loc);
    }

    pub fn new_with(coords: &Vec<Loc>) -> Grid {
        let min_x = coords.iter().map(|loc| loc.pos.x).min().unwrap();
        let max_x = coords.iter().map(|loc| loc.pos.x).max().unwrap();
        let min_y = coords.iter().map(|loc| loc.pos.y).min().unwrap();
        let max_y = coords.iter().map(|loc| loc.pos.y).max().unwrap();

        let mut grid = Grid { grid: HashMap::new() };

        // insert all Coord Loc
        for coord in coords {
            grid.insert(coord.clone());
        }

        // insert all is_edge Loc
        for x in min_x..max_x {
            grid.insert(Loc::new_edge(Pos::new(x, min_y)));
            grid.insert(Loc::new_edge(Pos::new(x, max_y)));
        }
        for y in min_y..max_y {
            grid.insert(Loc::new_edge(Pos::new(min_x, y)));
            grid.insert(Loc::new_edge(Pos::new(max_x, y)));
        }

        // fill remaining open locations with default Uninitialized Loc
        for x in min_x..max_x {
            for y in min_y..max_y {
                grid.insert_if_empty(Loc::new(Pos::new(x, y)));
            }
        }

        // update entire grid
        for loc in grid.grid.values_mut() {
            loc.update_state(&coords);
        }

        grid
    }

    pub fn get_locs_part_of(&self, id: CoordId) -> Vec<&Loc> {
        self.grid.values()
            .filter(|loc| loc.is_part_of(id))
            .collect()
    }

    pub fn locations(&self) -> Values<Pos, Loc> {
        self.grid.values()
    }
}
