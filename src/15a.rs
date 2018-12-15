use std::io::{self, prelude::*};
use std::collections::VecDeque;
use std::cmp::{min, max};

const INF: usize = 99999;
const HP: i32 = 200;
const AP: i32 = 3;

#[derive(Debug, Copy, Clone)]
struct Unit {
    coord: (usize, usize),
    hp: i32,
    race: char,
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(Result::unwrap);

    let mut grid = vec![];
    let mut units = vec![];
    for line in lines {
        let mut row = vec![];
        for ch in line.chars() {
            if ch == 'E' || ch == 'G' {
                units.push(Unit {
                    coord: (row.len(), grid.len()),
                    hp: HP,
                    race: ch,
                });
            }
            row.push(ch);
        }
        grid.push(row);
    }

    println!("{}", simulate(grid, units));
}

fn simulate(mut grid: Vec<Vec<char>>, mut units: Vec<Unit>) -> i32 {
    let mut round_counter = 0;
    'outer: loop {
        // Sort units to determine turn order
        units.sort_by_key(|unit| (unit.coord.1, unit.coord.0));

        for i in 0..units.len() {
            if units[i].hp <= 0 {
                continue;
            }

            let mut coord = units[i].coord;
            let race = units[i].race;

            // Move
            {
                // Find all targets
                let targets = units.iter().filter(|other| other.hp > 0 && other.race != race);

                // Determine paths to attacking positions
                let mut in_range = vec![];
                for &target in targets {
                    in_range.extend_from_slice(&adjacent(target.coord));
                }

                // Combat ends when there are no more enemies
                if in_range.is_empty() {
                    break 'outer;
                }

                // Sort by distance, then by first step
                let (distances, dirs) = shortest_paths(&grid, coord);
                in_range.sort_by_key(|coord| (distances[coord.1][coord.0], dirs[coord.1][coord.0]));

                let target = in_range[0];
                let distance = distances[target.1][target.0];
                if distance > 0 && distance < INF {
                    // Perform step
                    grid[coord.1][coord.0] = '.';
                    coord = match dirs[target.1][target.0] {
                        0 => (coord.0, coord.1 - 1),
                        1 => (coord.0 - 1, coord.1),
                        2 => (coord.0 + 1, coord.1),
                        3 => (coord.0, coord.1 + 1),
                        _ => panic!(),
                    };
                    grid[coord.1][coord.0] = race;
                    units[i].coord = coord;
                }
            }

            // Attack
            {
                // Check for adjacent targets
                let mut targets: Vec<_> = units.iter_mut().filter(|other|
                    other.hp > 0 &&
                    other.race != race &&
                    distance(other.coord, coord) == 1
                ).collect();
                targets.sort_by_key(|other| other.hp);

                if let Some(target) = targets.get_mut(0) {
                    target.hp -= AP;
                    if target.hp <= 0 {
                        grid[target.coord.1][target.coord.0] = '.';
                    }
                }
            }
        }
        round_counter += 1;
    }

    let mut sum_hp = 0;
    for unit in units {
        if unit.hp > 0 {
            sum_hp += unit.hp;
        }
    }

    round_counter * sum_hp
}

fn adjacent(coord: (usize, usize)) -> [(usize, usize); 4] {
    [
        (coord.0, coord.1 - 1),
        (coord.0 - 1, coord.1),
        (coord.0 + 1, coord.1),
        (coord.0, coord.1 + 1),
    ]
}

fn distance(a: (usize, usize), b: (usize, usize)) -> usize {
    max(a.0, b.0) - min(a.0, b.0) +
    max(a.1, b.1) - min(a.1, b.1)
}

fn shortest_paths(grid: &Vec<Vec<char>>, start: (usize, usize)) -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
    let mut dists = vec![vec![INF; grid[0].len()]; grid.len()];
    let mut dirs = vec![vec![0; grid[0].len()]; grid.len()];

    // Breadth-first search
    let mut queue = VecDeque::new();
    queue.push_back((start, 0));
    dists[start.1][start.0] = 0;

    while !queue.is_empty() {
        let (coord, dist) = queue.pop_front().unwrap();

        for (dir, &adj) in adjacent(coord).iter().enumerate() {
            if grid[adj.1][adj.0] != '.' || dists[adj.1][adj.0] < INF {
                continue;
            }
            dists[adj.1][adj.0] = dist + 1;
            queue.push_back((adj, dist + 1));
            if coord == start {
                dirs[adj.1][adj.0] = dir;
            } else {
                dirs[adj.1][adj.0] = dirs[coord.1][coord.0];
            }
        }
    }

    (dists, dirs)
}
