const INPUT: &str = include_str!("../../inputs/day22.txt");

fn mix(a: u64, b: u64) -> u64 {
    a ^ b
}

fn prune(a: u64) -> u64 {
    a % 16777216
}

fn advance(mut a: u64) -> u64 {
    a = prune(mix(a, a * 64));
    a = prune(mix(a, a / 32));
    a = prune(mix(a, a * 2048));
    a
}

fn main() {
    let total = INPUT
        .lines()
        .map(|line| {
            let mut a = line.parse().unwrap();
            for _ in 0..2000 {
                a = advance(a);
            }
            a
        })
        .sum::<u64>();
    println!("{}", total);
}
