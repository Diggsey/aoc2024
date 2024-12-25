use regex::RegexBuilder;

const INPUT: &str = include_str!("../../inputs/day3.txt");

fn main() {
    let r = RegexBuilder::new(r#"do\(\)|don't\(\)|mul\(([0-9]+),([0-9]+)\)"#)
        .multi_line(true)
        .build()
        .unwrap();
    let mut enabled = true;
    let mut result = 0;
    for cap in r.captures_iter(INPUT) {
        match &cap[0] {
            "do()" => enabled = true,
            "don't()" => enabled = false,
            _ => {
                if enabled {
                    let a = cap[1].parse::<i32>().unwrap();
                    let b = cap[2].parse::<i32>().unwrap();
                    result += a * b;
                }
            }
        }
    }
    println!("{}", result);
}
