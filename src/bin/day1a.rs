const INPUT: &str = include_str!("../../inputs/day1.txt");

fn main() {
    let (mut a, mut b): (Vec<_>, Vec<_>) = INPUT
        .lines()
        .map(|line| {
            let (a, b) = line.split_once("   ").unwrap();
            (a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap())
        })
        .unzip();
    a.sort();
    b.sort();
    let dist: i32 = a.into_iter().zip(b).map(|(a, b)| (a - b).abs()).sum();
    println!("{}", dist);
}
