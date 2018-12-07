use std::io::{self, prelude::*};
use std::collections::{HashMap, BinaryHeap};

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(Result::unwrap);

    let mut in_edges = HashMap::new();
    let mut out_edges = HashMap::new();
    for ch in b'A' ..= b'Z' {
        in_edges.insert(ch, vec![]);
        out_edges.insert(ch, vec![]);
    }

    for line in lines {
        let mut words: Vec<_> = line.split_whitespace().collect();
        let from = words[1].as_bytes()[0];
        let to = words[7].as_bytes()[0];

        in_edges.get_mut(&to).unwrap().push(from);
        out_edges.get_mut(&from).unwrap().push(to);
    }

    let mut ready = BinaryHeap::new();
    for (to, in_edges) in in_edges.iter() {
        if in_edges.is_empty() {
            ready.push(neg(*to));
        }
    }

    let mut order = String::new();
    while !ready.is_empty() {
        let next = neg(ready.pop().unwrap());
        order.push(next as char);

        for to in &out_edges[&next] {
            in_edges.get_mut(to).unwrap().retain(|edge| *edge != next);
            if in_edges[to].is_empty() {
                ready.push(neg(*to));
            }
        }
    }
    println!("{}", order);
}

fn neg(b: u8) -> u8 {
    255 - b
}
