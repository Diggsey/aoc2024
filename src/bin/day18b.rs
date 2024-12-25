use std::collections::{HashMap, HashSet, VecDeque};

const INPUT: &str = include_str!("../../inputs/day18.txt");

const SIZE: i32 = 71;

fn main() {
    let walls: HashMap<_, _> = INPUT
        .lines()
        .enumerate()
        .map(|(t, line)| {
            let (x, y) = line.split_once(',').unwrap();
            ((x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap()), t)
        })
        .collect();
    'next: for total_time in 0.. {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(((0, 0), 0));
        while let Some((pos, score)) = queue.pop_front() {
            if pos == (SIZE - 1, SIZE - 1) {
                continue 'next;
            }
            if !visited.insert(pos) {
                continue;
            }
            for (dx, dy) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let new_pos = (pos.0 + dx, pos.1 + dy);
                if new_pos.0 < 0 || new_pos.0 >= SIZE || new_pos.1 < 0 || new_pos.1 >= SIZE {
                    continue;
                }
                if walls.get(&new_pos).is_some_and(|t| *t <= total_time) {
                    continue;
                }
                queue.push_back((new_pos, score + 1));
            }
        }
        let coord = walls.iter().find(|(_, t)| **t == total_time).unwrap().0;
        println!("{},{}", coord.0, coord.1);
        break;
    }
}
