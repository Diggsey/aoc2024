use std::collections::{BTreeSet, HashMap, HashSet};

use itertools::Itertools;

const INPUT: &str = include_str!("../../inputs/day23.txt");

fn expand_connected(
    edges: &HashMap<&'static str, BTreeSet<&'static str>>,
    current: HashSet<BTreeSet<&'static str>>,
) -> HashSet<BTreeSet<&'static str>> {
    let mut result = HashSet::new();
    for subgraph in current {
        let mut it = subgraph.iter();
        let first_node = *it.next().unwrap();
        let mut set: BTreeSet<_> = edges[first_node].difference(&subgraph).copied().collect();
        for other_node in it {
            let other_set = &edges[other_node];
            set.retain(|node| other_set.contains(node));
        }
        for new_node in set {
            let mut new_subgraph = subgraph.clone();
            new_subgraph.insert(new_node);
            result.insert(new_subgraph);
        }
    }
    result
}

fn main() {
    let edges: HashMap<_, BTreeSet<_>> = INPUT
        .lines()
        .flat_map(|line| {
            let (a, b) = line.split_once('-').unwrap();
            [(a, b), (b, a)]
        })
        .into_grouping_map()
        .collect();

    let mut current = edges
        .iter()
        .flat_map(|(a, bs)| {
            bs.iter()
                .copied()
                .map(|b| [a, b].into_iter().collect::<BTreeSet<_>>())
        })
        .collect::<HashSet<_>>();

    while current.len() > 1 {
        current = expand_connected(&edges, current);
    }

    let secret = current.into_iter().flatten().join(",");

    println!("{}", secret);
}
