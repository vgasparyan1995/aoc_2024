use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    io::{stdin, BufRead},
};

type Mtx = Vec<Vec<u32>>;

fn get(mtx: &Mtx, r: i32, c: i32, r_max: i32, c_max: i32) -> u32 {
    if r < 0 || r >= r_max || c < 0 || c >= c_max {
        11
    } else {
        mtx[r as usize][c as usize]
    }
}

fn part1(mtx: &Mtx) -> usize {
    let r_max = mtx.len() as i32;
    let c_max = mtx[0].len() as i32;
    (0..r_max)
        .cartesian_product(0..c_max)
        .filter(|&(r, c)| get(mtx, r, c, r_max, c_max) == 0)
        .map(|pos| {
            (1..=9)
                .fold(HashSet::from([pos]), |positions, next_level| {
                    positions
                        .into_iter()
                        .map(|(r, c)| [(r + 1, c), (r - 1, c), (r, c + 1), (r, c - 1)].into_iter())
                        .flatten()
                        .filter(|&(r1, c1)| get(mtx, r1, c1, r_max, c_max) == next_level)
                        .collect()
                })
                .len()
        })
        .sum()
}

fn part2(mtx: &Mtx) -> usize {
    let r_max = mtx.len() as i32;
    let c_max = mtx[0].len() as i32;
    (0..r_max)
        .cartesian_product(0..c_max)
        .filter(|&(r, c)| get(mtx, r, c, r_max, c_max) == 0)
        .map(|pos| {
            (1..=9)
                .fold(HashMap::from([(pos, 1)]), |positions, next_level| {
                    positions
                        .into_iter()
                        .map(|((r, c), cnt)| {
                            [(r + 1, c), (r - 1, c), (r, c + 1), (r, c - 1)]
                                .into_iter()
                                .map(move |pos| (pos, cnt))
                        })
                        .flatten()
                        .filter(|&((r1, c1), _)| get(mtx, r1, c1, r_max, c_max) == next_level)
                        .fold(HashMap::new(), |mut trails, (pos, count)| {
                            *trails.entry(pos).or_insert(0) += count;
                            trails
                        })
                })
                .into_iter()
                .map(|(_, cnt)| cnt)
                .sum::<usize>()
        })
        .sum()
}

fn main() {
    let input = stdin()
        .lock()
        .lines()
        .map(Result::unwrap)
        .map(|line| line.chars().filter_map(|ch| ch.to_digit(10)).collect())
        .collect();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
