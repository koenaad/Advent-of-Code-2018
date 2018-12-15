struct RecipeBoard {
    list: Vec<u8>,
    elf1: usize,
    elf2: usize,
}

impl RecipeBoard {
    fn new(recipe_elf1: u8, recipe_elf2: u8) -> RecipeBoard {
        RecipeBoard { list: vec![recipe_elf1, recipe_elf2], elf1: 0, elf2: 1 }
    }

    fn create_new_recipe(&mut self) {
        let recipe_elf1 = self.list[self.elf1];
        let recipe_elf2 = self.list[self.elf2];

        let new_recipe = recipe_elf1 + recipe_elf2;

        if new_recipe >= 10 {
            self.list.push(new_recipe / 10);
        }
        self.list.push(new_recipe % 10);

        self.elf1 = (self.elf1 + 1 + recipe_elf1 as usize) % self.list.len();
        self.elf2 = (self.elf2 + 1 + recipe_elf2 as usize) % self.list.len();
    }

    fn find_10_recipes_after(&mut self, experiments: usize) -> Vec<u8> {
        for _ in 0..experiments + 10 {
            self.create_new_recipe();
        }

        self.list.iter()
            .skip(experiments)
            .take(10)
            .map(|&recipe| recipe)
            .collect()
    }

    fn find_recipes_until(&mut self, scores: &Vec<u8>) -> usize {
        let mut last_checked_offset = 0;

        let len_scores = scores.len();

        loop {
            self.create_new_recipe();

            if self.list.len() < len_scores {
                continue;
            }

            // continuously check the last `scores.len` values for a match
            while last_checked_offset < self.list.len() - len_scores {
                let slice = self.list[last_checked_offset..last_checked_offset + len_scores].to_vec();

                if slice == *scores {
                    return last_checked_offset;
                }
                last_checked_offset += 1;
            }
        }
    }
}

const INPUT: usize = 633601;
const INPUT_ARR: [u8; 6] = [6, 3, 3, 6, 0, 1];

fn main() {
    println!("Puzzle 1: {:?}", puzzle_1(INPUT));
    println!("Puzzle 2: {:?}", puzzle_2(&INPUT_ARR.to_vec()));
}

fn puzzle_1(experiments: usize) -> Vec<u8> {
    let mut board = RecipeBoard::new(3, 7);

    board.find_10_recipes_after(experiments)
}

fn puzzle_2(scores: &Vec<u8>) -> usize {
    let mut board = RecipeBoard::new(3, 7);

    board.find_recipes_until(scores)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle_1() {
        assert_eq!(puzzle_1(9), vec![5, 1, 5, 8, 9, 1, 6, 7, 7, 9]);
        assert_eq!(puzzle_1(5), vec![0, 1, 2, 4, 5, 1, 5, 8, 9, 1]);
        assert_eq!(puzzle_1(18), vec![9, 2, 5, 1, 0, 7, 1, 0, 8, 5]);
        assert_eq!(puzzle_1(2018), vec![5, 9, 4, 1, 4, 2, 9, 8, 8, 2]);
    }

    #[test]
    fn test_puzzle_2() {
        assert_eq!(puzzle_2(&vec![5, 1, 5, 8, 9]), 9);
        assert_eq!(puzzle_2(&vec![0, 1, 2, 4, 5]), 5);
        assert_eq!(puzzle_2(&vec![9, 2, 5, 1, 0]), 18);
        assert_eq!(puzzle_2(&vec![5, 9, 4, 1, 4]), 2018);
    }
}
