use std::{
    collections::HashMap,
    io::{stdin, BufRead},
    iter::successors,
};

fn prune(n: i64) -> i64 {
    n % 16777216
}

fn evolve(n: i64) -> i64 {
    let n = prune(n ^ (n << 6));
    let n = prune(n ^ (n >> 5));
    let n = prune(n ^ (n << 11));
    n
}

fn part1(input: &[i64]) -> i64 {
    input
        .into_iter()
        .map(|&n| {
            successors(Some(n), |&n| Some(evolve(n)))
                .skip(2000)
                .next()
                .unwrap()
        })
        .sum()
}

fn part2(input: &[i64]) -> i64 {
    input
        .into_iter()
        .map(|&n| {
            let prices = successors(Some(n), |&n| Some(evolve(n)))
                .map(|n| n % 10)
                .take(2000)
                .collect::<Vec<_>>();
            prices
                .windows(5)
                .filter_map(|window| match window {
                    [a, b, c, d, e] => Some((a, b, c, d, e)),
                    _ => None,
                })
                .map(|(a, b, c, d, e)| ((b - a, c - b, d - c, e - d), *e))
                .fold(HashMap::new(), |mut hm, (k, v)| {
                    hm.entry(k).or_insert(v);
                    hm
                })
        })
        .fold(HashMap::new(), |mut uhm, hm| {
            hm.into_iter().for_each(|(k, v)| {
                *uhm.entry(k).or_default() += v;
                if k == (0, 0, -1, 1) {
                }
            });
            uhm
        })
        .into_values()
        .max()
        .unwrap()
}

fn main() {
    let input = stdin()
        .lock()
        .lines()
        .map(Result::unwrap)
        .map(|line| line.parse().unwrap())
        .collect::<Vec<_>>();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
