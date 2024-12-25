use std::{
    cmp::Reverse,
    collections::{hash_map::Entry, BinaryHeap, HashMap, HashSet},
};

const INPUT: &str = include_str!("../../inputs/day16.txt");

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Pos {
    x: i32,
    y: i32,
    dir: i32,
}

struct Visited {
    best_score: i32,
    prev: Vec<Pos>,
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
    let mut visited = HashMap::<Pos, Visited>::new();
    heap.push((
        Reverse(0),
        Pos {
            x: start_pos.0,
            y: start_pos.1,
            dir: 0,
        },
        None,
    ));

    let mut best_score = None;
    while let Some((Reverse(score), pos, prev_pos)) = heap.pop() {
        if grid[pos.y as usize][pos.x as usize] {
            continue;
        }
        if let Some(best_score) = best_score {
            if score > best_score {
                break;
            }
        }
        match visited.entry(pos) {
            Entry::Occupied(mut entry) => {
                if score > entry.get().best_score {
                    continue;
                }
                entry.get_mut().prev.extend(prev_pos);
            }
            Entry::Vacant(entry) => {
                entry.insert(Visited {
                    best_score: score,
                    prev: prev_pos.into_iter().collect(),
                });
            }
        }
        if pos.x == end_pos.0 && pos.y == end_pos.1 {
            best_score = Some(score);
            continue;
        }
        for dir in [1, 3] {
            heap.push((
                Reverse(score + 1000),
                Pos {
                    x: pos.x,
                    y: pos.y,
                    dir: (pos.dir + dir) % 4,
                },
                Some(pos),
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
            Some(pos),
        ));
    }

    let mut stack = Vec::new();
    let mut seen = HashSet::new();
    let mut cells = HashSet::new();
    for visited in visited.keys() {
        if visited.x == end_pos.0 && visited.y == end_pos.1 {
            stack.push(*visited);
            break;
        }
    }
    while let Some(pos) = stack.pop() {
        if !seen.insert(pos) {
            continue;
        }
        cells.insert((pos.x, pos.y));
        for prev in &visited[&pos].prev {
            stack.push(*prev);
        }
    }
    println!("{}", cells.len());
}
