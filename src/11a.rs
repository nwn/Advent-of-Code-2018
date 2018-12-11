use std::io::{self, prelude::*};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(Result::unwrap);
    let line = lines.next().unwrap();

    let serial = line.parse::<i32>().unwrap();

    // Calculate the power at each point
    const DIM: usize = 300;
    let mut grid = [[0; DIM]; DIM];
    for y in 0..DIM {
        for x in 0..DIM {
            grid[y][x] = power(serial, x + 1, y + 1);
        }
    }

    // Find the 3x3 subsquare with the greatest sum
    let mut max = -99999;
    let mut answer = (0, 0);
    for y in 0..DIM - 3 {
        for x in 0..DIM - 3 {
            let mut power = 0;
            for y in y..y+3 {
                for x in x..x+3 {
                    power += grid[y][x];
                }
            }

            if power > max {
                max = power;
                answer = (x, y);
            }
        }
    }

    println!("{},{}", answer.0 + 1, answer.1 + 1);
}

fn power(serial: i32, x: usize, y: usize) -> i32 {
    let (x, y) = (x as i32, y as i32);
    let rack_id = x + 10;

    let mut power = rack_id * y;
    power += serial;
    power *= rack_id;
    power /= 100;
    power %= 10;
    power -= 5;

    power
}
