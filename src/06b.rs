extern crate regex;

use std::io::{self, prelude::*};

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(Result::unwrap);

    let regex_number = regex::Regex::new(r"(\d+)").unwrap();
    let mut locs = vec![];
    for line in lines {
        let mut nums: Vec<_> = regex_number.captures_iter(&line).map(|cap| cap[1].parse::<i32>().unwrap()).collect();
        locs.push((nums[0], nums[1]));
    }

    println!("{}", count_in_range(&locs, (0, 0), (400, 400)));
}

fn count_in_range(locs: &[(i32,i32)], low: (i32,i32), high: (i32, i32)) -> usize {
    let mut count = 0;
    for x in low.0 ..= high.0 {
        for y in low.1 ..= high.1 {
            let mut total_dist = 0;
            for loc in locs {
                total_dist += (loc.0 - x).abs() + (loc.1 - y).abs();
            }

            if total_dist < 10_000 {
                count += 1;
            }
        }
    }

    count
}
