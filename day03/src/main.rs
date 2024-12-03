use anyhow::Result;
use regex::Regex;
use std::io::{read_to_string, stdin};

fn part1(input: &str) -> Result<i32> {
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)")?;
    Ok(re
        .captures_iter(input)
        .map(|c| c.extract().1)
        .map(|[a, b]| (a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap()))
        .map(|(a, b)| a * b)
        .sum())
}

fn part2(input: &str) -> Result<i32> {
    let toggle_re = Regex::new(r"do\(\)|don't\(\)")?;
    let toggles = toggle_re
        .find_iter(input)
        .map(|m| (m.start(), m.as_str() == "do()"))
        .collect::<Vec<_>>();
    let mul_re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)")?;
    Ok(mul_re
        .captures_iter(input)
        .map(|c| (c.get(0).unwrap().start(), c.extract().1))
        .map(|(idx, [a, b])| (idx, a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap()))
        .map(|(idx, a, b)| (idx, a * b))
        .filter(|(m_idx, _)| {
            let toggle_idx = toggles.partition_point(|(t_idx, _)| t_idx < m_idx);
            toggle_idx == 0 || toggles[toggle_idx - 1].1
        })
        .map(|(_, val)| val)
        .sum())
}

fn main() -> Result<()> {
    let input = read_to_string(stdin())?;
    println!("Part1: {}", part1(&input)?);
    println!("Part2: {}", part2(&input)?);
    Ok(())
}
