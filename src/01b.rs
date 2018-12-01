use std::io::{self, prelude::*};
use std::collections::HashSet;
fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(Result::unwrap);

    let diffs: Vec<_> = lines.map(|line| line.parse::<i32>().unwrap()).collect();

    let mut freqs = HashSet::new();
    freqs.insert(0);
    let mut sum = 0;
    for diff in diffs.iter().cycle() {
        sum += diff;
        if !freqs.insert(sum) {
            println!("{}", sum);
            break;
        }
    }
}
