extern crate chrono;
#[macro_use]
extern crate nom;

mod event;
mod shift;

use event::*;
use shift::*;
use chrono::prelude::*;

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
    let events = Event::from_vec(&input);
    let shifts = Shift::process_events(events);

    let sleepy_guard = Shift::find_sleepy_guard(&shifts);

    println!("The most sleepy guard is: {}", sleepy_guard);

    let sleepy_shifts = Shift::filter_on_guard(shifts, sleepy_guard);
    let sleepy_minute = Shift::find_sleepy_minute(&sleepy_shifts)
        .expect("Could not find a sleepy minute")
        .minute() as i32;

    println!("The most sleepy minute is: {}", sleepy_minute);

    sleepy_guard * sleepy_minute
}

fn puzzle_2(input: &Vec<&str>) -> i32 {
    let events = Event::from_vec(&input);
    let shifts = Shift::process_events(events);

    let guards = Shift::get_guards(&shifts);

    let (guard, time, count) = guards.iter()
        .map(|guard| {
            let shifts_guard = Shift::filter_on_guard(shifts.clone(), *guard);   // TODO this clone tho...

            if let Some((time, count)) = Shift::find_sleepy_minute_and_count(&shifts_guard) {
                return Some((*guard, time, count));
            } else {
                return None;
            }
        })
        .filter(Option::is_some)
        .map(Option::unwrap)
        .max_by_key(|(_, _, c)| *c)
        .unwrap();

    println!("guard: {:?}, time: {:?}, count: {:?}", guard, time, count);

    guard * (time.minute() as i32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle_1() {
        let mut example = Vec::new();
        example.push("[1518-11-01 00:00] Guard #10 begins shift");
        example.push("[1518-11-01 00:05] falls asleep");
        example.push("[1518-11-01 00:25] wakes up");
        example.push("[1518-11-01 00:30] falls asleep");
        example.push("[1518-11-01 00:55] wakes up");
        example.push("[1518-11-01 23:58] Guard #99 begins shift");
        example.push("[1518-11-02 00:40] falls asleep");
        example.push("[1518-11-02 00:50] wakes up");
        example.push("[1518-11-03 00:05] Guard #10 begins shift");
        example.push("[1518-11-03 00:24] falls asleep");
        example.push("[1518-11-03 00:29] wakes up");
        example.push("[1518-11-04 00:02] Guard #99 begins shift");
        example.push("[1518-11-04 00:36] falls asleep");
        example.push("[1518-11-04 00:46] wakes up");
        example.push("[1518-11-05 00:03] Guard #99 begins shift");
        example.push("[1518-11-05 00:45] falls asleep");
        example.push("[1518-11-05 00:55] wakes up");

        assert_eq!(puzzle_1(&example), 240);
    }

    #[test]
    fn test_puzzle_2() {
        let mut example = Vec::new();
        example.push("[1518-11-01 00:00] Guard #10 begins shift");
        example.push("[1518-11-01 00:05] falls asleep");
        example.push("[1518-11-01 00:25] wakes up");
        example.push("[1518-11-01 00:30] falls asleep");
        example.push("[1518-11-01 00:55] wakes up");
        example.push("[1518-11-01 23:58] Guard #99 begins shift");
        example.push("[1518-11-02 00:40] falls asleep");
        example.push("[1518-11-02 00:50] wakes up");
        example.push("[1518-11-03 00:05] Guard #10 begins shift");
        example.push("[1518-11-03 00:24] falls asleep");
        example.push("[1518-11-03 00:29] wakes up");
        example.push("[1518-11-04 00:02] Guard #99 begins shift");
        example.push("[1518-11-04 00:36] falls asleep");
        example.push("[1518-11-04 00:46] wakes up");
        example.push("[1518-11-05 00:03] Guard #99 begins shift");
        example.push("[1518-11-05 00:45] falls asleep");
        example.push("[1518-11-05 00:55] wakes up");

        assert_eq!(puzzle_2(&example), 4455);
    }
}
