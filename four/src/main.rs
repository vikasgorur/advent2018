#[macro_use] extern crate lazy_static;
extern crate regex;

use std::fs::File;
use std::io::{BufReader, BufRead};
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
enum Action {
    FallsAsleep,
    WakesUp,
    BeginsShift(u32)
}

#[derive(Debug)]
struct Event {
    minute: u32,
    action: Action
}

fn parse_event(line: &str) -> Event {
    lazy_static! {
        static ref TIMESTAMP: Regex = Regex::new(r"\[(\d+)-(\d+)-(\d+) (\d\d):(\d\d)\] (.*)").unwrap();
    }
    let caps = TIMESTAMP.captures(line).unwrap();
    let minute = caps.get(5).unwrap().as_str().parse::<u32>().unwrap();

    let action_str = caps.get(6).unwrap().as_str();
    lazy_static! {
        static ref ACTION_STR: Regex = Regex::new(
            r"Guard #(\d+) begins shift"
        ).unwrap();
    }
    let action = if action_str == "wakes up" {
        Action::WakesUp
    } else if action_str == "falls asleep" {
        Action::FallsAsleep
    } else {
        let caps = ACTION_STR.captures(action_str).unwrap();
        Action::BeginsShift(caps.get(1).unwrap().as_str().parse::<u32>().unwrap())
    };

    Event {
        minute,
        action
    }
}

fn parse_events() -> Vec<Event> {
    let input = File::open("sorted.txt").unwrap();
    let reader = BufReader::new(input);

    let mut events = Vec::with_capacity(1100);
    for line in reader.lines() {
        events.push(parse_event(&line.unwrap()));
    }

    events
}

type Schedule = Vec<u32>;

fn process_events(events: &Vec<Event>) -> HashMap<u32, Schedule> {
    let mut guards: HashMap<u32, Schedule> = HashMap::new();

    let mut current_guard = 0;
    let mut current_asleep_min = 0;

    for e in events {
        match &e.action {
            &Action::BeginsShift(guard_id) => {
                current_guard = guard_id
            },
            &Action::FallsAsleep => { current_asleep_min = e.minute },
            &Action::WakesUp => {
                let guard = guards.entry(current_guard).or_insert(vec![0; 60]);
                for i in (current_asleep_min as usize)..(e.minute as usize) {
                    guard[i] += 1;
                }
            }
        }
    }
    guards
}

// returns (guard_id, minute)
fn laziest_guard(guards: &HashMap<u32, Schedule>) -> (u32, u32) {
    let mut guard_id = 0;
    let mut guard_max_asleep = 0;
    let mut worst_minute = 0;

    let keys = guards.keys();
    for id in keys {
        let schedule = guards.get(id).unwrap();
        // "most minutes asleep" means across ALL days
        let asleep: u32 = schedule.iter().sum();
        if asleep > guard_max_asleep {
            guard_id = *id;
            guard_max_asleep = asleep;
            let max: u32 = *schedule.iter().max().unwrap();
            worst_minute = schedule.iter().position(|m| *m == max).unwrap();
        }
    }

    (guard_id, worst_minute as u32)
}

fn part1(guards: &HashMap<u32, Schedule>) {
    let (guard_id, minute) = laziest_guard(guards);
    println!("{} {}", guard_id, minute);
    println!("{}", guard_id * minute);
}

// returns (guard_id, minute)
fn laziest_minute(guards: &HashMap<u32, Schedule>) -> (u32, u32) {
    let mut guard_id = 0;
    let mut worst_minute_sum = 0;
    let mut worst_minute = 0;

    let keys = guards.keys();
    for id in keys {
        let schedule = guards.get(id).unwrap();
        let asleep: u32 = *schedule.iter().max().unwrap();
        if asleep > worst_minute_sum {
            guard_id = *id;
            worst_minute_sum = asleep;
            worst_minute = schedule.iter().position(|m| *m == asleep).unwrap();
        }
    }

    (guard_id, worst_minute as u32)
}

fn part2(guards: &HashMap<u32, Schedule>) {
    let (guard_id, minute) = laziest_minute(guards);
    println!("{} {}", guard_id, minute);
    println!("{}", guard_id * minute);
}

fn main() {
    let events = parse_events();
    let guards = process_events(&events);
    //part1(&guards);
    part2(&guards);
}
