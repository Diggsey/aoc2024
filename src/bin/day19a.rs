use regex::Regex;

const INPUT: &str = include_str!("../../inputs/day19.txt");

fn main() {
    let (towels_str, patterns_str) = INPUT.split_once("\n\n").unwrap();
    let regex = Regex::new(&format!("^({})*$", towels_str.replace(", ", "|"))).unwrap();
    let count = patterns_str
        .lines()
        .filter(|pattern| regex.is_match(pattern))
        .count();
    println!("{}", count);
}
