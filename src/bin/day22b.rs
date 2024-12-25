use std::collections::{HashMap, HashSet};

use itertools::Itertools;

const INPUT: &str = include_str!("../../inputs/day22.txt");

fn mix(a: i64, b: i64) -> i64 {
    a ^ b
}

fn prune(a: i64) -> i64 {
    a % 16777216
}

struct Secret(i64);

impl Iterator for Secret {
    type Item = i64;

    fn next(&mut self) -> Option<i64> {
        let cur = self.0;
        self.0 = prune(mix(self.0, self.0 * 64));
        self.0 = prune(mix(self.0, self.0 / 32));
        self.0 = prune(mix(self.0, self.0 * 2048));
        Some(cur % 10)
    }
}

fn main() {
    let mut values = HashMap::<_, i64>::new();
    for line in INPUT.lines() {
        let secret = Secret(line.parse().unwrap());
        let mut visited = HashSet::new();
        for ((_, a), (_, b), (_, c), (initial_price, d)) in secret
            .tuple_windows()
            .take(2000)
            .map(|(a, b)| (b, (b - a)))
            .tuple_windows()
        {
            if visited.insert((a, b, c, d)) {
                *values.entry((a, b, c, d)).or_default() += initial_price;
            }
        }
    }
    let total = values.values().max().unwrap();
    println!("{}", total);
}
