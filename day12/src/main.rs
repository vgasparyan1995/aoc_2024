use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::io::{stdin, BufRead};

type Mtx = Vec<Vec<char>>;
type Pos = (i32, i32);
type Dir = (i32, i32);
type Fences = HashMap<Dir, Vec<Pos>>;

const DIRS: [Dir; 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

fn get(mtx: &Mtx, r: i32, c: i32, r_max: i32, c_max: i32) -> Option<char> {
    if r < 0 || r >= r_max || c < 0 || c >= c_max {
        None
    } else {
        Some(mtx[r as usize][c as usize])
    }
}

fn dfs<Visitor>(
    mtx: &Mtx,
    r: i32,
    c: i32,
    r_max: i32,
    c_max: i32,
    visited: &mut HashSet<Pos>,
    visitor: &mut Visitor,
) where
    Visitor: FnMut(i32, i32),
{
    if !visited.insert((r, c)) {
        return;
    }
    visitor(r, c);
    let ch = get(mtx, r, c, r_max, c_max);
    DIRS.iter()
        .filter(|(dr, dc)| get(mtx, r + dr, c + dc, r_max, c_max) == ch)
        .for_each(|(dr, dc)| dfs(mtx, r + dr, c + dc, r_max, c_max, visited, visitor));
}

fn part1(mtx: &Mtx) -> i32 {
    let r_max = mtx.len() as i32;
    let c_max = mtx[0].len() as i32;
    let mut visited = HashSet::new();
    (0..r_max)
        .cartesian_product(0..c_max)
        .map(|(r, c)| {
            let (mut area, mut perimeter) = (0, 0);
            dfs(mtx, r, c, r_max, c_max, &mut visited, &mut |r, c| {
                area += 1;
                let curr = get(mtx, r, c, r_max, c_max);
                perimeter += DIRS
                    .iter()
                    .map(|(dr, dc)| get(mtx, r + dr, c + dc, r_max, c_max))
                    .filter(|&neighbor| curr != neighbor)
                    .count() as i32;
            });
            area * perimeter
        })
        .sum()
}

fn part2(mtx: &Mtx) -> i32 {
    let r_max = mtx.len() as i32;
    let c_max = mtx[0].len() as i32;
    let mut visited = HashSet::new();
    (0..r_max)
        .cartesian_product(0..c_max)
        .map(|(r, c)| {
            let mut area = 0;
            let mut fences = Fences::new();
            dfs(mtx, r, c, r_max, c_max, &mut visited, &mut |r, c| {
                let curr = get(mtx, r, c, r_max, c_max);
                for dir in DIRS
                    .iter()
                    .filter(|&(dr, dc)| get(mtx, r + dr, c + dc, r_max, c_max) != curr)
                {
                    fences.entry(*dir).or_default().push((r, c));
                }
                area += 1;
            });
            let sides = fences
                .into_iter()
                .map(|(dir, mut positions)| {
                    if dir.0 == 0 {
                        positions.sort_by(|(r1, c1), (r2, c2)| c1.cmp(c2).then(r1.cmp(r2)));
                        (1..positions.len())
                            .map(|idx| (positions[idx - 1], positions[idx]))
                            .filter(|((r1, c1), (r2, c2))| c2 != c1 || r2 - r1 != 1)
                            .count() as i32
                            + 1
                    } else {
                        positions.sort_by(|(r1, c1), (r2, c2)| r1.cmp(r2).then(c1.cmp(c2)));
                        (1..positions.len())
                            .map(|idx| (positions[idx - 1], positions[idx]))
                            .filter(|((r1, c1), (r2, c2))| r2 != r1 || c2 - c1 != 1)
                            .count() as i32
                            + 1
                    }
                })
                .sum::<i32>();
            area * sides
        })
        .sum()
}

fn main() {
    let input = stdin()
        .lock()
        .lines()
        .map(Result::unwrap)
        .map(|line| line.chars().into_iter().collect())
        .collect();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
