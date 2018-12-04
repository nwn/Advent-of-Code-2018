extern crate regex;

use std::io::{self, prelude::*};
use std::collections::HashMap;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(Result::unwrap);
    let mut lines: Vec<_> = lines.collect();
    lines.sort();

    let regex_number = regex::Regex::new(r"(\d+)").unwrap();
    let regex_start = regex::Regex::new(r"begins shift").unwrap();
    let regex_sleep = regex::Regex::new(r"falls asleep").unwrap();
    let regex_wake = regex::Regex::new(r"wakes up").unwrap();

    // Track each guard's sleep ranges
    let mut sleep_ranges = HashMap::new();
    let mut current_guard = 0;
    let mut start_sleep = 0;
    for line in lines {
        let mut caps: Vec<_> = regex_number.captures_iter(&line).map(|cap| cap[1].parse::<usize>().unwrap()).collect();
        let mut time = ((caps[3] + 12) % 24) * 60 + caps[4];

        if regex_start.captures(&line).is_some() {
            current_guard = caps[5];
        } else if regex_sleep.captures(&line).is_some() {
            start_sleep = time;
        } else if regex_wake.captures(&line).is_some() {
            sleep_ranges.entry(current_guard).or_insert(vec![]).push((start_sleep, time));
        } else {
            panic!("Unable to parse input");
        }
    }

    // Find the minute that a specific guard slept through the most
    let mut most = (0, 0, 0); // (days, minute, guard)
    for (guard, ranges) in &sleep_ranges {
        let mut days_asleep = HashMap::new();
        for range in ranges {
            for minute in range.0 .. range.1 {
                *days_asleep.entry(minute).or_insert(0) += 1;
            }
        }

        let (days, minute) = days_asleep.iter().map(|(m, d)| (d, m)).max().unwrap();
        let cand = (*days, *minute, *guard);
        if cand > most {
            most = cand;
        }
    }
    let guard = most.2;
    let minute = most.1 % 60;
    println!("{}", guard * minute);
}
