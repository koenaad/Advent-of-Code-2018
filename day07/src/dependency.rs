use nom::*;
use nom::types::CompleteStr;

#[derive(Debug, Eq, PartialEq)]
pub struct Dependency {
    pub step: char,
    pub depends_on: char,
}

impl Dependency {
    pub fn parse(input: &str) -> Dependency {
        parse_dependency(CompleteStr(input))
            .expect("Could not parse Dependency")
            .1
    }

    pub fn parse_vec(input: &Vec<&str>) -> Vec<Dependency> {
        input.iter()
            .map(|input| Dependency::parse(input))
            .collect()
    }
}

named!(parse_char<CompleteStr, char>,
    map_res!(
        take_while_s!(char::is_alphanumeric),
        |s: CompleteStr| s.chars().next().ok_or(' ')
    )
);

named!(parse_dependency<CompleteStr, Dependency>,
    do_parse!(
        tag!("Step ")                           >>
        depends_on: parse_char                  >>
        tag!(" must be finished before step ")  >>
        step: parse_char                        >>
        tag!(" can begin.")                     >>

        (Dependency { step, depends_on })
    )
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            Dependency::parse("Step C must be finished before step A can begin."),
            Dependency { step: 'A', depends_on: 'C' }
        );
        assert_eq!(
            Dependency::parse("Step C must be finished before step F can begin."),
            Dependency { step: 'F', depends_on: 'C' }
        );
        assert_eq!(
            Dependency::parse("Step A must be finished before step B can begin."),
            Dependency { step: 'B', depends_on: 'A' }
        );
    }
}