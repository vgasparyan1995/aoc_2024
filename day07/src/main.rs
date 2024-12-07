use std::io::{stdin, BufRead};

type Input = Vec<(i64, Vec<i64>)>;

fn is_valid(target: i64, init: i64, operands: &[i64]) -> bool {
    if operands.is_empty() {
        return false;
    }
    match operands {
        [] => false,
        [first, rest @ ..] => {
            init * first == target
                || init + first == target
                || init * first < target && is_valid(target, init * first, rest)
                || init + first < target && is_valid(target, init + first, rest)
        }
    }
}

fn part1(input: &Input) -> i64 {
    input
        .iter()
        .filter(|&(test_value, operands)| is_valid(*test_value, 0, operands))
        .map(|&(test_value, _)| test_value)
        .sum()
}

fn is_valid2(target: i64, init: Option<i64>, operands: &[i64]) -> bool {
    if operands.is_empty() {
        return false;
    }
    match operands {
        [] => false,
        [first, rest @ ..] => {
            let mul = init.unwrap_or(1) * first;
            let add = init.unwrap_or(0) + first;
            let cat = match init {
                None => *first,
                Some(num) => format!("{num}{first}").parse().unwrap(),
            };
            (mul == target || cat == target || add == target) && rest.is_empty()
                || mul <= target && is_valid2(target, Some(mul), rest)
                || cat <= target && is_valid2(target, Some(cat), rest)
                || add <= target && is_valid2(target, Some(add), rest)
        }
    }
}

fn part2(input: &Input) -> i64 {
    input
        .iter()
        .filter(|&(test_value, operands)| is_valid2(*test_value, None, operands))
        .map(|&(test_value, _)| test_value)
        .sum()
}

fn main() {
    let input = stdin()
        .lock()
        .lines()
        .filter_map(Result::ok)
        .filter_map(|line| {
            let (test_value, operands) = line.split_once(":")?;
            let test_value = test_value.parse().ok()?;
            let operands = operands
                .split(" ")
                .filter_map(|o| o.parse().ok())
                .collect::<Vec<_>>();
            Some((test_value, operands))
        })
        .collect();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
