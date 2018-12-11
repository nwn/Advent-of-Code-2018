use std::io::{self, prelude::*};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(Result::unwrap);
    let line = lines.next().unwrap();

    let serial = line.parse::<i32>().unwrap();

    // Calculate grid of partial sums from the top-left corner
    const DIM: usize = 300;
    let mut grid = [[0; DIM]; DIM];
    for y in 0..DIM {
        for x in 0..DIM {
            grid[y][x] = power(serial, x + 1, y + 1);

            if x > 0 {
                grid[y][x] += grid[y][x - 1];
            }
            if y > 0 {
                grid[y][x] += grid[y - 1][x];
            }
            if x > 0 && y > 0 {
                grid[y][x] -= grid[y - 1][x - 1];
            }
        }
    }

    // Find the subsquare with the greatest sum
    let mut max = -99999;
    let mut answer = (0, 0, 0);
    for size in 1..=DIM {
        for y in size-1..DIM {
            for x in size-1..DIM {
                let mut power = grid[y][x];

                if x >= size {
                    power -= grid[y][x - size];
                }
                if y >= size {
                    power -= grid[y - size][x];
                }
                if x >= size && y >= size {
                    power += grid[y - size][x - size];
                }

                if power > max {
                    max = power;
                    answer = (x + 1 - size, y + 1 - size, size);
                }
            }
        }
    }

    println!("{},{},{}", answer.0 + 1, answer.1 + 1, answer.2);
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
