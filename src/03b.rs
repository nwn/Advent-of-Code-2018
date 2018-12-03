extern crate regex;

use std::io::{self, prelude::*};
use std::collections::HashSet;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(Result::unwrap);
    let regex = regex::Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();

    let mut viable = HashSet::new();
    let mut grid = vec![vec![HashSet::new(); 1000]; 1000];
    for line in lines {
        let caps = regex.captures(&line).unwrap();
        let id = caps[1].parse::<usize>().unwrap();
        let x = caps[2].parse::<usize>().unwrap();
        let y = caps[3].parse::<usize>().unwrap();
        let w = caps[4].parse::<usize>().unwrap();
        let h = caps[5].parse::<usize>().unwrap();

        viable.insert(id);
        for row in y..y+h {
            for col in x..x+w {
                grid[row][col].insert(id);
                if grid[row][col].len() > 1 {
                    for id in grid[row][col].iter() {
                        viable.remove(id);
                    }
                }
            }
        }
    }

    println!("{}", viable.iter().next().unwrap());
}
