#[macro_use]
extern crate nom;

use nom::types::CompleteStr;

#[derive(PartialEq, Eq, Hash, Debug)]
struct Claim {
    id: i32,
    dist_left: i32,
    dist_top: i32,
    width: i32,
    height: i32,
}

named!(parse_number<CompleteStr, i32>,
    map_res!(
        take_while_s!(char::is_numeric),
        |s: CompleteStr| s.parse::<i32>()
    )
);

// example: "#1 @ 1,3: 4x4"
named!(parse_claim<CompleteStr, Claim>,
    do_parse!( 
        tag!("#")               >>
        id: parse_number        >>
        take!(3)                >>  // this hard-coded stuff is pretty poor...
        dist_left: parse_number >>
        take!(1)                >>
        dist_top: parse_number  >>
        take!(2)                >>
        width: parse_number     >>
        take!(1)                >>
        height: parse_number    >>

        (Claim { id, dist_left, dist_top, width, height })
    )
);

impl Claim {
    fn from(line: &str) -> Claim {
        parse_claim(CompleteStr(line))
            .expect("Failed to parse claim")
            .1
    }
}

fn main() {
    let input = include_str!("../input.txt")
        .split("\n")
        .collect::<Vec<&str>>();

    println!("Puzzle 1: {}", puzzle_1(&input));
    println!("Puzzle 2: {}", puzzle_2(&input));
}

fn puzzle_1(input: &Vec<&str>) -> i32 {
    let claims: Vec<Claim> = input.iter()
        .map(|line| Claim::from(line))
        .collect();

    for claim in claims {
        println!("{:?}", claim);
    }

    0
}

fn puzzle_2(_input: &Vec<&str>) -> i32 {
    0
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
        example.push("");
        
        assert_eq!(puzzle_2(&example), 0);
    }
}
