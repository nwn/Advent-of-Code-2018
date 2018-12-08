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

    let mut children = vec![];
    for _ in 0..num_children {
        children.push(recur(nums, idx));
    }

    let mut metadata = vec![];
    for _ in 0..num_metadata {
        metadata.push(nums[*idx]);
        *idx += 1;
    }

    if num_children > 0 {
        let mut value = 0;
        for &idx in &metadata {
            if idx > 0 && idx <= children.len() {
                value += children[idx - 1];
            }
        }
        value
    } else {
        metadata.iter().sum()
    }
}
