use itertools::Itertools;
use std::{
    collections::{BTreeMap, BTreeSet, HashMap},
    fmt::Display,
    io::stdin,
    str::FromStr,
};

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
struct Id(u16);

impl Id {
    fn to_str(&self) -> String {
        let a = self.0 / 26;
        let b = self.0 % 26;
        [(a as u8 + 'a' as u8) as char, (b as u8 + 'a' as u8) as char]
            .iter()
            .collect()
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

impl FromStr for Id {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.as_bytes() {
            [a, b] => Ok(Id((*a as u16 - 'a' as u16) * 26 + *b as u16 - 'a' as u16)),
            _ => Err(()),
        }
    }
}

fn part1(input: &Vec<(Id, Id)>) -> usize {
    let graph: HashMap<Id, Vec<Id>> =
        input
            .into_iter()
            .fold(HashMap::new(), |mut graph, &(v, u)| {
                graph.entry(v).or_default().push(u);
                graph.entry(u).or_default().push(v);
                graph
            });
    graph
        .keys()
        .tuple_combinations()
        .filter(|(a, b, c)| graph[a].contains(b) && graph[b].contains(c) && graph[c].contains(a))
        .filter(|(a, b, c)| {
            a.to_str().starts_with('t')
                || b.to_str().starts_with('t')
                || c.to_str().starts_with('t')
        })
        .count()
}

fn part2(input: &Vec<(Id, Id)>) -> String {
    let graph: BTreeMap<Id, BTreeSet<Id>> =
        input
            .into_iter()
            .fold(BTreeMap::new(), |mut graph, &(v, u)| {
                graph.entry(v).or_default().insert(u);
                graph.entry(u).or_default().insert(v);
                graph
            });
    let groups = graph
        .into_iter()
        .map(|(id, mut neighbors)| {
            neighbors.insert(id);
            (id, neighbors)
        })
        .collect::<HashMap<_, _>>();
    groups
        .iter()
        .tuple_combinations()
        .filter_map(|((id1, g1), (id2, g2))| {
            let intersection = g1 & g2;
            if intersection.contains(id1) && intersection.contains(id2) {
                Some(intersection)
            } else {
                None
            }
        })
        .fold(
            HashMap::new(),
            |mut counted: HashMap<BTreeSet<Id>, usize>, group| {
                *counted.entry(group).or_default() += 1;
                counted
            },
        )
        .into_iter()
        .max_by_key(|(_, v)| *v)
        .unwrap()
        .0
        .into_iter()
        .collect::<Vec<_>>()
        .into_iter()
        .join(",")
}

fn main() {
    let input = stdin()
        .lines()
        .map(Result::unwrap)
        .filter_map(|line| {
            let (a, b) = line.split_once("-")?;
            let (a, b) = (Id::from_str(a).ok()?, Id::from_str(b).ok()?);
            Some((a, b))
        })
        .collect();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
