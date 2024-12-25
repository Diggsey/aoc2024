use std::cmp::Ordering;

use scan_fmt::scan_fmt;

const INPUT: &str = include_str!("../../inputs/day14.txt");

fn main() {
    const TICKS: i64 = 100;
    const WIDTH: i64 = 101;
    const HEIGHT: i64 = 103;
    let mut quadrants = [0; 4];
    for line in INPUT.lines() {
        let (px, py, vx, vy) = scan_fmt!(line, "p={},{} v={},{}", i64, i64, i64, i64).unwrap();
        let nx = (px + TICKS * vx).rem_euclid(WIDTH);
        let ny = (py + TICKS * vy).rem_euclid(HEIGHT);
        let qx = nx.cmp(&(WIDTH / 2));
        let qy = ny.cmp(&(HEIGHT / 2));
        match (qx, qy) {
            (Ordering::Less, Ordering::Less) => quadrants[0] += 1,
            (Ordering::Greater, Ordering::Less) => quadrants[1] += 1,
            (Ordering::Less, Ordering::Greater) => quadrants[2] += 1,
            (Ordering::Greater, Ordering::Greater) => quadrants[3] += 1,
            _ => {}
        }
    }
    let total = quadrants.iter().product::<i64>();
    println!("{}", total);
}
