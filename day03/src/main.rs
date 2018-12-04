#[macro_use]
extern crate nom;

use std::collections::HashSet;
use std::iter::FromIterator;
use nom::types::CompleteStr;

#[derive(PartialEq, Eq, Hash, Debug)]
struct Claim {
    id: i32,
    dist_left: i32,
    dist_top: i32,
    width: i32,
    height: i32,
}

named!(parse_i32<CompleteStr, i32>,
    map_res!(
        take_while_s!(char::is_numeric),
        |s: CompleteStr| s.parse::<i32>()
    )
);

// example: "#1 @ 1,3: 4x4"
named!(parse_claim<CompleteStr, Claim>,
    do_parse!( 
        tag!("#")               >>
        id: parse_i32           >>
        take!(3)                >>  // this hard-coded stuff is pretty poor...
        dist_left: parse_i32    >>
        take!(1)                >>
        dist_top: parse_i32     >>
        take!(2)                >>
        width: parse_i32        >>
        take!(1)                >>
        height: parse_i32       >>

        (Claim { id, dist_left, dist_top, width, height })
    )
);

impl Claim {
    fn from(line: &str) -> Claim {
        parse_claim(CompleteStr(line))
            .expect("Failed to parse claim")
            .1
    }

    fn from_vec(lines: &Vec<&str>) -> Vec<Claim> {
        lines.iter()
            .map(|line| Claim::from(line))
            .collect()
    }
}

// TODO: tests for fabric
struct Fabric {
    width: i32,
    height: i32,
    square_inches: Vec<Vec<i32>>,   // a 2D vector of claim ids
}

impl Fabric {
    fn new(width: i32, height: i32) -> Fabric {
        Fabric { width, height, square_inches: vec![vec![]; (width * height) as usize] }
    }

    fn apply(&mut self, claim: &Claim) {
        let x_range = (claim.dist_left, claim.dist_left + claim.width);
        let y_range = (claim.dist_top, claim.dist_top + claim.height);

        for x in x_range.0..x_range.1 {
            for y in y_range.0..y_range.1 {
                let index = (x + (y * self.width)) as usize;
                let square_inch = self.square_inches.get_mut(index)
                    .expect("Claim not in range");   // should not be able to happen

                square_inch.push(claim.id);
            }
        }
    }

    fn apply_all(&mut self, claims: &Vec<Claim>) {
        for claim in claims {
            self.apply(&claim);
        }
    }
}

fn get_input() -> Vec<&'static str> {
    include_str!("../input.txt")
        .split("\n")
        .filter(|s| s.len() > 0)
        .collect()
}

fn main() {
    let input = get_input();

    println!("Puzzle 1: {}", puzzle_1(&input));
    println!("Puzzle 2: {}", puzzle_2(&input));
}

fn puzzle_1(input: &Vec<&str>) -> i32 {
    let claims = Claim::from_vec(&input);

    let mut fabric = Fabric::new(1000, 1000);
    fabric.apply_all(&claims);

    fabric.square_inches.iter()
        .filter(|count| count.len() > 1)
        .count() as i32
}

fn puzzle_2(input: &Vec<&str>) -> i32 {
    let claims = Claim::from_vec(&input);

    let mut fabric = Fabric::new(1000, 1000);
    fabric.apply_all(&claims);

    let ids = HashSet::from_iter(
        claims.iter()
            .map(|claim| claim.id)
    );

    let ids_in_conflict = HashSet::<i32>::from_iter(
        fabric.square_inches.iter()
            .filter(|square_inch| square_inch.len() > 1)
            .flatten()
            .map(|id: &i32| id.clone())
    );

    ids.difference(&ids_in_conflict)
        .map(|id: &i32| id.clone())
        .next()
        .expect("Could not find solution")  // fails if difference is empty
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_claim_from() {
        assert_eq!(Claim::from("#1 @ 1,3: 4x4"), Claim{ id: 1, dist_left: 1, dist_top: 3, width: 4, height: 4 });
        assert_eq!(Claim::from("#2 @ 3,1: 4x4"), Claim{ id: 2, dist_left: 3, dist_top: 1, width: 4, height: 4 });
        assert_eq!(Claim::from("#3 @ 5,5: 2x2"), Claim{ id: 3, dist_left: 5, dist_top: 5, width: 2, height: 2 });
        assert_eq!(Claim::from("#1 @ 662,777: 18x27"), Claim{ id: 1, dist_left: 662, dist_top: 777, width: 18, height: 27 });
    }

    #[test]
    fn test_claim_from_input() {
        let input = get_input();
        let claims = Claim::from_vec(&input);

        assert_eq!(claims.len(), input.len());
    }

    #[test]
    fn test_puzzle_1() {
        let mut example = Vec::new();
        example.push("#1 @ 1,3: 4x4");
        example.push("#2 @ 3,1: 4x4");
        example.push("#3 @ 5,5: 2x2");

        assert_eq!(puzzle_1(&example), 4);
    }

    #[test]
    fn test_puzzle_2() {
        let mut example = Vec::new();
        example.push("#1 @ 1,3: 4x4");
        example.push("#2 @ 3,1: 4x4");
        example.push("#3 @ 5,5: 2x2");
        
        assert_eq!(puzzle_2(&example), 3);
    }
}
