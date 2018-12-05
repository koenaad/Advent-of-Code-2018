use chrono::prelude::*;
use nom::types::CompleteStr;

#[derive(Clone, Debug)]
pub struct Event {
    pub datetime: NaiveDateTime,
    pub action: Action,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Action {
    BeginShift(i32),
    WakesUp,
    FallsAsleep,
}

impl Event {
    pub fn from(line: &str) -> Event {
        parse_event(CompleteStr(line))
            .expect("Failed to parse event")
            .1
    }

    pub fn from_vec(lines: &Vec<&str>) -> Vec<Event> {
        lines.iter()
            .map(|line| Event::from(line))
            .collect()
    }
}

//***** nom parsing *****//

// TODO: generify into something like parse_num<T>

named!(parse_i32<CompleteStr, i32>,
    map_res!(
        take_while_s!(char::is_numeric),
        |s: CompleteStr| s.parse::<i32>()
    )
);

named!(parse_u32<CompleteStr, u32>,
    map_res!(
        take_while_s!(char::is_numeric),
        |s: CompleteStr| s.parse::<u32>()
    )
);

// example: "[1518-11-01 00:00]"
named!(parse_datetime<CompleteStr, NaiveDateTime>,
    do_parse!(
        tag!("[")               >>
        year: parse_i32         >>
        take!(1)                >>
        month: parse_u32        >>
        take!(1)                >>
        day: parse_u32          >>
        take!(1)                >>
        hour: parse_u32         >>
        take!(1)                >>
        minute: parse_u32       >>
        tag!("]")               >>

        (NaiveDate::from_ymd(year, month, day).and_hms(hour, minute, 00))
    )
);

// 3 actions:
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

// example: "[1518-11-01 00:00] Guard #10 begins shift"
named!(parse_event<CompleteStr, Event>,
    do_parse!(
        datetime: parse_datetime    >>
        take!(1)                    >>
        action: parse_action        >>

        (Event { datetime, action })
    )
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_datetime() {
        assert_eq!(
            parse_datetime(CompleteStr("[1518-11-01 00:00]")).unwrap().1,
            (NaiveDate::from_ymd(1518, 11, 01).and_hms(00, 00, 00))
        );
        assert_eq!(
            parse_datetime(CompleteStr("[1518-11-02 00:40]")).unwrap().1,
            (NaiveDate::from_ymd(1518, 11, 02).and_hms(00, 40, 00))
        );
        assert_eq!(
            parse_datetime(CompleteStr("[1518-11-03 00:24]")).unwrap().1,
            (NaiveDate::from_ymd(1518, 11, 03).and_hms(00, 24, 00))
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
        let event = Event::from("[1518-11-01 00:00] Guard #10 begins shift");

        assert_eq!(event.datetime, NaiveDate::from_ymd(1518, 11, 01).and_hms(00, 00, 00));
        assert_eq!(event.action, Action::BeginShift(10));

        let event = Event::from("[1518-11-01 00:05] falls asleep");

        assert_eq!(event.datetime, NaiveDate::from_ymd(1518, 11, 01).and_hms(00, 05, 00));
        assert_eq!(event.action, Action::FallsAsleep);

        let event = Event::from("[1518-11-01 00:25] wakes up");

        assert_eq!(event.datetime, NaiveDate::from_ymd(1518, 11, 01).and_hms(00, 25, 00));
        assert_eq!(event.action, Action::WakesUp);
    }
}
