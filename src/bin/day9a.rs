const INPUT: &str = include_str!("../../inputs/day9.txt");

fn main() {
    let mut blocks = Vec::new();
    for (i, c) in INPUT.trim().chars().enumerate() {
        let size = c.to_digit(10).unwrap();
        let v = if i % 2 == 0 { Some(i / 2) } else { None };
        for _ in 0..size {
            blocks.push(v);
        }
    }

    let mut left = 0;
    let mut right = blocks.len() - 1;
    while right > left {
        if blocks[left].is_some() {
            left += 1;
        } else if blocks[right].is_none() {
            right -= 1;
        } else {
            blocks.swap(left, right);
        }
    }

    let checksum: i64 = blocks
        .iter()
        .flatten()
        .enumerate()
        .map(|(i, j)| i as i64 * *j as i64)
        .sum();

    println!("{}", checksum);
}
