use std::io::{self, prelude::*};
use std::collections::BinaryHeap;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(Result::unwrap);

    let mut adj = vec![vec![false; 26]; 26];
    for line in lines {
        let mut words: Vec<_> = line.split_whitespace().collect();
        let from = (words[1].as_bytes()[0] - b'A') as usize;
        let to = (words[7].as_bytes()[0] - b'A') as usize;

        adj[to][from] = true;
    }

    let mut ready = BinaryHeap::new();
    for (to, in_edges) in adj.iter().enumerate() {
        if in_edges.iter().all(|b| !b) {
            ready.push(neg(to));
        }
    }

    let num_workers = 5;
    let mut workers = vec![];
    for time in 0.. {
        for (job, secs) in &mut workers {
            *secs -= 1;
            if *secs == 0 {
                for (to, in_edges) in adj.iter_mut().enumerate() {
                    if in_edges[*job] == false { continue; }

                    in_edges[*job] = false;
                    if in_edges.iter().all(|b| !b) {
                        ready.push(neg(to));
                    }
                }
            }
        }
        workers.retain(|(_job, secs)| *secs > 0);

        while !ready.is_empty() && workers.len() < num_workers {
            let next = neg(ready.pop().unwrap());
            workers.push((next, next + 61));
        }

        if workers.is_empty() {
            println!("{}", time);
            break;
        }
    }
}

fn neg(b: usize) -> usize {
    25 - b
}
