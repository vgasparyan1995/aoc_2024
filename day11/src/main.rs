use lazy_static::lazy_static;
use std::sync::Mutex;
use std::{
    collections::HashMap,
    io::{stdin, BufRead},
};

type Stone = u64;

lazy_static! {
    static ref MEMO: Mutex<HashMap<Stone, HashMap<Stone, u64>>> = Mutex::new(HashMap::new());
}

fn iterate(stone: Stone) -> HashMap<Stone, u64> {
    match stone {
        0 => HashMap::from([(1, 1)]),
        stone if stone.to_string().len() % 2 == 0 => {
            let stone_str = stone.to_string();
            let (s1, s2) = stone_str.split_at(stone_str.len() / 2);
            let (s1, s2) = (s1.parse().unwrap(), s2.parse().unwrap());
            let mut stones = HashMap::new();
            *stones.entry(s1).or_default() += 1;
            *stones.entry(s2).or_default() += 1;
            stones
        }
        stone => HashMap::from([(stone * 2024, 1)]),
    }
}

fn num_stones(stones: &Vec<Stone>, num_blinks: u64) -> u64 {
    let mut grouped_stones =
        stones
            .into_iter()
            .map(|stone| (stone, 1))
            .fold(HashMap::new(), |mut hm, (&k, v)| {
                *hm.entry(k).or_insert(0) += v;
                hm
            });
    for _ in 0..num_blinks {
        grouped_stones = grouped_stones
            .into_iter()
            .map(|(stone, cnt)| {
                MEMO.lock()
                    .unwrap()
                    .entry(stone)
                    .or_insert(iterate(stone))
                    .iter()
                    .map(|(&next_stone, &next_cnt)| (next_stone, next_cnt * cnt))
                    .fold(HashMap::new(), |mut hm, (k, v)| {
                        *hm.entry(k).or_insert(0) += v;
                        hm
                    })
            })
            .fold(HashMap::new(), |mut hm, next_stones| {
                for (next_stone, next_cnt) in next_stones.into_iter() {
                    *hm.entry(next_stone).or_insert(0) += next_cnt;
                }
                hm
            });
    }
    // println!("{grouped_stones:?}");
    grouped_stones.into_iter().map(|(_, cnt)| cnt).sum()
}

fn part1(stones: &Vec<Stone>) -> u64 {
    num_stones(stones, 25)
}

fn part2(stones: &Vec<Stone>) -> u64 {
    num_stones(stones, 75)
}

fn main() {
    let mut stones = String::new();
    assert!(stdin().lock().read_line(&mut stones).is_ok());
    let stones = stones
        .trim()
        .split(' ')
        .filter_map(|stone| stone.parse().ok())
        .collect::<Vec<Stone>>();

    println!("Part 1: {}", part1(&stones));
    println!("Part 2: {}", part2(&stones));
}
