use itertools::Itertools;
use std::ops::{Add, Mul, Sub};
use std::{
    collections::{HashMap, HashSet},
    io::{stdin, BufRead},
};

type Mtx = Vec<Vec<char>>;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
struct Pos {
    r: i32,
    c: i32,
}

impl Add for Pos {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Pos {
            r: self.r + other.r,
            c: self.c + other.c,
        }
    }
}

impl Sub for Pos {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            r: self.r - other.r,
            c: self.c - other.c,
        }
    }
}

impl Mul<i32> for Pos {
    type Output = Self;
    fn mul(self, s: i32) -> Self {
        Self {
            r: self.r * s,
            c: self.c * s,
        }
    }
}

fn part1(mtx: &Mtx) -> usize {
    let r_max = mtx.len();
    let c_max = mtx[0].len();
    (0..r_max)
        .cartesian_product(0..c_max)
        .filter_map(|(r, c)| match mtx[r][c] {
            '.' => None,
            ch => Some((
                ch,
                Pos {
                    r: r as i32,
                    c: c as i32,
                },
            )),
        })
        .fold(HashMap::new(), |mut groups, (freq, pos)| {
            groups.entry(freq).or_insert(Vec::new()).push(pos);
            groups
        })
        .into_iter()
        .map(|(_, positions)| {
            positions
                .iter()
                .cartesian_product(positions.iter())
                .filter(|(p1, p2)| p1 != p2)
                .map(|(&p1, &p2)| [p1 + p1 - p2, p2 + p2 - p1].into_iter())
                .flatten()
                .collect::<Vec<_>>()
        })
        .flatten()
        .filter(|pos| pos.r >= 0 && pos.r < r_max as i32 && pos.c >= 0 && pos.c < c_max as i32)
        .fold(HashSet::new(), |mut antinodes, pos| {
            antinodes.insert(pos);
            antinodes
        })
        .len()
}

fn part2(mtx: &Mtx) -> usize {
    let r_max = mtx.len();
    let c_max = mtx[0].len();
    (0..r_max)
        .cartesian_product(0..c_max)
        .filter_map(|(r, c)| match mtx[r][c] {
            '.' => None,
            ch => Some((
                ch,
                Pos {
                    r: r as i32,
                    c: c as i32,
                },
            )),
        })
        .fold(HashMap::new(), |mut groups, (freq, pos)| {
            groups.entry(freq).or_insert(Vec::new()).push(pos);
            groups
        })
        .into_iter()
        .map(|(_, positions)| {
            let within_bounds =
                |&p: &Pos| p.r >= 0 && p.r < r_max as i32 && p.c >= 0 && p.c < c_max as i32;
            positions
                .iter()
                .cartesian_product(positions.iter())
                .filter(|(p1, p2)| p1 != p2)
                .map(|(p1, p2)| {
                    (0..)
                        .map(|n| *p1 + (*p1 - *p2) * n)
                        .take_while(within_bounds)
                        .chain((0..).map(|n| *p2 + (*p2 - *p1) * n).take_while(within_bounds))
                })
                .flatten()
                .collect::<Vec<_>>()
        })
        .flatten()
        .fold(HashSet::new(), |mut antinodes, pos| {
            antinodes.insert(pos);
            antinodes
        })
        .len()
}

fn main() {
    let input = stdin()
        .lock()
        .lines()
        .map(Result::unwrap)
        .map(|line| line.chars().collect())
        .collect();

    println!("Part1: {:?}", part1(&input));
    println!("Part2: {:?}", part2(&input));
}
