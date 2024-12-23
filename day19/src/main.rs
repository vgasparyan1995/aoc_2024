use std::{
    collections::HashMap,
    io::{stdin, BufRead},
};

#[derive(Default, Debug)]
struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_word: bool,
}

#[derive(Default, Debug)]
struct Trie {
    root: TrieNode,
}

impl Trie {
    fn insert(&mut self, entry: &str) {
        let mut node = &mut self.root;
        for ch in entry.chars() {
            node = node.children.entry(ch).or_default();
        }
        node.is_word = true;
    }
}

fn is_fillable(design: &str, towels: &Trie, from: usize, dp: &mut Vec<Option<bool>>) -> bool {
    match dp.get(from) {
        None => true,
        Some(&Some(fillable)) => fillable,
        Some(None) => {
            let fillable = (|| {
                let mut node = Some(&towels.root);
                let mut idx = from;
                while node.is_some() {
                    if node.unwrap().is_word && is_fillable(design, towels, idx, dp) {
                        return true;
                    }
                    if idx >= design.len() {
                        return false;
                    }
                    node = node
                        .unwrap()
                        .children
                        .get(&(design.as_bytes()[idx] as char));
                    idx += 1;
                }
                false
            })();
            dp[from] = Some(fillable);
            fillable
        }
    }
}

fn part1(towels: &Vec<String>, designs: &Vec<String>) -> usize {
    let towels = towels
        .into_iter()
        .fold(Trie::default(), |mut towels, towel| {
            towels.insert(towel);
            towels
        });
    designs
        .into_iter()
        .filter(|design| {
            let mut dp: Vec<Option<bool>> = vec![None; design.len()];
            is_fillable(design, &towels, 0, &mut dp)
        })
        .count()
}

fn num_combinations(
    design: &str,
    towels: &Trie,
    from: usize,
    dp: &mut Vec<Option<usize>>,
) -> usize {
    match dp.get(from) {
        None => 1,
        Some(&Some(n)) => n,
        Some(None) => {
            let num_combos = (from..design.len())
                .scan(&towels.root, |node, idx| {
                    if let Some(next_node) = node.children.get(&(design.as_bytes()[idx] as char)) {
                        *node = next_node;
                        Some((node.is_word, idx))
                    } else {
                        None
                    }
                })
                .filter_map(|(is_word, idx)| {
                    if is_word {
                        Some(num_combinations(design, towels, idx + 1, dp))
                    } else {
                        None
                    }
                })
                .sum();
            dp[from] = Some(num_combos);
            num_combos
        }
    }
}

fn part2(towels: &Vec<String>, designs: &Vec<String>) -> usize {
    let towels = towels
        .into_iter()
        .fold(Trie::default(), |mut towels, towel| {
            towels.insert(towel);
            towels
        });
    designs
        .into_iter()
        .map(|design| {
            let mut dp: Vec<Option<usize>> = vec![None; design.len()];
            let n = num_combinations(design, &towels, 0, &mut dp);
            n
        })
        .sum()
}

fn main() {
    let mut lines = stdin().lock().lines().map(Result::unwrap);
    let towels: Vec<String> = lines
        .next()
        .unwrap()
        .split(", ")
        .map(|towel| towel.to_string())
        .collect();
    let _ = lines.next();
    let designs: Vec<String> = lines.collect();

    println!("Part 1: {}", part1(&towels, &designs));
    println!("Part 2: {}", part2(&towels, &designs));
}
