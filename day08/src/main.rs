use itertools::Itertools;
use std::ops::{Add, Sub};
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

fn main() {
    let input = stdin()
        .lock()
        .lines()
        .map(Result::unwrap)
        .map(|line| line.chars().collect())
        .collect();

    println!("Part1: {:?}", part1(&input));
}
