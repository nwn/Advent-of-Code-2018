use std::io::{self, prelude::*};
fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(Result::unwrap);

    let lines: Vec<_> = lines.collect();
    for i in 0..lines.len() {
        for j in i+1 .. lines.len() {
            for k in 0..lines[i].len() {
                if &lines[i][..k] == &lines[j][..k] && &lines[i][k+1..] == &lines[j][k+1..] {
                    println!("{}{}", &lines[i][..k], &lines[i][k+1..]);
                    break;
                }
            }
        }
    }
}
