#[macro_use]
extern crate nom;

use nom::types::CompleteStr;

named!(parse_i32<CompleteStr, i32>,
    map_res!(
        take_while_s!(char::is_numeric),
        |s: CompleteStr| s.parse::<i32>()
    )
);

#[derive(PartialEq, Eq, Hash, Debug)]
struct Timestamp {
    year: i32,
    month: i32,
    day: i32,
    hour: i32,
    minute: i32,
}

// example: "[1518-11-01 00:00]"
named!(parse_timestamp<CompleteStr, Timestamp>,
    do_parse!(
        tag!("[")               >>
        year: parse_i32         >>
        take!(1)                >>  // this hard-coded stuff is pretty poor...
        month: parse_i32        >>
        take!(1)                >>
        day: parse_i32          >>
        take!(1)                >>
        hour: parse_i32         >>
        take!(1)                >>
        minute: parse_i32       >>
        tag!("]")               >>

        (Timestamp { year, month, day, hour, minute })
    )
);

#[derive(PartialEq, Eq, Hash, Debug)]
enum Action {
    BeginShift(i32),
    WakesUp,
    FallsAsleep,
}

// 3 options:
//  - "Guard #10 begins shift"
//  - "falls asleep"
//  - "wakes up"
named!(parse_action<CompleteStr, Action>,
    alt!(
        do_parse!(
            tag!("Guard #") >>
            id: parse_i32   >>

            (Action::BeginShift(id))
        )                                               |
        value!(Action::FallsAsleep, tag!("falls"))      |
        value!(Action::WakesUp, tag!("wakes"))
    )
);

#[derive(PartialEq, Eq, Hash, Debug)]
struct Event {
    time: Timestamp,
    action: Action,
}

// example: "[1518-11-01 00:00] Guard #10 begins shift"
named!(parse_event<CompleteStr, Event>,
    do_parse!(
        time: parse_timestamp   >>
        take!(1)                >>
        action: parse_action    >>

        (Event { time, action })
    )
);

impl Event {
    fn from(line: &str) -> Event {
        parse_event(CompleteStr(line))
            .expect("Failed to parse event")
            .1
    }

    fn from_vec(lines: &Vec<&str>) -> Vec<Event> {
        lines.iter()
            .map(|line| Event::from(line))
            .collect()
    }
}

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
    let mut events = Event::from_vec(&input);

    // TODO: implement Ord/PartialOrd and internalize this
    events.sort_by(|ev1, ev2| {
        let t1 = &ev1.time;
        let t2 = &ev2.time;

        t1.year.cmp(&t2.year)
            .then(t1.month.cmp(&t2.month))
            .then(t1.day.cmp(&t2.day))
            .then(t1.hour.cmp(&t2.hour))
            .then(t1.minute.cmp(&t2.minute))
    });

    for event in events {
        println!("{:?}", event);
    }

    0
}

fn puzzle_2(input: &Vec<&str>) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_timestamp() {
        assert_eq!(
            parse_timestamp(CompleteStr("[1518-11-01 00:00]")).unwrap().1,
            Timestamp { year: 1518, month: 11, day: 01, hour: 00, minute: 00 }
        );
        assert_eq!(
            parse_timestamp(CompleteStr("[1518-11-02 00:40]")).unwrap().1,
            Timestamp { year: 1518, month: 11, day: 02, hour: 00, minute: 40 }
        );
        assert_eq!(
            parse_timestamp(CompleteStr("[1518-11-03 00:24]")).unwrap().1,
            Timestamp { year: 1518, month: 11, day: 03, hour: 00, minute: 24 }
        );
    }

    #[test]
    fn test_parse_action() {
        assert_eq!(
            parse_action(CompleteStr("Guard #10 begins shift")).unwrap().1,
            Action::BeginShift(10)
        );
        assert_eq!(
            parse_action(CompleteStr("Guard #5 begins shift")).unwrap().1,
            Action::BeginShift(5)
        );
        assert_eq!(
            parse_action(CompleteStr("falls asleep")).unwrap().1,
            Action::FallsAsleep
        );
        assert_eq!(
            parse_action(CompleteStr("wakes up")).unwrap().1,
            Action::WakesUp
        );
    }

    #[test]
    fn test_event_from() {
        assert_eq!(
            Event::from("[1518-11-01 00:00] Guard #10 begins shift"),
            Event {
                time: Timestamp { year: 1518, month: 11, day: 01, hour: 00, minute: 00 },
                action: Action::BeginShift(10),
            }
        );
        assert_eq!(
            Event::from("[1518-11-01 00:05] falls asleep"),
            Event {
                time: Timestamp { year: 1518, month: 11, day: 01, hour: 00, minute: 05 },
                action: Action::FallsAsleep,
            }
        );
        assert_eq!(
            Event::from("[1518-11-01 00:25] wakes up"),
            Event {
                time: Timestamp { year: 1518, month: 11, day: 01, hour: 00, minute: 25 },
                action: Action::WakesUp,
            }
        );
    }

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
        
        assert_eq!(puzzle_2(&example), 0);
    }
}
