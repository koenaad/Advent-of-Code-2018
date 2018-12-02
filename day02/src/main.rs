use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt")
        .split("\n")
        .collect::<Vec<&str>>();

    println!("Puzzle 1: {}", puzzle_1(&input));
    println!("Puzzle 2: {}", puzzle_2(&input));
}

fn puzzle_1(input: &Vec<&str>) -> i32 {
    let mut count_doubles = 0;
    let mut count_triples = 0;

    for id in input {
        let occurances = count_occurances(id);

        if occurances.values().find(|&&e| e == 2).is_some() {
            count_doubles += 1;
        }
        if occurances.values().find(|&&e| e == 3).is_some() {
            count_triples += 1;
        }
    }
    count_doubles * count_triples
}

fn count_occurances(id: &str) -> HashMap<char, i32> {
    let mut occurances = HashMap::new();

    for c in id.chars() {
        let count = occurances.entry(c).or_insert(0);
        *count += 1;
    }
    occurances
}

fn puzzle_2(input: &Vec<&str>) -> String {
    let len = input.len();

    for i in 0..len {
        for j in i..len {
            let id1 = input.get(i).unwrap();
            let id2 = input.get(j).unwrap();

            if differs_by_one_char(id1, id2) {
                return filter_differences(id1, id2);
            }
        }
    }
    panic!("Found no solution.");
}

fn differs_by_one_char(str1: &str, str2: &str) -> bool {
    str1.chars().zip(str2.chars())
        .filter(|c| c.0 != c.1)
        .count() == 1
}

fn filter_differences(str1: &str, str2: &str) -> String {
    str1.chars().zip(str2.chars())
        .filter(|c| c.0 == c.1)
        .map(|(c1, _c2)| c1)    // just throw away half of tuple, unzip is having issue deducting types... 🙄
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_occurances() {
        let occurances = count_occurances("aabc");

        assert_eq!(*occurances.get(&'a').unwrap(), 2);
        assert_eq!(*occurances.get(&'b').unwrap(), 1);
        assert_eq!(*occurances.get(&'c').unwrap(), 1);
        assert!(occurances.get(&'d').is_none());
    }

    #[test]
    fn test_puzzle_1() {
        let mut example = Vec::new();
        example.push("abcdef");
        example.push("bababc");
        example.push("abbcde");
        example.push("abcccd");
        example.push("aabcdd");
        example.push("abcdee");
        example.push("ababab");

        assert_eq!(puzzle_1(&example), 12);
    }

    #[test]
    fn test_differs_by_one_char() {
        assert_eq!(differs_by_one_char("abcd", "abcd"), false);
        assert_eq!(differs_by_one_char("abcd", "abcc"), true);
        assert_eq!(differs_by_one_char("abcd", "abbb"), false);
        assert_eq!(differs_by_one_char("abcd", "aaaa"), false);
    }

    #[test]
    fn test_filter_differences() {
        assert_eq!(filter_differences("abcd", "abcd"), "abcd");
        assert_eq!(filter_differences("abcd", "abcc"), "abc");
        assert_eq!(filter_differences("abcd", "abbb"), "ab");
        assert_eq!(filter_differences("abcd", "aaaa"), "a");
        assert_eq!(filter_differences("abcd", "aacd"), "acd");
    }

    #[test]
    fn test_puzzle_2() {
        let mut example = Vec::new();
        example.push("abcde");
        example.push("fghij");
        example.push("klmno");
        example.push("pqrst");
        example.push("fguij");
        example.push("axcye");
        example.push("wvxyz");

        assert_eq!(puzzle_2(&example), "fgij".to_string());
    }
}
