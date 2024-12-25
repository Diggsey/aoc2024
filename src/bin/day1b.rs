use itertools::Itertools;

const INPUT: &str = include_str!("../../inputs/day1.txt");

fn main() {
    let (a, b): (Vec<_>, Vec<_>) = INPUT
        .lines()
        .map(|line| {
            let (a, b) = line.split_once("   ").unwrap();
            (a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap())
        })
        .unzip();
    let counts = b.into_iter().counts();
    let similarity = a
        .into_iter()
        .map(|a| (a as usize) * counts.get(&a).copied().unwrap_or(0))
        .sum::<usize>();
    println!("{}", similarity);
}
