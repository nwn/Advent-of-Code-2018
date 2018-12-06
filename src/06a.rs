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

    println!("{}", smallest_region(&locs, (0, 0), (400, 400)));
}

fn smallest_region(locs: &[(i32,i32)], low: (i32,i32), high: (i32, i32)) -> isize {
    let mut counts = vec![0; locs.len()];

    for x in low.0 ..= high.0 {
        for y in low.1 ..= high.1 {
            let mut dists = Vec::with_capacity(locs.len());
            for (i, loc) in locs.iter().enumerate() {
                dists.push(((loc.0 - x).abs() + (loc.1 - y).abs(), i));
            }
            dists.sort();

            // Only count if not tied
            if dists[0].0 != dists[1].0 {
                if x == low.0 || x == high.0 || y == low.1 || y == high.1 {
                    // Disqualify those on the outer edge
                    counts[dists[0].1] = -1_000_000;
                } else {
                    counts[dists[0].1] += 1;
                }
            }
        }
    }

    *counts.iter().max().unwrap()
}
