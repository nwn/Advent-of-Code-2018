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

    // Track each guard's minutes asleep and the sleep ranges
    let mut time_asleep = HashMap::new();
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
            *time_asleep.entry(current_guard).or_insert(0) += time - start_sleep;
            sleep_ranges.entry(current_guard).or_insert(vec![]).push((start_sleep, time));
        } else {
            panic!("Unable to parse input");
        }
    }

    // Find which guard slept the most
    let guard = time_asleep.iter().map(|(guard, minutes)| (minutes, guard)).max().unwrap().1;

    // Find the minute that the guard spent the most days asleep during
    let mut days_asleep = HashMap::new();
    for range in &sleep_ranges[guard] {
        for minute in range.0 .. range.1 {
            *days_asleep.entry(minute).or_insert(0) += 1;
        }
    }
    let minute = days_asleep.iter().map(|(m, d)| (d, m)).max().unwrap().1 % 60;

    println!("{}", guard * minute);
}
