use std::io::{stdin, BufRead};

fn is_safe(report: &[i32]) -> bool {
    let sign = (report[0] - report[1]).signum();
    (0..report.len() - 1)
        .map(|i| (report[i], report[i + 1]))
        .map(|(a, b)| a - b)
        .all(|d| d.signum() == sign && d.abs() <= 3 && d.abs() >= 1)
}

fn is_almost_safe(report: &[i32]) -> bool {
    is_safe(report)
        || (0..report.len())
            .map(|i| {
                let it = report.iter();
                it.clone()
                    .take(i)
                    .chain(it.clone().skip(i + 1))
                    .map(|&n| n)
                    .collect::<Vec<_>>()
            })
            .any(|new_report| is_safe(&new_report[..]))
}

fn part1(input: &Vec<Vec<i32>>) -> usize {
    input.iter().filter(|report| is_safe(report)).count()
}

fn part2(input: &Vec<Vec<i32>>) -> usize {
    input.iter().filter(|report| is_almost_safe(report)).count()
}

fn main() {
    let input: Vec<Vec<i32>> = stdin()
        .lock()
        .lines()
        .map(Result::unwrap)
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse::<i32>().unwrap_or(0))
                .collect()
        })
        .collect();

    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
}
