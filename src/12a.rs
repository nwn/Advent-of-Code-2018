use std::io::{self, prelude::*};
use std::collections::HashMap;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(Result::unwrap);

    const CONTEXT: usize = 5;
    const BOUNDS: usize = 300;
    const OFFSET: usize = 10;
    const GENERATIONS: usize = 20;

    let mut plants = vec![false; BOUNDS];
    let state = lines.next().unwrap().split_whitespace().last().unwrap().to_string();
    for (i, ch) in state.chars().enumerate() {
        plants[i + OFFSET] = ch == '#';
    }
    lines.next(); // Skip blank line

    let mut rules = HashMap::new();
    for line in lines {
        let words: Vec<_> = line.split_whitespace().collect();

        let mut from = [false; CONTEXT];
        for (i, ch) in words[0].chars().enumerate() {
            from[i] = ch == '#';
        }
        let to = words[2] == "#";

        rules.insert(from, to);
    }

    for _gen in 1..=GENERATIONS {
        let mut next_gen = vec![false; BOUNDS];
        for (mut i, window) in plants.windows(CONTEXT).enumerate() {
            i += CONTEXT / 2;
            next_gen[i] = rules[window];
        }
        plants = next_gen;
    }

    let mut sum = 0;
    for (i, b) in plants.into_iter().enumerate() {
        if b {
            sum += (i as isize) - (OFFSET as isize);
        }
    }
    println!("{}", sum);
}
