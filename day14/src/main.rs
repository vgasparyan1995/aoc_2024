use std::{
    collections::{HashMap, HashSet},
    env,
    io::{stdin, BufRead},
    ops::{Add, Mul, Rem},
};

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
struct Pos {
    x: i32,
    y: i32,
}
type Velocity = Pos;

impl Add for Pos {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Mul<i32> for Pos {
    type Output = Self;
    fn mul(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Rem<(i32, i32)> for Pos {
    type Output = Self;
    fn rem(self, (rhsx, rhsy): (i32, i32)) -> Self::Output {
        Self {
            x: self.x.rem_euclid(rhsx),
            y: self.y.rem_euclid(rhsy),
        }
    }
}

fn part1(input: &Vec<(Pos, Velocity)>, x_max: i32, y_max: i32) -> i32 {
    let x_mid = x_max / 2;
    let y_mid = y_max / 2;
    input
        .iter()
        .map(|&(p, v)| (p + v * 100) % (x_max, y_max))
        .filter_map(|p| match p {
            Pos { x, y } if x < x_mid && y < y_mid => Some(0),
            Pos { x, y } if x > x_mid && y < y_mid => Some(1),
            Pos { x, y } if x < x_mid && y > y_mid => Some(2),
            Pos { x, y } if x > x_mid && y > y_mid => Some(3),
            _ => None,
        })
        .fold(HashMap::<i32, i32>::new(), |mut hm, q| {
            *hm.entry(q).or_default() += 1;
            hm
        })
        .into_iter()
        .map(|(_, cnt)| cnt)
        .product()
}

fn part2(input: &Vec<(Pos, Velocity)>, x_max: i32, y_max: i32) -> i32 {
    (0..)
        .find(|&n| {
            let robots = input
                .iter()
                .map(|&(p, v)| (p + v * n) % (x_max, y_max))
                .collect::<HashSet<_>>();
            if robots.len() == input.len() {
                (0..y_max).for_each(|y| {
                    println!(
                        "{}",
                        (0..x_max)
                            .map(|x| if robots.contains(&Pos { x, y }) {
                                '*'
                            } else {
                                '.'
                            })
                            .collect::<String>()
                    )
                });
                true
            } else {
                false
            }
        })
        .unwrap_or(0)
}

fn main() {
    let (x_max, y_max) = match env::args().skip(1).next() {
        Some(arg) if arg == "-t" => (11, 7),
        _ => (101, 103),
    };
    let input = stdin()
        .lock()
        .lines()
        .map(Result::unwrap)
        .filter_map(|line| {
            let (p, v) = line.split_once(" ")?;
            let (px, py) = p.strip_prefix("p=")?.split_once(",")?;
            let (px, py) = (px.parse().ok()?, py.parse().ok()?);
            let (vx, vy) = v.strip_prefix("v=")?.split_once(",")?;
            let (vx, vy) = (vx.parse().ok()?, vy.parse().ok()?);
            Some((Pos { x: px, y: py }, Velocity { x: vx, y: vy }))
        })
        .collect::<Vec<(Pos, Velocity)>>();

    println!("Part 1: {}", part1(&input, x_max, y_max));
    println!("Part 2: {}", part2(&input, x_max, y_max));
}
