use regex::RegexBuilder;

const INPUT: &str = include_str!("../../inputs/day3.txt");

fn main() {
    let r = RegexBuilder::new(r#"mul\(([0-9]+),([0-9]+)\)"#)
        .multi_line(true)
        .build()
        .unwrap();
    let result = r
        .captures_iter(INPUT)
        .map(|cap| {
            let a = cap[1].parse::<i32>().unwrap();
            let b = cap[2].parse::<i32>().unwrap();
            a * b
        })
        .sum::<i32>();
    println!("{}", result);
}
