const INPUT: &str = include_str!("../../inputs/day4.txt");

fn main() {
    let grid = INPUT
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let w = grid[0].len();
    let h = grid.len();
    let mut count = 0;
    for y in 1..h - 1 {
        for x in 1..w - 1 {
            if grid[y][x] == 'A' {
                let a = grid[y - 1][x - 1];
                let b = grid[y + 1][x + 1];
                let c = grid[y - 1][x + 1];
                let d = grid[y + 1][x - 1];
                if ((a == 'M' && b == 'S') || (a == 'S' && b == 'M'))
                    && ((c == 'M' && d == 'S') || (c == 'S' && d == 'M'))
                {
                    count += 1;
                }
            }
        }
    }
    println!("{}", count);
}
