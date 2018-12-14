use std::io::{self, prelude::*};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(Result::unwrap);
    let line = lines.next().unwrap();
    let goal = line.parse::<usize>().unwrap();

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

        if recipes.len() >= goal + 10 {
            for score in &recipes[goal..goal+10] {
                print!("{}", score);
            }
            println!("");
            break;
        }
    }
}
