extern crate regex;

use std::io::{self, prelude::*};

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(Result::unwrap);
    let regex = regex::Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();

    let mut grid = vec![vec![0; 1000]; 1000];
    for line in lines {
        let caps = regex.captures(&line).unwrap();
        let _id = caps[1].parse::<usize>().unwrap();
        let x = caps[2].parse::<usize>().unwrap();
        let y = caps[3].parse::<usize>().unwrap();
        let w = caps[4].parse::<usize>().unwrap();
        let h = caps[5].parse::<usize>().unwrap();

        for row in y..y+h {
            for col in x..x+w {
                grid[row][col] += 1;
            }
        }
    }

    println!("{}", grid.iter().flat_map(|row| row.iter().filter(|&&cell| cell > 1)).count());
}
