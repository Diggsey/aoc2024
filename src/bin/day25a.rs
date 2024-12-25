use itertools::{Either, Itertools};

const INPUT: &str = include_str!("../../inputs/day25.txt");

fn main() {
    let (locks, keys): (Vec<_>, Vec<_>) = INPUT.split("\n\n").partition_map(|item_str| {
        let mut heights = Vec::new();
        for line in item_str.lines() {
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    if heights.len() <= x {
                        heights.resize(x + 1, -1);
                    }
                    heights[x] += 1;
                }
            }
        }
        if item_str.starts_with("#") {
            Either::Left(heights)
        } else {
            Either::Right(heights)
        }
    });

    let result = keys
        .iter()
        .cartesian_product(&locks)
        .filter(|(key, lock)| key.iter().zip(lock.iter()).all(|(k, l)| k + l <= 5))
        .count();

    println!("{}", result);
}
