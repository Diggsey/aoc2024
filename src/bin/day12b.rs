use std::collections::{BTreeSet, HashMap};

use itertools::Itertools;

const INPUT: &str = include_str!("../../inputs/day12.txt");

fn main() {
    let grid = INPUT
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let w = grid[0].len();
    let h = grid.len();
    let mut visited = vec![false; w * h];
    let mut cost = 0;
    for y in 0..h {
        for x in 0..w {
            if visited[y * w + x] {
                continue;
            }
            let current = grid[y][x];
            let mut top_edges = HashMap::<_, BTreeSet<_>>::new();
            let mut bottom_edges = HashMap::<_, BTreeSet<_>>::new();
            let mut left_edges = HashMap::<_, BTreeSet<_>>::new();
            let mut right_edges = HashMap::<_, BTreeSet<_>>::new();
            let mut area = 0;
            let mut stack = Vec::new();
            stack.push((x, y));

            while let Some((x, y)) = stack.pop() {
                if visited[y * w + x] {
                    continue;
                }
                area += 1;
                visited[y * w + x] = true;

                if x > 0 && grid[y][x - 1] == current {
                    stack.push((x - 1, y));
                } else {
                    left_edges.entry(x).or_default().insert(y);
                }
                if y > 0 && grid[y - 1][x] == current {
                    stack.push((x, y - 1));
                } else {
                    top_edges.entry(y).or_default().insert(x);
                }
                if x < w - 1 && grid[y][x + 1] == current {
                    stack.push((x + 1, y));
                } else {
                    right_edges.entry(x + 1).or_default().insert(y);
                }
                if y < h - 1 && grid[y + 1][x] == current {
                    stack.push((x, y + 1));
                } else {
                    bottom_edges.entry(y + 1).or_default().insert(x);
                }
            }
            let mut sides = 0;
            for edge in left_edges
                .values()
                .chain(top_edges.values())
                .chain(right_edges.values())
                .chain(bottom_edges.values())
            {
                sides += edge
                    .iter()
                    .copied()
                    .tuple_windows()
                    .filter(|&(a, b)| a + 1 != b)
                    .count()
                    + 1;
            }
            cost += area * sides;
        }
    }
    println!("{}", cost);
}
