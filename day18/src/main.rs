use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
    io::{stdin, BufRead},
    ops::Add,
};

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
struct Pos {
    r: i32,
    c: i32,
}
impl Add for Pos {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            r: self.r + other.r,
            c: self.c + other.c,
        }
    }
}
type Dir = Pos;
const LEFT: Dir = Dir { r: 0, c: -1 };
const RIGHT: Dir = Dir { r: 0, c: 1 };
const UP: Dir = Dir { r: -1, c: 0 };
const DOWN: Dir = Dir { r: 1, c: 0 };
const DIRS: [Dir; 4] = [LEFT, RIGHT, UP, DOWN];

#[derive(PartialOrd, Ord, PartialEq, Eq)]
struct Path(i32, Pos);

fn distance(src: Pos, dst: Pos, r_max: i32, c_max: i32, barriers: HashSet<Pos>) -> Option<i32> {
    let mut distance_to = HashMap::from([(src, 0)]);
    let mut queue = BinaryHeap::new();
    queue.push(Reverse(Path(0, src)));
    while let Some(Reverse(Path(dist, pos))) = queue.pop() {
        if pos == dst {
            return Some(dist);
        }
        for neighbor in DIRS
            .into_iter()
            .map(|dir| pos + dir)
            .filter(|&pos| pos.r >= 0 && pos.r < r_max && pos.c >= 0 && pos.c < c_max)
            .filter(|pos| !barriers.contains(pos))
        {
            let curr_dist = *distance_to.get(&neighbor).unwrap_or(&i32::MAX);
            if dist + 1 < curr_dist {
                *distance_to.entry(neighbor).or_default() = dist + 1;
                queue.push(Reverse(Path(dist + 1, neighbor)));
            }
        }
    }
    None
}

fn part1(input: &Vec<Pos>, r_max: i32, c_max: i32, num_barriers: usize) -> i32 {
    distance(
        Pos { r: 0, c: 0 },
        Pos {
            r: r_max - 1,
            c: c_max - 1,
        },
        r_max,
        c_max,
        input.into_iter().take(num_barriers).map(|&p| p).collect(),
    )
    .unwrap()
}

fn part2(input: &Vec<Pos>, r_max: i32, c_max: i32) -> String {
    let iota = (1..=input.len()).collect::<Vec<_>>();
    let num_corrupt = iota.partition_point(|&num_corrupt| {
        distance(
            Pos { r: 0, c: 0 },
            Pos {
                r: r_max - 1,
                c: c_max - 1,
            },
            r_max,
            c_max,
            input.into_iter().take(num_corrupt).map(|&p| p).collect(),
        )
        .is_some()
    });
    let pos = input[num_corrupt];
    format!("{},{}", pos.c, pos.r)
}

fn main() {
    let input = stdin()
        .lock()
        .lines()
        .map(Result::unwrap)
        .map(|line| {
            let (c, r) = line.split_once(",").unwrap();
            let c = c.parse().unwrap();
            let r = r.parse().unwrap();
            Pos { r, c }
        })
        .collect();

    println!("Part 1: {}", part1(&input, 71, 71, 1024));
    println!("Part 2: {}", part2(&input, 71, 71));
}
