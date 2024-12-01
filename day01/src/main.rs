use std::{
    collections::HashMap,
    io::{stdin, BufRead},
};

fn part1(input: &[(i32, i32)]) -> u32 {
    let (mut vec1, mut vec2): (Vec<i32>, Vec<i32>) = input.iter().cloned().unzip();
    vec1.sort();
    vec2.sort();
    vec1.into_iter()
        .zip(vec2.into_iter())
        .map(|(id1, id2)| id1.abs_diff(id2))
        .sum()
}

fn part2(input: &[(i32, i32)]) -> i32 {
    let freq =
        input
            .into_iter()
            .map(|(_, id2)| id2)
            .fold(HashMap::<i32, i32>::new(), |freq, &id| {
                let mut freq = freq;
                *freq.entry(id).or_insert(0) += 1;
                freq
            });
    input
        .into_iter()
        .map(|(id1, _)| id1)
        .map(|id| id * freq.get(id).unwrap_or(&0))
        .sum()
}

fn main() {
    let input: Vec<(i32, i32)> = stdin()
        .lock()
        .lines()
        .map(Result::unwrap)
        .filter_map(|line| {
            let (id1, id2) = line.split_once("   ")?;
            let id1 = id1.parse::<i32>().ok()?;
            let id2 = id2.parse::<i32>().ok()?;
            Some((id1, id2))
        })
        .collect();
    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
}
