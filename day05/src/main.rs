use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    io::{stdin, BufRead},
};

fn part1(ord_rules: &Vec<(i32, i32)>, updates: &Vec<Vec<i32>>) -> i32 {
    let mut updates = updates
        .into_iter()
        .map(|update| {
            (
                update[update.len() / 2],
                update
                    .into_iter()
                    .enumerate()
                    .fold(HashMap::new(), |mut pages, (idx, page)| {
                        pages.insert(page, idx);
                        pages
                    }),
                true,
            )
        })
        .collect::<Vec<_>>();
    for (a, b) in ord_rules.into_iter() {
        for (_, pages, valid) in updates.iter_mut() {
            if let (Some(a_idx), Some(b_idx)) = (pages.get(a), pages.get(b)) {
                if a_idx > b_idx {
                    *valid = false;
                }
            }
        }
    }
    updates
        .iter()
        .filter(|(_, _, valid)| *valid)
        .map(|(mid, _, _)| mid)
        .sum()
}

fn part2(ord_rules: &Vec<(i32, i32)>, updates: &Vec<Vec<i32>>) -> i32 {
    let mut updates = updates
        .into_iter()
        .map(|update| {
            (
                update.clone(),
                update
                    .into_iter()
                    .enumerate()
                    .fold(HashMap::new(), |mut pages, (idx, page)| {
                        pages.insert(page, idx);
                        pages
                    }),
                true,
            )
        })
        .collect::<Vec<_>>();
    for (a, b) in ord_rules.into_iter() {
        for (_, pages, valid) in updates.iter_mut() {
            if let (Some(a_idx), Some(b_idx)) = (pages.get(a), pages.get(b)) {
                if a_idx > b_idx {
                    *valid = false;
                }
            }
        }
    }
    let ord_rules = ord_rules
        .into_iter()
        .fold(HashMap::new(), |mut ord_rules, (a, b)| {
            ord_rules.entry(a).or_insert(HashSet::new()).insert(b);
            ord_rules
        });
    updates
        .into_iter()
        .filter(|(_, _, valid)| !*valid)
        .map(|(mut pages, _, _)| {
            pages.sort_by(|a, b| {
                if ord_rules.get(&a).is_some_and(|set| set.contains(b)) {
                    return Ordering::Less;
                }
                if ord_rules.get(&b).is_some_and(|set| set.contains(a)) {
                    return Ordering::Greater;
                }
                return Ordering::Equal;
            });
            pages
        })
        .map(|sorted_pages| sorted_pages[sorted_pages.len() / 2])
        .sum()
}

fn main() {
    let mut lines = stdin().lock().lines().map(Result::unwrap);
    let ord_rules: Vec<(i32, i32)> = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .filter_map(|line| {
            let (a, b) = line.split_once("|")?;
            Some((a.parse().ok()?, b.parse().ok()?))
        })
        .collect();
    let updates: Vec<Vec<i32>> = lines
        .map(|line| {
            line.split(",")
                .filter_map(|page| page.parse().ok())
                .collect()
        })
        .collect();

    println!("Part1: {}", part1(&ord_rules, &updates));
    println!("Part2: {}", part2(&ord_rules, &updates));
}
