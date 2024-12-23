use std::{
    collections::HashMap,
    io::{stdin, BufRead},
    ops::Add,
};

use itertools::Itertools;

type Mtx = Vec<Vec<char>>;
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
struct Pos {
    r: i32,
    c: i32,
}
type Dir = Pos;
const LEFT: Dir = Dir { r: 0, c: -1 };
const RIGHT: Dir = Dir { r: 0, c: 1 };
const UP: Dir = Dir { r: -1, c: 0 };
const DOWN: Dir = Dir { r: 1, c: 0 };
const DIRS: [Dir; 4] = [RIGHT, LEFT, UP, DOWN];

impl Add for Pos {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            r: self.r + other.r,
            c: self.c + other.c,
        }
    }
}

impl Dir {
    fn turn_left(&self) -> Dir {
        match *self {
            LEFT => DOWN,
            DOWN => RIGHT,
            RIGHT => UP,
            UP => LEFT,
            _ => panic!(),
        }
    }

    fn turn_right(&self) -> Dir {
        match *self {
            LEFT => UP,
            UP => RIGHT,
            RIGHT => DOWN,
            DOWN => LEFT,
            _ => panic!(),
        }
    }
}

fn get(mtx: &Mtx, p: Pos) -> Option<char> {
    mtx.get(p.r as usize)?.get(p.c as usize).copied()
}

fn find(mtx: &Mtx, ch: char) -> Pos {
    let r_max = mtx.len();
    let c_max = mtx[0].len();
    (0..r_max)
        .cartesian_product(0..c_max)
        .map(|(r, c)| Pos {
            r: r as i32,
            c: c as i32,
        })
        .find(|&p| get(mtx, p) == Some(ch))
        .unwrap()
}

fn part1(mtx: &Mtx, moves_saved: i32) -> usize {
    let start_pos = find(mtx, 'S');
    let start_dir = DIRS
        .into_iter()
        .find(|&dir| get(mtx, start_pos + dir) == Some('.'))
        .unwrap();
    let path = std::iter::successors(Some((start_pos, start_dir)), |&(pos, dir)| {
        [dir, dir.turn_left(), dir.turn_right()]
            .into_iter()
            .map(|dir| (pos + dir, dir))
            .find(|&(next_pos, _)| get(mtx, next_pos) != Some('#'))
    })
    .map(|(pos, _)| pos)
    .enumerate()
    .fold(HashMap::new(), |mut path, (idx, pos)| {
        path.insert(pos, idx);
        path
    });
    let num_cheats = std::iter::successors(Some((start_pos, start_dir)), |&(pos, dir)| {
        [dir, dir.turn_left(), dir.turn_right()]
            .into_iter()
            .map(|dir| (pos + dir, dir))
            .find(|&(next_pos, _)| get(mtx, next_pos) != Some('#'))
    })
    .map(|(orig_pos, orig_dir)| {
        [orig_dir, orig_dir.turn_left(), orig_dir.turn_right()]
            .into_iter()
            .filter(|&dir| get(mtx, orig_pos + dir) == Some('#'))
            .map(|dir| orig_pos + dir + dir)
            .filter(|&cheat_end_pos| get(mtx, cheat_end_pos) != Some('#'))
            .filter(|cheat_end_pos| {
                path.get(&cheat_end_pos).is_some_and(|&cheat_end_idx| {
                    cheat_end_idx as i32 - path[&orig_pos] as i32 >= moves_saved + 2
                })
            })
            .count()
    })
    .sum();
    num_cheats
}

fn part2(mtx: &Mtx, moves_saved: usize) -> usize {
    let start_pos = find(mtx, 'S');
    let start_dir = DIRS
        .into_iter()
        .find(|&dir| get(mtx, start_pos + dir) == Some('.'))
        .unwrap();
    let path = std::iter::successors(Some((start_pos, start_dir)), |&(pos, dir)| {
        [dir, dir.turn_left(), dir.turn_right()]
            .into_iter()
            .map(|dir| (pos + dir, dir))
            .find(|&(next_pos, _)| get(mtx, next_pos) != Some('#'))
    })
    .map(|(pos, _)| pos)
    .collect::<Vec<_>>();
    (0..path.len())
        .cartesian_product(0..path.len())
        .filter(|&(i, j)| i < j)
        .map(|(i, j)| {
            let p1 = path[i];
            let p2 = path[j];
            let old_dist = j - i;
            let new_dist = p1.r.abs_diff(p2.r) + p1.c.abs_diff(p2.c);
            (path[i], path[j], old_dist as usize, new_dist as usize)
        })
        .filter_map(|(p1, p2, old_dist, new_dist)| {
            if new_dist <= 20 && new_dist + moves_saved <= old_dist {
                Some((p1, p2))
            } else {
                None
            }
        })
        .unique()
        .count()
}

fn main() {
    let input = stdin()
        .lock()
        .lines()
        .map(Result::unwrap)
        .map(|line| line.chars().collect())
        .collect();

    println!("Part 1: {}", part1(&input, 100));
    println!("Part 2: {}", part2(&input, 100));
}
