use std::io::{self, prelude::*};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(Result::unwrap);
    let line = lines.next().unwrap();
    let goal: Vec<_> = line.bytes().map(|b| b - b'0').collect();

    let mut recipes = vec![3u8, 7];
    let mut current = vec![0, 1];
    loop {
        // Add new recipes
        let sum = recipes[current[0]] + recipes[current[1]];
        if sum >= 10 {
            recipes.push(sum / 10);
        }
        recipes.push(sum % 10);

        // Advance elves
        for cur in &mut current {
            *cur = (*cur + 1 + recipes[*cur] as usize) % recipes.len();
        }

        // Check for matches on the last two "windows".
        // These should really be reversed once more to account for overlapping
        // matches, but that cannot happen for the given input.
        for (pos, window) in recipes.windows(goal.len())
                                    .enumerate().rev().take(2) {
            if window == goal.as_slice() {
                println!("{}", pos);
                return;
            }
        }
    }
}
