use std::collections::{HashMap, HashSet};

use itertools::Itertools;

const INPUT: &str = include_str!("../../inputs/day8.txt");

fn main() {
    let mut antennas = HashMap::<_, HashSet<_>>::new();
    let mut h = 0;
    let mut w = 0;
    for (y, line) in INPUT.lines().enumerate() {
        h += 1;
        for (x, c) in line.chars().enumerate() {
            if h == 1 {
                w += 1;
            }
            if c != '.' {
                antennas.entry(c).or_default().insert((x as i32, y as i32));
            }
        }
    }

    let mut antinodes = HashSet::<(i32, i32)>::new();
    for antenna in antennas.values() {
        for ((x0, y0), (x1, y1)) in antenna.iter().tuple_combinations() {
            let dx = x1 - x0;
            let dy = y1 - y0;
            for i in 0.. {
                let ax = x1 + dx * i;
                let ay = y1 + dy * i;
                if ax >= 0 && ax < w && ay >= 0 && ay < h {
                    antinodes.insert((ax, ay));
                } else {
                    break;
                }
            }
            for i in 0.. {
                let ax = x0 - dx * i;
                let ay = y0 - dy * i;
                if ax >= 0 && ax < w && ay >= 0 && ay < h {
                    antinodes.insert((ax, ay));
                } else {
                    break;
                }
            }
        }
    }

    println!("{}", antinodes.len());
}
