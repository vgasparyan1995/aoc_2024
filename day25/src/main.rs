use itertools::Itertools;
use std::io::stdin;

fn do_match(lock: &Vec<i32>, key: &Vec<i32>) -> bool {
    lock.iter()
        .zip(key.iter())
        .map(|(l, k)| l - k)
        .all(|n| n <= 5)
}

fn part1(input: &Vec<Vec<i32>>) -> usize {
    let locks = input.iter().filter(|arr| arr.iter().all(|&n| n >= 0));
    let keys = input.iter().filter(|arr| arr.iter().all(|&n| n <= 0));
    locks
        .cartesian_product(keys)
        .filter(|(lock, key)| do_match(lock, key))
        .count()
}

fn main() {
    let input = (0..)
        .map(|_| {
            stdin()
                .lines()
                .map(Result::unwrap)
                .take_while(|line| !line.is_empty())
                .collect::<Vec<_>>()
        })
        .take_while(|mask| !mask.is_empty())
        .map(|mask| {
            let r_max = mask.len();
            let c_max = mask[0].len();
            let sign = if mask[0].starts_with("#") { 1 } else { -1 };
            (0..c_max)
                .map(|c| {
                    let num = (0..r_max)
                        .filter_map(|r| mask[r].chars().nth(c))
                        .filter(|&ch| ch == '#')
                        .count() as i32
                        - 1;
                    num * sign
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    println!("Part 1: {}", part1(&input));
}
