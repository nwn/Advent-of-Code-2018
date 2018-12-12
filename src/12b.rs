use std::io::{self, prelude::*};
use std::collections::HashMap;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(Result::unwrap);

    const CONTEXT: usize = 5;
    const BOUNDS: usize = 300;
    const OFFSET: usize = 10;
    const GENERATIONS: usize = 50_000_000_000;

    let mut plants = vec![false; BOUNDS];
    let state = lines.next().unwrap().split_whitespace().last().unwrap().to_string();
    for (i, ch) in state.chars().enumerate() {
        plants[i + OFFSET] = ch == '#';
    }
    lines.next(); // Skip blank line

    let mut rules = HashMap::new();
    for line in lines {
        let words: Vec<_> = line.split_whitespace().collect();

        let mut from = [false; CONTEXT];
        for (i, ch) in words[0].chars().enumerate() {
            from[i] = ch == '#';
        }
        let to = words[2] == "#";

        rules.insert(from, to);
    }

    let mut visited = HashMap::new();
    for gen in 1..=GENERATIONS {
        let mut next_gen = vec![false; BOUNDS];

        let mut sum = 0;
        let mut min = None;
        let mut max = None;
        for (mut i, window) in plants.windows(CONTEXT).enumerate() {
            i += CONTEXT / 2;
            next_gen[i] = rules[window];
            if next_gen[i] {
                sum += (i as isize) - (OFFSET as isize);
                min = min.or(Some(i));
                max = Some(i);
            }
        }
        plants = next_gen;

        let min = *min.get_or_insert(0);
        let max = *max.get_or_insert(0);
        let key = plants[min..=max].to_owned();
        let val = (gen, sum);
        if let Some((last_gen, last_sum)) = visited.insert(key, val) {
            let period = gen - last_gen;
            let gens_to_go = GENERATIONS - gen;
            if gens_to_go % period != 0 {
                // Continue if we're not an even number of periods away.
                // Since we are repeating from this point, that should happen soon.
                continue;
            }

            let periods_to_go = gens_to_go / period;
            let sum_diff = sum - last_sum;
            let prediction = sum + sum_diff * periods_to_go as isize;
            println!("{}", prediction);
            break;
        }
    }
}
