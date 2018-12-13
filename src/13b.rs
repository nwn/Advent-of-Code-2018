use std::io::{self, prelude::*};

#[derive(Copy, Clone)]
struct Cart {
    coord: (usize, usize),
    travel: Dir,
    turns: u32,
}

#[derive(Copy, Clone)]
enum Dir {
    Left, Right, Up, Down,
}
use Dir::*;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(Result::unwrap);

    let mut carts = vec![];
    let mut grid = vec![];
    for line in lines {
        let mut row = vec![];
        for ch in line.chars() {
            let coord = (grid.len(), row.len());
            let turns = 0;
            row.push(match ch {
                '<' => {
                    carts.push(Cart { coord, turns, travel: Left });
                    b'-'
                },
                '>' => {
                    carts.push(Cart { coord, turns, travel: Right });
                    b'-'
                },
                '^' => {
                    carts.push(Cart { coord, turns, travel: Up });
                    b'|'
                },
                'v' => {
                    carts.push(Cart { coord, turns, travel: Down });
                    b'|'
                },
                _ => {
                    ch as u8
                },
            });
        }
        grid.push(row);
    }

    loop {
        // Sort in reverse since we'll pop from the end
        carts.sort_by(|a, b| b.coord.cmp(&a.coord));

        let mut new_carts = vec![];
        while let Some(mut cart) = carts.pop() {
            // Turn cart if necessary
            match grid[cart.coord.0][cart.coord.1] {
                b'+' => {
                    match (cart.travel, cart.turns % 3) {
                        (Left,  0) => cart.travel = Down,
                        (Left,  1) => cart.travel = Left,
                        (Left,  2) => cart.travel = Up,
                        (Right, 0) => cart.travel = Up,
                        (Right, 1) => cart.travel = Right,
                        (Right, 2) => cart.travel = Down,
                        (Up,    0) => cart.travel = Left,
                        (Up,    1) => cart.travel = Up,
                        (Up,    2) => cart.travel = Right,
                        (Down,  0) => cart.travel = Right,
                        (Down,  1) => cart.travel = Down,
                        (Down,  2) => cart.travel = Left,
                        (_, _) => unreachable!(),
                    }
                    cart.turns += 1;
                },
                b'/' => {
                    match cart.travel {
                        Left  => cart.travel = Down,
                        Right => cart.travel = Up,
                        Up    => cart.travel = Right,
                        Down  => cart.travel = Left,
                    }
                },
                b'\\' => {
                    match cart.travel {
                        Left  => cart.travel = Up,
                        Right => cart.travel = Down,
                        Up    => cart.travel = Left,
                        Down  => cart.travel = Right,
                    }
                },
                _ => (),
            }

            // Advance cart
            match cart.travel {
                Left  => cart.coord.1 -= 1,
                Right => cart.coord.1 += 1,
                Up    => cart.coord.0 -= 1,
                Down  => cart.coord.0 += 1,
            }

            // Check for collisions
            if carts.iter().chain(new_carts.iter()).any(|other| other.coord == cart.coord) {
                carts.retain(|other| other.coord != cart.coord);
                new_carts.retain(|other| other.coord != cart.coord);
            } else {
                new_carts.push(cart);
            }
        }

        carts = new_carts;
        if carts.len() == 1 {
            let cart = carts.pop().unwrap();
            println!("{},{}", cart.coord.1, cart.coord.0);
            return;
        }
    }
}
