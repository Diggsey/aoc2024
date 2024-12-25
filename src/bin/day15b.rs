const INPUT: &str = include_str!("../../inputs/day15.txt");

#[derive(Clone, Copy)]
enum Cell {
    Empty,
    Wall,
    BoxLeft,
    BoxRight,
}

fn push_boxes(pos: (i32, i32), dir: (i32, i32), grid: &mut [Vec<Cell>], dry_run: bool) -> bool {
    let cur = (pos.0 + dir.0, pos.1 + dir.1);
    let result = match grid[cur.1 as usize][cur.0 as usize] {
        Cell::Wall => false,
        Cell::BoxLeft => {
            if dir.1 == 0 {
                push_boxes(cur, dir, grid, dry_run)
            } else {
                push_boxes(cur, dir, grid, dry_run)
                    && push_boxes((cur.0 + 1, cur.1), dir, grid, dry_run)
            }
        }
        Cell::BoxRight => {
            if dir.1 == 0 {
                push_boxes(cur, dir, grid, dry_run)
            } else {
                push_boxes(cur, dir, grid, dry_run)
                    && push_boxes((cur.0 - 1, cur.1), dir, grid, dry_run)
            }
        }
        Cell::Empty => true,
    };
    if result && !dry_run {
        grid[cur.1 as usize][cur.0 as usize] = grid[pos.1 as usize][pos.0 as usize];
        grid[pos.1 as usize][pos.0 as usize] = Cell::Empty;
    }
    result
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
                .flat_map(|(x, c)| match c {
                    '#' => [Cell::Wall, Cell::Wall],
                    '.' => [Cell::Empty, Cell::Empty],
                    'O' => [Cell::BoxLeft, Cell::BoxRight],
                    '@' => {
                        pos = (x as i32 * 2, y as i32);
                        [Cell::Empty, Cell::Empty]
                    }
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    for dir in directions_str.chars() {
        let (dx, dy) = match dir {
            '^' => (0, -1),
            'v' => (0, 1),
            '<' => (-1, 0),
            '>' => (1, 0),
            _ => continue,
        };
        if push_boxes(pos, (dx, dy), &mut grid, true) {
            push_boxes(pos, (dx, dy), &mut grid, false);
            pos.0 += dx;
            pos.1 += dy;
        }
    }

    let total = grid
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter_map(|(x, cell)| {
                    if matches!(cell, Cell::BoxLeft) {
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
