use itertools::Itertools;
use std::{
    collections::HashSet,
    io::{stdin, BufRead},
};

type Mtx = Vec<Vec<char>>;

fn get(mtx: &Mtx, r: i32, c: i32, r_max: i32, c_max: i32) -> Option<char> {
    if r < 0 || r >= r_max || c < 0 || c >= c_max {
        None
    } else {
        Some(mtx[r as usize][c as usize])
    }
}

fn turn_right((dr, dx): (i32, i32)) -> (i32, i32) {
    match (dr, dx) {
        (-1, 0) => (0, 1),
        (0, 1) => (1, 0),
        (1, 0) => (0, -1),
        (0, -1) => (-1, 0),
        _ => panic!(),
    }
}

fn part1(mtx: &Mtx) -> Option<usize> {
    let r_max = mtx.len() as i32;
    let c_max = mtx[0].len() as i32;
    let (mut r, mut c) = (0..r_max)
        .cartesian_product(0..c_max)
        .find(|&(r, c)| get(mtx, r, c, r_max, c_max) == Some('^'))?;
    let (mut dr, mut dc) = (-1, 0);
    let mut positions = HashSet::new();
    loop {
        positions.insert((r, c));
        while let Some('#') = get(mtx, r + dr, c + dc, r_max, c_max) {
            (dr, dc) = turn_right((dr, dc));
        }
        if let None = get(mtx, r + dr, c + dc, r_max, c_max) {
            break;
        }
        (r, c) = (r + dr, c + dc);
    }
    Some(positions.len())
}

fn main() {
    let input = stdin()
        .lock()
        .lines()
        .map(Result::unwrap)
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Mtx>();

    println!("Part1: {:?}", part1(&input));
    // println!("Part2: {}", part2(&input));
}
