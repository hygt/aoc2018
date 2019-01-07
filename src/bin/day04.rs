extern crate chrono;
extern crate regex;

use chrono::offset::FixedOffset;
use chrono::DateTime;
use chrono::Timelike;
use regex::Regex;
use std::collections::HashMap;
use std::fs;

const DELIM: usize = 20;

#[derive(Debug)]
enum Action {
    WakeUp(u32),
    FallAsleep(u32),
    BeginShift(u32),
}

fn main() {
    let re = Regex::new(r"^Guard #(\d+) begins shift$").unwrap();
    // the content was edited for convenience
    let input = fs::read_to_string("resources/day04.txt").expect("Couldn't read file");
    let mut actions: Vec<(DateTime<FixedOffset>, Action)> = input
        .lines()
        .into_iter()
        .map(|line| {
            let time = line.get(..DELIM).unwrap();
            let dt = DateTime::parse_from_rfc3339(time).unwrap();
            let action = line.get((DELIM + 1)..).unwrap();
            let ac = if action == "wakes up" {
                Action::WakeUp(dt.minute())
            } else if action == "falls asleep" {
                Action::FallAsleep(dt.minute())
            } else {
                let groups = re.captures(action).unwrap();
                let id = &groups[1].parse::<u32>().unwrap();
                Action::BeginShift(*id)
            };
            (dt, ac)
        })
        .collect();
    actions.sort_unstable_by_key(|(dt, _)| dt.timestamp());

    let mut guards: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut iterator = actions.iter();
    let mut guard_id: u32 = 0;
    let mut from: u32 = 0;
    while let Some((_, ac)) = iterator.next() {
        match ac {
            Action::BeginShift(id) => guard_id = *id,
            Action::FallAsleep(min) => from = *min,
            Action::WakeUp(to) => {
                let mut minutes = guards.entry(guard_id).or_insert(vec![0u32; 60]);
                for min in from..*to {
                    minutes[min as usize] += 1;
                }
            }
        }
    }

    let mut max: u32 = 0;
    let mut freq: u32 = 0;
    let mut minute: u32 = 0;

    let mut max_freq: u32 = 0;
    let mut max_freq_min: u32 = 0;
    let mut max_guard_id: u32 = 0;

    for (guard, minutes) in guards {
        // part one
        let sum = minutes.iter().sum();
        if sum > max {
            guard_id = guard;
            max = sum;
            freq = 0;
            minute = 0;
            for min in 0usize..60 {
                if minutes[min] > freq {
                    freq = minutes[min];
                    minute = min as u32;
                }
            }
        }
        // part two
        for min in 0usize..60 {
            if minutes[min] > max_freq {
                max_freq = minutes[min];
                max_guard_id = guard;
                max_freq_min = min as u32;
            }
        }
    }
    println!(
        "guard: {}, minute: {}, freq: {}, result: {}",
        guard_id,
        minute,
        freq,
        guard_id * minute
    );
    println!(
        "guard: {}, minute: {}, freq: {}, result: {}",
        max_guard_id,
        max_freq_min,
        max_freq,
        max_guard_id * max_freq_min
    );
}
