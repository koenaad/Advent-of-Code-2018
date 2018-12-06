// Make sure to use 'cargo run --release` when using the complete input. This redcues execution
// time from 4m20s to 7s...

fn to_vec(input: &str) -> Vec<char> {
    input.chars()
        .collect()
}

/// Finds an active pair (= same type, but difference case) and returns the index.
fn find_active_pair(polymer: &Vec<char>) -> Option<usize> {
    polymer.iter()
        .enumerate()
        .find(|(i, c1)| {
            if let Some(c2) = polymer.get(i + 1) {
                return c1.eq_ignore_ascii_case(&c2) && *c1 != c2;
            } else {
                return false;
            }
        })
        .map(|(i, _)| i)
}

/// Removes all active pairs from the polymer.
fn let_it_react(mut polymer: Vec<char>) -> Vec<char> {
    loop {
        let active_pair = find_active_pair(&polymer);

        if let Some(index) = active_pair {
            // TODO: optimization:
            //  - each remove means elements to the right have to be move one forward...
            //  - use a linked list instead? remove multiple indexes at once?
            polymer.remove(index);
            polymer.remove(index);
        } else {
            break;
        }
    }
    polymer
}

fn main() {
    let input = include_str!("../input.txt")
        .trim();

    println!("Puzzle 1: {}", puzzle_1(input));
    println!("Puzzle 2: {}", puzzle_2(input));
}

fn puzzle_1(input: &str) -> i32 {
    let mut polymer = to_vec(&input);

    polymer = let_it_react(polymer);

    polymer.len() as i32
}

fn puzzle_2(input: &str) -> i32 {
    let mut polymer = to_vec(&input);

    polymer = let_it_react(polymer);

    let filtered_polymers: Vec<Vec<char>> = "abcdefghijklmnopqrstuvwxyz".chars()
        .map(|filtered_type| {
            polymer.clone().iter()
                .filter(|c| !c.eq_ignore_ascii_case(&filtered_type))
                .map(|c| *c)
                .collect()
        })
        .collect();

    filtered_polymers.iter()
        .map(|polymer| let_it_react(polymer.clone()).len() as i32)
        .min()
        .expect("Could not find a solution")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_first_active_unit() {
        assert_eq!(find_active_pair(&vec!['a', 'A', 'b', 'c']), Some(0));
        assert_eq!(find_active_pair(&vec!['a', 'b', 'B', 'c']), Some(1));
        assert_eq!(find_active_pair(&vec!['a', 'A', 'b', 'B']), Some(0));
        assert_eq!(find_active_pair(&vec!['a', 'B', 'B', 'b']), Some(2));
        assert_eq!(find_active_pair(&vec!['a', 'a', 'B', 'B']), None);
        assert_eq!(find_active_pair(&vec!['a', 'b', 'c', 'D']), None);
    }

    #[test]
    fn test_puzzle_1() {
        let example = "dabAcCaCBAcCcaDA";

        assert_eq!(puzzle_1(example), 10);
    }

    #[test]
    fn test_puzzle_2() {
        let example = "dabAcCaCBAcCcaDA";

        assert_eq!(puzzle_2(example), 4);
    }
}
