use event::*;
use std::collections::HashMap;
use chrono::Duration;
use chrono::prelude::*;

#[derive(Clone, Debug)]
pub struct Shift {
    pub guard: i32,
    pub events: Vec<Event>,
}

impl Shift {
    pub fn process_events(mut events: Vec<Event>) -> Vec<Shift> {
        // sort chronological
        events.sort_by(|e1, e2| { e1.datetime.cmp(&e2.datetime) });

        let mut shifts: Vec<Shift> = Vec::new();

        // parse into separate shifts, by taking everything up to the next BeginShiftOf
        while events.len() > 0 {
            match events.get(0).unwrap().action {
                Action::BeginShiftOf(guard) => {
                    let drain_index = Shift::index_of_second_begin_shift(&events)
                        .unwrap_or(events.len());

                    let shift_events = events.drain(..drain_index).collect();

                    shifts.push(Shift { guard, events: shift_events });
                }
                _ => {
                    panic!("BeginShiftOf not at begin of events")
                }
            }
        }
        shifts
    }

    fn index_of_second_begin_shift(events: &Vec<Event>) -> Option<usize> {
        events.iter()
            .enumerate()
            .skip(1)
            .find(|(_, e)| {
                match e.action {
                    Action::BeginShiftOf(_) => true,
                    _ => false,
                }
            })
            .map(|(i, _)| i)
    }

    /// Find the total amount of minutes the guard was asleep this shift.
    pub fn total_time_asleep(&self) -> i32 {
        let mut total_asleep = 0;
        let mut start_asleep: Option<NaiveDateTime> = None;

        // events are expected to be sorted by date and time
        for event in &self.events {
            match event.action {
                Action::BeginShiftOf(_)   => {},
                Action::FallsAsleep     => {
                    start_asleep = Some(event.datetime);
                },
                Action::WakesUp         => {
                    if let Some(start) = start_asleep {
                        total_asleep += event.datetime.signed_duration_since(start).num_minutes();
                    } else {
                        panic!("A WakesUp occured before FallsAsleep");
                    }
                }
            }
        }
        total_asleep as i32
    }

    /// From a list of shifts, get all unique guards.
    pub fn get_guards(shifts: &Vec<Shift>) -> Vec<i32> {
        let mut guards: Vec<i32> = shifts.iter()
            .map(|shift| shift.guard)
            .collect();

        guards.sort();
        guards.dedup();

        guards
    }

    /// Filter a list of shifts on the specified guard.
    pub fn filter_on_guard(shifts: Vec<Shift>, guard: i32) -> Vec<Shift> {
        shifts.into_iter()
            .filter(|shift| shift.guard == guard)
            .collect()
    }

    /// From a list of shifts, find the guard that slept the most in total.
    pub fn find_sleepy_guard(shifts: &Vec<Shift>) -> i32 {
        let mut sleep_time_per_guard: HashMap<i32, i32> = HashMap::new();

        for shift in shifts {
            *sleep_time_per_guard.entry(shift.guard).or_insert(0) += shift.total_time_asleep();
        }

        *sleep_time_per_guard.iter()
            .max_by_key(|entry| entry.1)    // maximize on the value of the hashmap tuple
            .unwrap()
            .0
    }

    /// From a list of shifts, find the minute a guard was most often asleep.
    pub fn find_sleepy_minute(shifts: &Vec<Shift>) -> Option<NaiveTime> {
        if let Some((time, _)) = Shift::find_sleepy_minute_and_count(shifts) {
            return Some(time);
        } else {
            return None;
        }
    }

    /// From a list of shifts, find the minute a guard was most often asleep and how often that was.
    pub fn find_sleepy_minute_and_count(shifts: &Vec<Shift>) -> Option<(NaiveTime, i32)> {
        let mut time_asleep: HashMap<NaiveTime, i32> = HashMap::new();
        let mut start_sleep: Option<NaiveDateTime> = None;

        for shift in shifts {
            // events are expected to be sorted by date and time
            // also, we _assume_ each FallsAsleep is paired with a WakesUp
            // and that a WakesUp never occurs before a FallsAsleep
            for event in &shift.events {
                match event.action {
                    Action::BeginShiftOf(_)       => {},
                    Action::FallsAsleep         => {
                        start_sleep = Some(event.datetime);
                    },
                    Action::WakesUp             => {
                        let mut start = start_sleep
                            .expect("A WakesUp occured before FallsAsleep");
                        let end = event.datetime;

                        while start != end {
                            let entry = time_asleep.entry(start.time()).or_insert(0);
                            *entry += 1;

                            start += Duration::minutes(1);
                        }
                    }
                }
            }
        }

        let sleepy_minute = time_asleep.iter()
            .max_by_key(|entry| entry.1);    // maximize on the value of the hashmap entry

        if let Some((time, count)) = sleepy_minute {
            return Some((time.clone(), *count));
        } else {
            return None;
        }
    }
}

#[cfg(test)]
mod tests {
    // the lack of tests makes me sad
}
