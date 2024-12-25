use itertools::Itertools;

const INPUT: &str = include_str!("../../inputs/day2.txt");

fn main() {
    let result = INPUT
        .lines()
        .filter(|line| {
            let values: Vec<_> = line
                .split_whitespace()
                .map(|item| item.parse::<i32>().unwrap())
                .collect();
            values
                .iter()
                .tuple_windows()
                .all(|(a, b)| b - a >= 1 && b - a <= 3)
                || values
                    .iter()
                    .tuple_windows()
                    .all(|(b, a)| b - a >= 1 && b - a <= 3)
        })
        .count();
    println!("{}", result);
}
