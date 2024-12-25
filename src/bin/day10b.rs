use std::collections::VecDeque;

const INPUT: &str = include_str!("../../inputs/day10.txt");

fn walk(grid: &[Vec<u32>], x: usize, y: usize, w: usize, h: usize) -> u32 {
    let mut visited = vec![false; w * h];
    let mut routes = vec![0; w * h];
    let mut queue = VecDeque::new();
    queue.push_front((x, y));
    let mut total = 0;
    routes[y * w + x] = 1;

    while let Some((x, y)) = queue.pop_front() {
        if visited[y * w + x] {
            continue;
        }

        visited[y * w + x] = true;
        let next_v = grid[y][x] + 1;
        let num_routes = routes[y * w + x];
        if next_v == 10 {
            total += num_routes;
        } else {
            if x < w - 1 && grid[y][x + 1] == next_v {
                queue.push_back((x + 1, y));
                routes[y * w + x + 1] += num_routes;
            }
            if y < h - 1 && grid[y + 1][x] == next_v {
                queue.push_back((x, y + 1));
                routes[(y + 1) * w + x] += num_routes;
            }
            if x > 0 && grid[y][x - 1] == next_v {
                queue.push_back((x - 1, y));
                routes[y * w + x - 1] += num_routes;
            }
            if y > 0 && grid[y - 1][x] == next_v {
                queue.push_back((x, y - 1));
                routes[(y - 1) * w + x] += num_routes;
            }
        }
    }

    total
}

fn main() {
    let grid = INPUT
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let w = grid[0].len();
    let h = grid.len();

    let mut total = 0;
    for y in 0..h {
        for x in 0..w {
            if grid[y][x] == 0 {
                total += walk(&grid, x, y, w, h);
            }
        }
    }
    println!("{}", total);
}
