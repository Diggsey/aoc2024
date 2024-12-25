use std::collections::HashSet;

const INPUT: &str = include_str!("../../inputs/day6.txt");

fn in_bounds(pos: (i32, i32), size: (i32, i32)) -> bool {
    pos.0 >= 0 && pos.0 < size.0 && pos.1 >= 0 && pos.1 < size.1
}

fn main() {
    let mut start_pos = (0, 0);
    let mut dir = (0, -1);
    let board = INPUT
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    if c == '^' {
                        start_pos = (x as i32, y as i32);
                    }
                    c == '#'
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let size = (board[0].len() as i32, board.len() as i32);
    let mut visited = HashSet::new();

    let mut pos = start_pos;
    while in_bounds(pos, size) {
        visited.insert(pos);
        let new_pos = (pos.0 + dir.0, pos.1 + dir.1);
        if in_bounds(new_pos, size) && board[new_pos.1 as usize][new_pos.0 as usize] {
            dir = (-dir.1, dir.0);
        } else {
            pos = new_pos;
        }
    }

    let result = visited
        .into_iter()
        .filter(|obstruction| {
            if start_pos == *obstruction {
                return false;
            }

            let mut visited = HashSet::new();
            let mut pos = start_pos;
            let mut dir = (0, -1);
            while in_bounds(pos, size) {
                if !visited.insert((pos, dir)) {
                    return true;
                }
                let new_pos = (pos.0 + dir.0, pos.1 + dir.1);
                if in_bounds(new_pos, size)
                    && (board[new_pos.1 as usize][new_pos.0 as usize] || new_pos == *obstruction)
                {
                    dir = (-dir.1, dir.0);
                } else {
                    pos = new_pos;
                }
            }
            false
        })
        .count();
    println!("{}", result);
}
