use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

const INPUT: &str = include_str!("../../inputs/day16.txt");

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Pos {
    x: i32,
    y: i32,
    dir: i32,
}

fn main() {
    let mut start_pos = (0, 0);
    let mut end_pos = (0, 0);
    let grid = INPUT
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '#' => true,
                    '.' => false,
                    'S' => {
                        start_pos = (x as i32, y as i32);
                        false
                    }
                    'E' => {
                        end_pos = (x as i32, y as i32);
                        false
                    }
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut heap = BinaryHeap::new();
    let mut visited = HashSet::new();
    heap.push((
        Reverse(0),
        Pos {
            x: start_pos.0,
            y: start_pos.1,
            dir: 0,
        },
    ));

    while let Some((Reverse(score), pos)) = heap.pop() {
        if grid[pos.y as usize][pos.x as usize] || !visited.insert(pos) {
            continue;
        }
        if pos.x == end_pos.0 && pos.y == end_pos.1 {
            println!("{score}");
            break;
        }
        for dir in [1, 3] {
            heap.push((
                Reverse(score + 1000),
                Pos {
                    x: pos.x,
                    y: pos.y,
                    dir: (pos.dir + dir) % 4,
                },
            ));
        }
        let (dx, dy) = match pos.dir {
            0 => (1, 0),
            1 => (0, 1),
            2 => (-1, 0),
            3 => (0, -1),
            _ => unreachable!(),
        };
        heap.push((
            Reverse(score + 1),
            Pos {
                x: pos.x + dx,
                y: pos.y + dy,
                dir: pos.dir,
            },
        ));
    }
}
