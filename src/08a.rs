extern crate regex;

use std::io::{self, prelude::*};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(Result::unwrap);

    let regex_number = regex::Regex::new(r"(\d+)").unwrap();
    let nums: Vec<_> = regex_number.captures_iter(&lines.next().unwrap())
                                   .map(|cap| cap[1].parse::<usize>().unwrap())
                                   .collect();

    println!("{}", recur(&nums, &mut 0));
}

fn recur(nums: &[usize], idx: &mut usize) -> usize {
    let num_children = nums[*idx];
    *idx += 1;
    let num_metadata = nums[*idx];
    *idx += 1;

    let mut total = 0;

    for _ in 0..num_children {
        total += recur(nums, idx);
    }

    for _ in 0..num_metadata {
        total += nums[*idx];
        *idx += 1;
    }

    total
}
