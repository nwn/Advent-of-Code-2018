extern crate regex;
extern crate linked_list;

use std::io::{self, prelude::*};
use linked_list::{LinkedList, Cursor};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(Result::unwrap);

    let regex_number = regex::Regex::new(r"(\d+)").unwrap();
    let nums: Vec<_> = regex_number.captures_iter(&lines.next().unwrap())
                                   .map(|cap| cap[1].parse::<usize>().unwrap())
                                   .collect();

    let num_players = nums[0];
    let num_marbles = nums[1];

    let mut points = vec![0; num_players];
    let mut circle = LinkedList::new();
    circle.push_back(0);

    let mut cur = circle.cursor();
    for marble in 1..=num_marbles {
        if marble % 23 == 0 {
            let cur_player = (marble - 1) % num_players;
            points[cur_player] += marble;

            seek(&mut cur, -7);
            points[cur_player] += cur.remove().unwrap();
        } else {
            seek(&mut cur, 2);
            cur.insert(marble);
        }
    }
    println!("{}", points.iter().max().unwrap());
}

// Skip the sentinel object when wrapping around the list
fn seek<T>(cursor: &mut Cursor<T>, n: isize) {
    for _ in 0..n.abs() {
        if n > 0 {
            if cursor.next().is_none() {
                cursor.next();
            }
        } else {
            if cursor.prev().is_none() {
                cursor.prev();
            }
        }
    }
}
