use std::collections::HashMap;

const INPUT: &str = include_str!("../../inputs/day11.txt");

fn solve(cache: &mut HashMap<(u64, u64), u64>, stone: u64, count: u64) -> u64 {
    if count == 0 {
        return 1;
    }
    if let Some(&result) = cache.get(&(stone, count)) {
        return result;
    }
    let total = if stone == 0 {
        solve(cache, 1, count - 1)
    } else {
        let num_digits = stone.ilog10() + 1;
        if num_digits % 2 == 0 {
            let f = 10u64.pow(num_digits / 2);
            let a = stone / f;
            let b = stone % f;
            solve(cache, a, count - 1) + solve(cache, b, count - 1)
        } else {
            solve(cache, stone * 2024, count - 1)
        }
    };
    cache.insert((stone, count), total);
    total
}

fn main() {
    let mut cache = HashMap::new();
    let total: u64 = INPUT
        .split_ascii_whitespace()
        .map(|x| solve(&mut cache, x.parse::<u64>().unwrap(), 75))
        .sum();
    println!("{}", total);
}
