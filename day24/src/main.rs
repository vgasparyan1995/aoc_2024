use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    io::stdin,
    str::FromStr,
};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Wire([char; 3]);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Op {
    AND,
    OR,
    XOR,
}

impl FromStr for Wire {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.as_bytes() {
            &[a, b, c] => Ok(Wire([a as char, b as char, c as char])),
            _ => Err(()),
        }
    }
}

impl Display for Wire {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.iter().collect::<String>())
    }
}

fn calculate(
    wire: Wire,
    connections: &HashMap<Wire, (Wire, Wire, Op)>,
    calculated: &mut HashMap<Wire, bool>,
) -> bool {
    match calculated.get(&wire) {
        Some(&value) => value,
        None => {
            let new_value = match connections.get(&wire).unwrap() {
                &(w1, w2, Op::AND) => {
                    calculate(w1, connections, calculated) && calculate(w2, connections, calculated)
                }
                &(w1, w2, Op::OR) => {
                    calculate(w1, connections, calculated) || calculate(w2, connections, calculated)
                }
                &(w1, w2, Op::XOR) => {
                    calculate(w1, connections, calculated) ^ calculate(w2, connections, calculated)
                }
            };
            calculated.insert(wire, new_value);
            new_value
        }
    }
}

fn part1(initials: &HashMap<Wire, bool>, connections: &HashMap<Wire, (Wire, Wire, Op)>) -> u64 {
    let mut calculated = initials.clone();
    connections
        .keys()
        .filter(|k| k.0[0] == 'z')
        .map(|k| (k, calculate(*k, connections, &mut calculated)))
        .map(|(k, v)| {
            (v as u64)
                << (k.0[1].to_digit(10).unwrap() as u64 * 10 + k.0[2].to_digit(10).unwrap() as u64)
        })
        .fold(0, |a, b| a | b)
}

fn part2(mut connections: HashMap<Wire, (Wire, Wire, Op)>) -> String {
    let mut mappings = connections
        .iter()
        .filter_map(|(c, (a, b, op))| {
            if c.0[0] == 'z' {
                return None;
            }
            if HashSet::from([a.0[0], b.0[0]]) == HashSet::from(['x', 'y']) {
                let mut new_name = *a;
                match op {
                    Op::AND => {
                        new_name.0[0] = 'c';
                        Some((c, new_name))
                    }
                    Op::XOR => {
                        new_name.0[0] = 'a';
                        Some((c, new_name))
                    }
                    _ => None,
                }
            } else {
                None
            }
        })
        .collect::<HashMap<_, _>>();
    for idx in 1..=44 {
        if let Some((d, _)) = connections.iter().find(|&(_, (a, c, op))| {
            let (a, c) = (mappings.get(&a).unwrap_or(a), mappings.get(&c).unwrap_or(c));
            let (a, c) = (*a.min(c), *a.max(c));
            matches!(op, Op::AND)
                && a == Wire::from_str(&format!("a{}{}", idx / 10, idx % 10)).unwrap()
                && (c == Wire::from_str(&format!("c{}{}", (idx - 1) / 10, (idx - 1) % 10)).unwrap()
                    || c == Wire::from_str(&format!("e{}{}", (idx - 1) / 10, (idx - 1) % 10))
                        .unwrap())
        }) {
            mappings.insert(
                d,
                Wire::from_str(&format!("d{}{}", idx / 10, idx % 10)).unwrap(),
            );
        } else {
            println!("Failed finding d{}{}", idx / 10, idx % 10);
            println!(
                "It should be a{}{} AND e{}{}",
                idx / 10,
                idx % 10,
                (idx - 1) / 10,
                (idx - 1) % 10
            );
            println!("Mappings: {mappings:?}");
        }
        if let Some((e, _)) = connections.iter().find(|&(_, (c, d, op))| {
            let (c, d) = (mappings.get(&c).unwrap_or(c), mappings.get(&d).unwrap_or(d));
            let (c, d) = (*c.min(d), *c.max(d));
            matches!(op, Op::OR)
                && c == Wire::from_str(&format!("c{}{}", idx / 10, idx % 10)).unwrap()
                && d == Wire::from_str(&format!("d{}{}", idx / 10, idx % 10)).unwrap()
        }) {
            mappings.insert(
                e,
                Wire::from_str(&format!("e{}{}", idx / 10, idx % 10)).unwrap(),
            );
        } else {
            println!("Failed finding e{}{}", idx / 10, idx % 10);
            println!(
                "It should be c{}{} OR d{}{}",
                idx / 10,
                idx % 10,
                idx / 10,
                idx % 10,
            );
            println!("Mappings: {mappings:?}");
        }
    }
    connections
        .iter()
        .map(|(c, (a, b, op))| {
            (
                mappings.get(&c).unwrap_or(c),
                (
                    mappings.get(&a).unwrap_or(a),
                    mappings.get(&b).unwrap_or(b),
                    op,
                ),
            )
        })
        .sorted()
        .for_each(|(c, (a, b, op))| {
            println!(
                "{c}: {a} {} {b}",
                match op {
                    Op::AND => "AND",
                    Op::XOR => "XOR",
                    Op::OR => "OR",
                }
            );
        });
    String::new()
}

fn main() {
    let initial_values: HashMap<Wire, bool> = stdin()
        .lines()
        .map(Result::unwrap)
        .take_while(|line| !line.is_empty())
        .filter_map(|line| {
            let (wire, value) = line.split_once(": ")?;
            Some((Wire::from_str(wire).ok()?, value == "1"))
        })
        .collect();
    let connections: HashMap<Wire, (Wire, Wire, Op)> = stdin()
        .lines()
        .map(Result::unwrap)
        .filter_map(|line| {
            let (deps, wire) = line.split_once(" -> ")?;
            let wire = Wire::from_str(wire).ok()?;
            if let [dep1, op, dep2] = &deps.split(" ").collect::<Vec<_>>()[..] {
                let dep1 = Wire::from_str(dep1).ok()?;
                let dep2 = Wire::from_str(dep2).ok()?;
                let op = match *op {
                    "AND" => Op::AND,
                    "OR" => Op::OR,
                    "XOR" => Op::XOR,
                    _ => return None,
                };
                Some((wire, (dep1, dep2, op)))
            } else {
                None
            }
        })
        .collect();
    println!("Part 1: {}", part1(&initial_values, &connections));
    println!("Part 2: {}", part2(connections));
}
