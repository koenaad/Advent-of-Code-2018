use std::fmt::{Display, Formatter, Result};
use crate::rule::Rule;

#[derive(Debug)]
pub struct Row {
    state: Vec<bool>,
    offset: usize,  // pot 0 is at index `offset`
}

impl Row {
    pub fn new(initial_state: &str) -> Row {
        let state = initial_state.chars()
            .map(|c| c == '#')
            .collect();

        Row { state, offset: 0 }
    }

    pub fn is_pot_alive(&self, num: isize) -> bool {
        if num + (self.offset as isize) < 0 {
            return false;
        }
        *self.state.get(self.from_pot_num(num)).unwrap_or(&false)
    }

    fn to_pot_num(&self, index: usize) -> isize {
        (index as isize) - (self.offset as isize)
    }

    fn from_pot_num(&self, pot_num: isize) -> usize {
        (pot_num + (self.offset as isize)) as usize
    }

    pub fn sum_of_pots_alive(&self) -> isize {
        self.state.iter()
            .enumerate()
            .filter(|(_, b)| **b)
            .map(|(i, _)| self.to_pot_num(i))
            .sum()
    }

    pub fn next_generation(&mut self, rules: &Vec<Rule>) -> Row {
        // make sure there are 4 dead pots at the beginning
        while self.state[0..4].to_vec() != vec![false, false, false, false] {
            self.state.insert(0, false);
            self.offset += 1;
        }

        // make sure there are 4 dead pots at the end
        while self.state[self.state.len()-4..self.state.len()].to_vec() != vec![false, false, false, false] {
            self.state.push(false);
        }

        let mut new_state = vec![false; self.state.len()];
        let new_offset = self.offset;

        rules.iter()
            .for_each(|rule| {
                let slice = rule.pattern.as_slice();

                self.state.windows(5)
                    .enumerate()
                    .filter(|(_, w)| slice == *w)
                    .for_each(|(i, _)| {
                        new_state[i + 2] = rule.result;
                    });
            });

        Row { state: new_state, offset: new_offset }
    }
}

impl Display for Row {
    fn fmt(&self, f: &mut Formatter) -> Result {
        // row 1: pot numbers
        for i in 0..self.state.len() {
            let pot_num = self.to_pot_num(i);
            if pot_num % 10 == 0 {
                write!(f, "{}", pot_num / 10)?;
            } else {
                write!(f, " ")?;
            }
        }
        write!(f, "\n")?;

        // row 2: pots
        for i in 0..self.state.len() {
            if self.state[i] {
                write!(f, "#")?;
            } else {
                write!(f, ".")?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sum_of_pots_alive() {
        let row1 = Row { state: vec![true, false, false, true, true], offset: 0 };
        let row2 = Row { state: vec![true, false, true, false, false, true, true], offset: 2 };

        assert_eq!(row1.sum_of_pots_alive(), 7);
        assert_eq!(row2.sum_of_pots_alive(), 5);
    }

    #[test]
    fn test_next_generation() {
        let mut row1 = Row { state: vec![true, false, false, true, true], offset: 0 };

        let rules = vec![
            Rule { pattern: vec![true, false, false, true, true], result: true },
        ];

        let row2 = row1.next_generation(&rules);

        println!("{}", row1);
        println!("{}", row2);

        assert_eq!(row2.is_pot_alive(0), false);
        assert_eq!(row2.is_pot_alive(1), false);
        assert_eq!(row2.is_pot_alive(2), true);
        assert_eq!(row2.is_pot_alive(3), false);
        assert_eq!(row2.is_pot_alive(4), false);
    }

}