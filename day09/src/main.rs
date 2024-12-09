use std::io::{stdin, BufRead};

fn part1(mut input: Vec<usize>) -> usize {
    let mut l_file = 0;
    let mut r_file = (input.len() + 1) / 2 - 1;
    let mut l_idx = 0;
    let mut r_idx = r_file * 2;
    let mut checksum_idx = 0;
    let mut checksum = 0;
    'main: while l_idx <= r_idx {
        for _ in 0..input[l_idx] {
            checksum += l_file * checksum_idx;
            print!("{l_file}");
            checksum_idx += 1;
        }
        input[l_idx] = 0;
        l_file += 1;
        l_idx += 1;

        for _ in 0..input[l_idx] {
            while input[r_idx] == 0 {
                r_idx -= 2;
                r_file -= 1;
                if l_idx > r_idx {
                    break 'main;
                }
            }
            if input[r_idx] > 0 {
                checksum += r_file * checksum_idx;
                print!("{r_file}");
                checksum_idx += 1;
                input[r_idx] -= 1;
            }
        }
        l_idx += 1;
    }
    println!("");
    checksum
}

fn main() {
    let mut input = String::new();
    let _ = stdin().lock().read_line(&mut input);
    let input = input
        .chars()
        .filter_map(|ch| ch.to_digit(10))
        .map(|n| n as usize)
        .collect::<Vec<_>>();

    println!("Part 1: {}", part1(input.clone()));
}
