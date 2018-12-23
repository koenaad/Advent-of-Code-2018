use nom::*;
use nom::types::CompleteStr;
use crate::vec3::Vec3;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Nanobot {
    pub pos: Vec3,
    pub radius: usize,
}

impl Nanobot {
    pub fn parse(input: &str) -> Result<Nanobot, &'static str> {
        let nanobot = do_parse!(CompleteStr(input),
            tag!("pos=<")           >>
            x: parse_int            >>
            tag!(",")               >>
            y: parse_int            >>
            tag!(",")               >>
            z: parse_int            >>
            tag!(">, r=")           >>
            r: parse_int            >>

            (Nanobot { pos: Vec3::new(x, y, z), radius: r as usize })
        );
        if let Ok(nanobot) = nanobot {
            Ok(nanobot.1)
        } else {
            Err("could not parse nanobot")
        }
    }

    pub fn can_reach(&self, pos: &Vec3) -> bool {
        Vec3::distance(&self.pos, &pos) <= self.radius
    }

    pub fn bots_within_reach_of(pos: &Vec3, bots: &Vec<Self>) -> usize {
        bots.iter()
            .filter(|bot| bot.can_reach(&pos))
            .count()
    }
}

named!(pub parse_int<CompleteStr, isize>,
    alt!(
        do_parse!(
            take_while!(char::is_whitespace)    >>
            tag!("-")                           >>
            int: take_while!(char::is_numeric)  >>
            
            (-1 * int.parse::<isize>().expect("parse<isize>"))
        ) |
        do_parse!(
            take_while!(char::is_whitespace)    >>
            int: take_while!(char::is_numeric)  >>

            (int.parse::<isize>().expect("parse<isize>"))
        )
    )
);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(Nanobot::parse("pos=<0,0,0>, r=4"), Ok(Nanobot { pos: Vec3::new(0, 0, 0), radius: 4 }));
        assert_eq!(Nanobot::parse("pos=<-1,3,5>, r=1"), Ok(Nanobot { pos: Vec3::new(-1, 3, 5), radius: 1 }));
    }
}
