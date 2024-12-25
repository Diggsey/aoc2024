use std::collections::{BTreeSet, HashMap, HashSet};

use itertools::Itertools;

const INPUT: &str = include_str!("../../inputs/day23.txt");

fn main() {
    let edges: HashMap<_, HashSet<_>> = INPUT
        .lines()
        .flat_map(|line| {
            let (a, b) = line.split_once('-').unwrap();
            [(a, b), (b, a)]
        })
        .into_grouping_map()
        .collect();

    let triplets = edges
        .iter()
        .flat_map(|(a, bs)| {
            bs.iter()
                .copied()
                .tuple_combinations()
                .filter(|(b, c)| edges[b].contains(c))
                .map(|(b, c)| [a, b, c].into_iter().collect::<BTreeSet<_>>())
        })
        .collect::<HashSet<_>>();

    let count = triplets
        .iter()
        .filter(|t| t.iter().any(|x| x.starts_with("t")))
        .count();

    println!("{}", count);
}
