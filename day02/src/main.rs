use std::io::{stdin, BufRead};

fn safety_errors(report: &[i32]) -> usize {
    let sign = (report[0] - report[report.len() - 1]).signum();
    (0..report.len() - 1)
        .map(|i| report[i] - report[i + 1])
        .filter(|delta| delta.signum() != sign || delta.abs() > 3 || delta.abs() == 0)
        .count()
}

fn part1(input: &Vec<Vec<i32>>) -> usize {
    input
        .iter()
        .map(|report| safety_errors(report))
        .filter(|&cnt| cnt == 0)
        .count()
}

fn part2(input: &Vec<Vec<i32>>) -> usize {
    input
        .iter()
        .map(|report| safety_errors(report))
        .filter(|&cnt| cnt <= 1)
        .count()
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
