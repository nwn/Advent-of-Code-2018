use std::io::{self, prelude::*};
fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(Result::unwrap);
    println!("{}", lines.map(|line| line.parse::<i32>().unwrap()).sum::<i32>());
}
