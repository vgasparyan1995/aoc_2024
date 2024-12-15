use itertools::Itertools;
use std::{
    io::{stdin, BufRead},
    ops::{Add, Mul},
};

type Mtx = Vec<Vec<char>>;
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
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

impl Mul<i32> for Pos {
    type Output = Self;
    fn mul(self, rhs: i32) -> Self::Output {
        Self {
            r: self.r * rhs,
            c: self.c * rhs,
        }
    }
}

fn get(mtx: &Mtx, p: Pos) -> char {
    mtx[p.r as usize][p.c as usize]
}

fn set(mtx: &mut Mtx, p: Pos, ch: char) {
    mtx[p.r as usize][p.c as usize] = ch;
}

fn to_dir(dir: char) -> Dir {
    match dir {
        '<' => LEFT,
        '>' => RIGHT,
        '^' => UP,
        'v' => DOWN,
        _ => panic!("Unexpected direction"),
    }
}

fn find_robot(mtx: &Mtx, r_max: i32, c_max: i32) -> Pos {
    (0..r_max)
        .cartesian_product(0..c_max)
        .map(|(r, c)| Pos { r, c })
        .find(|&p| get(mtx, p) == '@')
        .unwrap()
}

fn part1(mtx: Mtx, moves: &Vec<Dir>) -> i32 {
    let r_max = mtx.len() as i32;
    let c_max = mtx[0].len() as i32;
    let robot = find_robot(&mtx, r_max, c_max);
    let (mtx, _) = moves
        .into_iter()
        .fold((mtx, robot), |(mut mtx, mut robot), &mov| {
            if do_move(&mut mtx, robot, mov) {
                robot = robot + mov;
            }
            (mtx, robot)
        });
    (0..r_max)
        .cartesian_product(0..c_max)
        .filter(|&(r, c)| get(&mtx, Pos { r, c }) == 'O')
        .map(|(r, c)| 100 * r + c)
        .sum()
}

fn check_move(mtx: &mut Mtx, p: Pos, d: Dir) -> bool {
    let ch = get(mtx, p);
    let next = p + d;
    match ch {
        '.' => true,
        '#' => false,
        'O' | '@' => check_move(mtx, next, d),
        '[' => check_move(mtx, next, d) && check_move(mtx, p + RIGHT + d, d),
        ']' => check_move(mtx, next, d) && check_move(mtx, p + LEFT + d, d),
        _ => panic!(),
    }
}

fn do_move(mtx: &mut Mtx, p: Pos, d: Dir) -> bool {
    let ch = get(mtx, p);
    let next = p + d;
    match ch {
        '.' => true,
        '#' => false,
        'O' | '@' => {
            if do_move(mtx, next, d) {
                set(mtx, next, ch);
                set(mtx, p, '.');
                return true;
            }
            false
        }
        '[' | ']' => {
            let pair = p + if ch == '[' { RIGHT } else { LEFT };
            let pair_next = pair + d;
            let pair_ch = if ch == '[' { ']' } else { '[' };
            if pair == next {
                if do_move(mtx, next + d, d) {
                    set(mtx, pair_next, pair_ch);
                    set(mtx, next, ch);
                    set(mtx, p, '.');
                    return true;
                }
            } else {
                if check_move(mtx, next, d) && check_move(mtx, pair_next, d) {
                    do_move(mtx, next, d);
                    set(mtx, next, ch);
                    set(mtx, p, '.');
                    do_move(mtx, pair_next, d);
                    set(mtx, pair_next, pair_ch);
                    set(mtx, pair, '.');
                    return true;
                }
            }
            false
        }
        _ => panic!(),
    }
}

fn part2(mtx: Mtx, moves: &Vec<Dir>) -> i32 {
    let r_max = mtx.len() as i32;
    let c_max = mtx[0].len() as i32;
    let robot = find_robot(&mtx, r_max, c_max);
    let (mtx, _) = moves
        .into_iter()
        .fold((mtx, robot), |(mut mtx, mut robot), &mov| {
            if do_move(&mut mtx, robot, mov) {
                robot = robot + mov;
            }
            (mtx, robot)
        });
    (0..r_max)
        .cartesian_product(0..c_max)
        .filter(|&(r, c)| get(&mtx, Pos { r, c }) == '[')
        .map(|(r, c)| 100 * r + c)
        .sum()
}

fn main() {
    let mut input_iter = stdin().lock().lines().map(Result::unwrap);
    let mtx: Mtx = input_iter
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect();
    let moves = input_iter
        .flat_map(|line| line.chars().map(to_dir).collect::<Vec<_>>())
        .collect();

    println!("Part 1: {}", part1(mtx.clone(), &moves));

    let mtx = mtx
        .into_iter()
        .map(|row| {
            row.into_iter()
                .map(|ch| match ch {
                    '.' => ['.', '.'],
                    'O' => ['[', ']'],
                    '#' => ['#', '#'],
                    '@' => ['@', '.'],
                    _ => panic!(),
                })
                .flatten()
                .collect()
        })
        .collect();
    println!("Part 2: {}", part2(mtx, &moves));
}
