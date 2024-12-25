use std::collections::VecDeque;

const INPUT: &str = include_str!("../../inputs/day20.txt");

fn main() {
    let mut start = (0, 0);
    let mut end = (0, 0);
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
                        start = (x as i32, y as i32);
                        false
                    }
                    'E' => {
                        end = (x as i32, y as i32);
                        false
                    }
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let w = grid[0].len();
    let h = grid.len();
    let mut distances = vec![vec![u64::MAX; w]; h];
    let mut queue = VecDeque::new();
    queue.push_back((end, 0));
    while let Some((pos, dist)) = queue.pop_front() {
        if dist >= distances[pos.1 as usize][pos.0 as usize] {
            continue;
        }
        distances[pos.1 as usize][pos.0 as usize] = dist;
        for (dx, dy) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let new_pos = (pos.0 + dx, pos.1 + dy);
            if new_pos.0 >= 0
                && new_pos.0 < w as i32
                && new_pos.1 >= 0
                && new_pos.1 < h as i32
                && !grid[new_pos.1 as usize][new_pos.0 as usize]
            {
                queue.push_back((new_pos, dist + 1));
            }
        }
    }

    let mut cheats = 0;
    for y in 1..(h - 1) {
        for x in 1..(w - 1) {
            if !grid[y][x] {
                continue;
            }
            if !grid[y][x - 1]
                && !grid[y][x + 1]
                && distances[y][x - 1] < u64::MAX
                && distances[y][x + 1] < u64::MAX
                && distances[y][x - 1].abs_diff(distances[y][x + 1]) >= 102
            {
                cheats += 1;
            }
            if !grid[y - 1][x]
                && !grid[y + 1][x]
                && distances[y - 1][x] < u64::MAX
                && distances[y + 1][x] < u64::MAX
                && distances[y - 1][x].abs_diff(distances[y + 1][x]) >= 102
            {
                cheats += 1;
            }
        }
    }
    println!("{}", cheats);
}
