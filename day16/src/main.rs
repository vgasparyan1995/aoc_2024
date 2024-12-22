use itertools::Itertools;
use std::{
    collections::HashMap,
    io::{stdin, BufRead},
    ops::Add,
};

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

impl Add for Pos {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            r: self.r + other.r,
            c: self.c + other.c,
        }
    }
}

fn find(mtx: &Mtx, target: char) -> Pos {
    let (r, c, _) = mtx
        .iter()
        .enumerate()
        .flat_map(|(r, row)| row.iter().enumerate().map(move |(c, ch)| (r, c, ch)))
        .find(|(_, _, &ch)| ch == target)
        .unwrap();
    Pos {
        r: r as i32,
        c: c as i32,
    }
}

fn get(mtx: &Mtx, p: Pos) -> char {
    mtx[p.r as usize][p.c as usize]
}

fn turn_left(dir: Dir) -> Dir {
    match dir {
        LEFT => DOWN,
        DOWN => RIGHT,
        RIGHT => UP,
        UP => LEFT,
        _ => panic!(),
    }
}

fn turn_right(dir: Dir) -> Dir {
    match dir {
        LEFT => UP,
        UP => RIGHT,
        RIGHT => DOWN,
        DOWN => LEFT,
        _ => panic!(),
    }
}

fn distances_from(mtx: &Mtx, pos: Pos, dir: Dir) -> HashMap<(Pos, Dir), i32> {
    let mut distance_to = HashMap::from([((pos, dir), 0)]);
    let mut paths = vec![(pos, dir, 0)];
    while !paths.is_empty() {
        paths = paths
            .into_iter()
            .map(|(pos, dir, distance)| {
                let mut branches = Vec::new();
                if get(mtx, pos + dir) != '#'
                    && *distance_to.get(&(pos + dir, dir)).unwrap_or(&i32::MAX) > distance + 1
                {
                    distance_to.insert((pos + dir, dir), distance + 1);
                    branches.push((pos + dir, dir, distance + 1));
                }
                if *distance_to.get(&(pos, turn_left(dir))).unwrap_or(&i32::MAX) > distance + 1000 {
                    distance_to.insert((pos, turn_left(dir)), distance + 1000);
                    branches.push((pos, turn_left(dir), distance + 1000));
                }
                if *distance_to
                    .get(&(pos, turn_right(dir)))
                    .unwrap_or(&i32::MAX)
                    > distance + 1000
                {
                    distance_to.insert((pos, turn_right(dir)), distance + 1000);
                    branches.push((pos, turn_right(dir), distance + 1000));
                }
                branches.into_iter()
            })
            .flatten()
            .collect();
    }
    distance_to
}

fn shortest_distanec_to(distances: &HashMap<(Pos, Dir), i32>, dest: Pos) -> i32 {
    [RIGHT, LEFT, UP, DOWN]
        .into_iter()
        .filter_map(|dir| distances.get(&(dest, dir)))
        .map(|&dist| dist)
        .min()
        .unwrap_or(i32::MAX)
}

fn part1(mtx: &Mtx) -> i32 {
    let start_position = find(mtx, 'S');
    let end_position = find(mtx, 'E');
    let distance_to = distances_from(mtx, start_position, RIGHT);
    shortest_distanec_to(&distance_to, end_position)
}

fn part2(mtx: &Mtx) -> usize {
    let start_position = find(mtx, 'S');
    let end_position = find(mtx, 'E');
    let distance_from_start = distances_from(mtx, start_position, RIGHT);
    let distance_from_end = distances_from(mtx, end_position, DOWN);
    let alt_distance_from_end = distances_from(mtx, end_position, LEFT);

    let shortest_path = shortest_distanec_to(&distance_from_start, end_position);
    let r_max = mtx.len();
    let c_max = mtx[0].len();
    (0..r_max)
        .cartesian_product(0..c_max)
        .map(|(r, c)| Pos {
            r: r as i32,
            c: c as i32,
        })
        .filter(|&pos| get(mtx, pos) != '#')
        .cartesian_product([RIGHT, LEFT, UP, DOWN].into_iter())
        .filter(|&(pos, dir)| {
            let opposite = turn_left(turn_left(dir));
            let dist_from_start = *distance_from_start.get(&(pos, dir)).unwrap();
            let dist_from_end = *distance_from_end.get(&(pos, opposite)).unwrap();
            let alt_dist_from_end = *alt_distance_from_end.get(&(pos, opposite)).unwrap();
            dist_from_start + dist_from_end == shortest_path
                || dist_from_start + alt_dist_from_end == shortest_path
        })
        .map(|(pos, _)| pos)
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

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
