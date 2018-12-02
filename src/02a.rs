use std::io::{self, prelude::*};
use std::collections::HashMap;
fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(Result::unwrap);

    let mut sum_2 = 0;
    let mut sum_3 = 0;
    for line in lines {
        let mut seen = HashMap::new();
        for byte in line.bytes() {
            let count = seen.entry(byte).or_insert(0);
            *count += 1;
        }

        let mut ch_2 = 0;
        let mut ch_3 = 0;
        for &v in seen.values() {
            if v == 2 { ch_2 = 1; }
            if v == 3 { ch_3 = 1; }
        }

        sum_2 += ch_2;
        sum_3 += ch_3;
    }

    println!("{}", sum_2 * sum_3);
}
