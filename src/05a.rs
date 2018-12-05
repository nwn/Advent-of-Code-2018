use std::io::{self, prelude::*};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(Result::unwrap);
    let line = lines.next().unwrap();

    println!("{}", react(line));
}

fn react(mut line: String) -> usize {
    loop {
        let mut done = true;
        for (i, pair) in line.clone().as_bytes().windows(2).enumerate() {
            if pair[0] != pair[1] && pair[0].to_ascii_lowercase() == pair[1].to_ascii_lowercase() {
                line.remove(i);
                line.remove(i);
                done = false;
                break;
            }
        }

        if done {
            break;
        }
    }
    line.len()
}
