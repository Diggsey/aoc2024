const INPUT: &str = include_str!("../../inputs/day15.txt");

enum Cell {
    Empty,
    Wall,
    Box,
}

fn main() {
    let (grid_str, directions_str) = INPUT.split_once("\n\n").unwrap();
    let mut pos = (0, 0);
    let mut grid = grid_str
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '#' => Cell::Wall,
                    '.' => Cell::Empty,
                    'O' => Cell::Box,
                    '@' => {
                        pos = (x as i32, y as i32);
                        Cell::Empty
                    }
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    'next_dir: for dir in directions_str.chars() {
        let (dx, dy) = match dir {
            '^' => (0, -1),
            'v' => (0, 1),
            '<' => (-1, 0),
            '>' => (1, 0),
            _ => continue,
        };
        let mut cur = pos;
        loop {
            cur.0 += dx;
            cur.1 += dy;
            match grid[cur.1 as usize][cur.0 as usize] {
                Cell::Wall => continue 'next_dir,
                Cell::Box => {}
                Cell::Empty => {
                    grid[cur.1 as usize][cur.0 as usize] = Cell::Box;
                    break;
                }
            }
        }
        pos.0 += dx;
        pos.1 += dy;
        grid[pos.1 as usize][pos.0 as usize] = Cell::Empty;
    }

    let total = grid
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter_map(|(x, cell)| {
                    if matches!(cell, Cell::Box) {
                        Some(y * 100 + x)
                    } else {
                        None
                    }
                })
                .sum::<usize>()
        })
        .sum::<usize>();
    println!("{}", total);
}
