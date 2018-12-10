extern crate regex;

use std::io::{self, prelude::*};

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(Result::unwrap);

    let regex_number = regex::Regex::new(r"(-?\d+)").unwrap();

    let mut particles = vec![];
    for line in lines {
        let nums: Vec<_> = regex_number.captures_iter(&line)
                                       .map(|cap| cap[1].parse::<i32>().unwrap())
                                       .collect();

        let pos = (nums[0], nums[1]);
        let vel = (nums[2], nums[3]);

        particles.push((pos, vel));
    }

    // Iterate until the bounding max begins growing again
    for time in 0.. {
        let (min_x, max_x, min_y, max_y) = min_max(particles.iter().map(|(pos, _)| pos).cloned());
        let cur_size = (max_y - min_y + 1, max_x - min_x + 1);

        let mut new_particles = particles.clone();
        for (pos, vel) in &mut new_particles {
            pos.0 += vel.0;
            pos.1 += vel.1;
        }
        let (min_x, max_x, min_y, max_y) = min_max(new_particles.iter().map(|(pos, _)| pos).cloned());
        let next_size = (max_y - min_y + 1, max_x - min_x + 1);

        if next_size > cur_size {
            println!("{}", time);
            break;
        } else {
            particles = new_particles;
        }
    }
}

fn min_max(pos: impl Iterator<Item = (i32,i32)>) -> (i32, i32, i32, i32) {
    let (mut min_x, mut max_x, mut min_y, mut max_y) = (1000, -1000, 1000, -1000);
    for (x, y) in pos {
        if x < min_x { min_x = x; }
        if x > max_x { max_x = x; }
        if y < min_y { min_y = y; }
        if y > max_y { max_y = y; }
    }

    (min_x, max_x, min_y, max_y)
}
