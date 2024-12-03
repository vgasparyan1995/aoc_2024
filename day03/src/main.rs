use anyhow::Result;
use regex::Regex;
use std::io::{read_to_string, stdin};

fn part1(input: &str) -> Result<i32> {
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)")?;
    let result = re
        .captures_iter(input)
        .map(|c| c.extract().1)
        .map(|[a, b]| (a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap()))
        .map(|(a, b)| a * b)
        .sum();
    Ok(result)
}

enum Cmd {
    Do,
    Dont,
    Mul(i32),
}

fn part2(input: &str) -> Result<i32> {
    let re = Regex::new(r"mul\([0-9]+,[0-9]+\)|do\(\)|don't\(\)")?;
    let result = re
        .find_iter(input)
        .map(|m| m.as_str())
        .filter_map(|m| {
            if m == "do()" {
                return Some(Cmd::Do);
            }
            if m == "don't()" {
                return Some(Cmd::Dont);
            }
            assert!(m.starts_with("mul("));
            let (a, b) = m.strip_prefix("mul(")?.strip_suffix(")")?.split_once(",")?;
            Some(Cmd::Mul(a.parse::<i32>().ok()? * b.parse::<i32>().ok()?))
        })
        .fold((0, true), |(sum, enabled), cmd| match cmd {
            Cmd::Do => (sum, true),
            Cmd::Dont => (sum, false),
            Cmd::Mul(n) => (if enabled { sum + n } else { sum }, enabled),
        })
        .0;
    Ok(result)
}

fn main() -> Result<()> {
    let input = read_to_string(stdin())?;
    println!("Part1: {}", part1(&input)?);
    println!("Part2: {}", part2(&input)?);
    Ok(())
}
