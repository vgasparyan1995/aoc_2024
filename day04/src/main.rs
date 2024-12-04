use anyhow::Result;
use itertools::Itertools;
use std::{
    io::{stdin, BufRead},
    iter,
};

fn get(mtx: &Vec<Vec<char>>, r: i32, c: i32, r_max: i32, c_max: i32) -> char {
    if r < 0 || r >= r_max || c < 0 || c >= c_max {
        ' '
    } else {
        mtx[r as usize][c as usize]
    }
}

fn part1(mtx: &Vec<Vec<char>>) -> usize {
    let dirs: Vec<(i32, i32)> = (-1..=1)
        .cartesian_product(-1..=1)
        .filter(|&(dr, dc)| !(dr == 0 && dc == 0))
        .collect();

    let r_max = mtx.len() as i32;
    let c_max = mtx[0].len() as i32;
    (0..r_max)
        .cartesian_product(0..c_max)
        .map(|(r, c)| {
            dirs.iter()
                .map(|&(dr, dc)| {
                    let (mut r, mut c) = (r, c);
                    iter::from_fn(|| {
                        let result = (r, c);
                        r += dr;
                        c += dc;
                        Some(result)
                    })
                    .take(4)
                    .map(|(r, c)| get(mtx, r, c, r_max, c_max))
                    .collect::<String>()
                })
                .filter(|str| str == "XMAS")
                .count()
        })
        .sum()
}

fn part2(mtx: &Vec<Vec<char>>) -> usize {
    let r_max = mtx.len() as i32;
    let c_max = mtx[0].len() as i32;
    (0..r_max)
        .cartesian_product(0..c_max)
        .filter(|&(r, c)| {
            let word1 = [(-1, -1), (0, 0), (1, 1)]
                .into_iter()
                .map(|(dr, dc)| get(mtx, r + dr, c + dc, r_max, c_max))
                .collect::<String>();
            let word2 = [(-1, 1), (0, 0), (1, -1)]
                .into_iter()
                .map(|(dr, dc)| get(mtx, r + dr, c + dc, r_max, c_max))
                .collect::<String>();
            (word1 == "MAS" || word1 == "SAM") && (word2 == "MAS" || word2 == "SAM")
        })
        .count()
}

fn main() {
    let input = stdin()
        .lock()
        .lines()
        .map(Result::unwrap)
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
}
