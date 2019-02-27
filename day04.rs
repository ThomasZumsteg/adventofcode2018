extern crate common;
extern crate regex;

use std::ops::Sub;
use std::cmp::Ordering;
use std::collections::HashMap;

use common::get_input;
use regex::Regex;

type GuardId = u16;

#[derive(Clone, Debug)]
enum EventType {
    WakesUp,
    FallsAsleep,
    GuardChange(GuardId),
}

impl EventType {
    fn new(event: &str) -> EventType {
        let guard = Regex::new(r"Guard #(\d+) begins shift").unwrap();
        match event {
            "falls asleep" => EventType::FallsAsleep,
            "wakes up" => EventType::WakesUp,
            _ => {
                let g = guard.captures(event).unwrap();
                EventType::GuardChange(g[1].parse().unwrap())
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Date {
    year: u16,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
}

impl Date {
    fn new(year: &str, month: &str, day: &str, hour: &str, minute: &str) -> Date {
        return Date {
            year: year.parse().unwrap(),
            month: month.parse().unwrap(),
            day: day.parse().unwrap(),
            hour: hour.parse().unwrap(),
            minute: minute.parse().unwrap(),
        }
    }

    fn hour_diff(&self, other: &Date) -> u32 {
        return (u32::from(other.hour) - u32::from(self.hour)) * 60 +
            u32::from(other.minute) - u32::from(self.minute);
    }
}

#[derive(Clone, Debug)]
struct Event {
    date: Date,
    event_type: EventType,
}

impl PartialOrd for Date {
    fn partial_cmp(&self, other: &Date) -> Option<Ordering> {
        return Some(self.cmp(&other));
    }
}

impl Ord for Date {
    fn cmp(&self, other: &Date) -> Ordering {
        if self.year != other.year { return self.year.cmp(&other.year) }
        if self.month != other.month { return self.month.cmp(&other.month) }
        if self.day != other.day { return self.day.cmp(&other.day) }
        if self.hour != other.hour { return self.hour.cmp(&other.hour) }
        if self.minute != other.minute { return self.minute.cmp(&other.minute) }
        return Ordering::Equal
    }
}

fn part1(guard_events: &HashMap<GuardId, Vec<(Date, Date)>>) -> u32 {
    let (guard, times) = guard_events
        .iter()
        .max_by(|(a, b)| {
            let mut total = 0;)
            for (start, stop) in times {
                total += start.hour_diff(&stop);
            }
            return total;
        })
        .unwrap();
    unimplemented!();
}

fn part2(events: &HashMap<GuardId, Vec<(Date, Date)>>) -> String {
    unimplemented!()
}

fn parse<'a>(lines: String) -> HashMap<GuardId, Vec<(Date, Date)>> {
    let event = Regex::new(r"\[(\d{4})\-(\d{2})\-(\d{2}) (\d{2}):(\d{2})\] (.*)$").unwrap();
    let mut events: Vec<Event> = Vec::new();
    for line in lines.trim().split('\n') {
        let m = event.captures(line).unwrap();
        events.push( Event {
            date: Date::new(&m[1], &m[2], &m[3], &m[4], &m[5]),
            event_type: EventType::new(&m[6]),
        })
    }
    events.sort_unstable_by_key(|a| a.date.clone());
    let mut result: HashMap<GuardId, Vec<(Date, Date)>> = HashMap::new();
    let mut guard: Option<GuardId> = None;
    let mut sleep_start: Option<Date> = None;
    for event in events {
        match event.event_type {
            EventType::GuardChange(id) => guard = Some(id),
            EventType::FallsAsleep => sleep_start = Some(event.date),
            EventType::WakesUp => {
                result.entry(guard.unwrap())
                   .or_insert(Vec::new())
                   .push((sleep_start.unwrap(), event.date));
                sleep_start = None;
            }
        }
    }
    return result;
}

fn main() {
    let input = parse(get_input(04, 2018));
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
