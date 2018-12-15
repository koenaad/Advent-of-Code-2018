pub const PATTERN_LEN: usize = 5;

#[derive(Debug, Eq, PartialEq)]
pub struct Rule {
    pub pattern: Vec<bool>,
    pub result: bool,
}

impl Rule {
    /// Parse input with the format `...## => .` as a propagation rule.
    pub fn parse(input: &str) -> Rule {
        let pattern = input.chars()
            .take(PATTERN_LEN)
            .map(|c| c == '#' )
            .collect();

        let result = input.chars().nth(9).map(|c| c == '#')
            .expect("Could not get result from rule");

        Rule { pattern, result }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            Rule::parse("...## => #"),
            Rule { pattern: vec!(false, false, false, true, true ), result: true }
        );
        assert_eq!(
            Rule::parse(".#.#. => ."),
            Rule { pattern: vec!(false, true, false, true, false ), result: false }
        );
    }
}
