mod circular_vec;

use crate::circular_vec::CircularVec;
use std::collections::HashMap;
use std::time::Instant;

/// Returns (winning player, winning score).
fn play_the_game(players: i32, marbles: u64) -> (i32, u64) {
    let mut list = CircularVec::new();  // TODO: use a linked list
    let mut scores = HashMap::new();

    let mut curr_index = 0;
    let mut curr_player = 0;

    list.insert(0, 0);

    let start = Instant::now();

    for marble in 1..marbles+1 {
        if marble % 23 != 0 {
            curr_index = list.insert(curr_index + 2, marble);
        } else {
            let (removed_index, removed_marble) = list.get_and_remove(curr_index - 7);
            curr_index = removed_index;

            let player_score = scores.entry(curr_player).or_insert(0);
            *player_score += marble;
            *player_score += removed_marble;
        }
        // println!("p: {}\tm: {}\ti: {}\tl: {:?}\ts: {:?}", curr_player, marble, curr_index, list, scores);

        if marble % 100_000 == 0 {
            println!("{}: {} of {}", start.elapsed().as_secs(), marble, marbles);
        }

        curr_player = (curr_player + 1) % players;
    }

    let (player, score) = scores.iter()
        .max_by_key(|(_, score)| *score)
        .unwrap();

    println!("Play time: {}", start.elapsed().as_secs());

    // players is zero-indexed!
    (player + 1, *score)
}

fn main() {
    println!("Puzzle 1: {}", play_the_game(405, 70_953).1);
    println!("Puzzle 2: {}", play_the_game(405, 70_953 * 100).1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_1() {
        assert_eq!(play_the_game(9, 25).1, 32);
    }

    #[test]
    fn test_game_2() {
        assert_eq!(play_the_game(10, 1618).1, 8317);
    }

    #[test]
    fn test_game_3() {
        assert_eq!(play_the_game(13, 7999).1, 146_373);
    }

    #[test]
    fn test_game_4() {
        assert_eq!(play_the_game(17, 1104).1, 2764);
    }

    #[test]
    fn test_game_5() {
        assert_eq!(play_the_game(21, 6111).1, 54_718);
    }

    #[test]
    fn test_game_6() {
        assert_eq!(play_the_game(30, 5807).1, 37305);
    }
}
