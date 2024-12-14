use std::io::{stdin, BufRead};

type Config = (i64, i64, i64, i64, i64, i64);
type Input = Vec<Config>;

fn part1(input: &Input) -> i64 {
    input
        .into_iter()
        .map(|&(x0, y0, x1, y1, xt, yt)| {
            let (n0, r0) = num::integer::div_rem(xt * y1 - yt * x1, x0 * y1 - y0 * x1);
            let (n1, r1) = num::integer::div_rem(xt * y0 - yt * x0, x1 * y0 - y1 * x0);
            if r0 != 0 || r1 != 0 {
                return 0;
            }
            if n0 > 100 || n1 > 100 {
                return 0;
            }
            n0 * 3 + n1
        })
        .sum()
}

fn part2(input: &Input) -> i64 {
    input
        .into_iter()
        .map(|&(x0, y0, x1, y1, xt, yt)| {
            let xt = xt + 10000000000000i64;
            let yt = yt + 10000000000000i64;
            let (n0, r0) = num::integer::div_rem(xt * y1 - yt * x1, x0 * y1 - y0 * x1);
            let (n1, r1) = num::integer::div_rem(xt * y0 - yt * x0, x1 * y0 - y1 * x0);
            if r0 != 0 || r1 != 0 {
                return 0;
            }
            n0 * 3 + n1
        })
        .sum()
}

fn main() {
    let mut input = Input::new();
    let mut config = (0, 0, 0, 0, 0, 0);
    for line in stdin().lock().lines().map(Result::unwrap) {
        if line.is_empty() {
            input.push(config);
        } else if let Some(s) = line.strip_prefix("Button A: X+") {
            let (x, y) = s.split_once(", Y+").unwrap();
            let (x, y) = (x.parse().unwrap(), y.parse().unwrap());
            config.0 = x;
            config.1 = y;
        } else if let Some(s) = line.strip_prefix("Button B: X+") {
            let (x, y) = s.split_once(", Y+").unwrap();
            let (x, y) = (x.parse().unwrap(), y.parse().unwrap());
            config.2 = x;
            config.3 = y;
        } else if let Some(s) = line.strip_prefix("Prize: X=") {
            let (x, y) = s.split_once(", Y=").unwrap();
            let (x, y) = (x.parse().unwrap(), y.parse().unwrap());
            config.4 = x;
            config.5 = y;
        }
    }
    input.push(config);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
