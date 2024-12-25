use std::cmp::Ordering;

const INPUT: &str = include_str!("../../inputs/day5.txt");

fn main() {
    let (rules, updates) = INPUT.split_once("\n\n").unwrap();
    let rules = rules
        .lines()
        .map(|line| line.split_once('|').unwrap())
        .map(|(a, b)| (a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap()))
        .collect::<Vec<_>>();

    let sum = updates
        .lines()
        .map(|line| {
            line.split(',')
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .filter(|update| {
            !rules.iter().all(|&(a, b)| {
                let ai = update.iter().position(|&x| x == a);
                let bi = update.iter().position(|&x| x == b);
                if let (Some(ai), Some(bi)) = (ai, bi) {
                    ai < bi
                } else {
                    true
                }
            })
        })
        .map(|mut update| {
            update.sort_by(|&a, &b| {
                if rules.contains(&(a, b)) {
                    Ordering::Less
                } else if rules.contains(&(b, a)) {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            });
            update
        })
        .map(|update| update[update.len() / 2])
        .sum::<i32>();
    println!("{}", sum);
}
