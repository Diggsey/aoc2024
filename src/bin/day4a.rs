const INPUT: &str = include_str!("../../inputs/day4.txt");

fn main() {
    let grid = INPUT
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let w = grid[0].len();
    let h = grid.len();
    let mut count = 0;
    for y in 0..h {
        for x in 0..w {
            if grid[y][x] == 'X' {
                for dx in -1..=1 {
                    'next_dir: for dy in -1..=1 {
                        if dy == 0 && dx == 0 {
                            continue 'next_dir;
                        }
                        for (i, l) in "MAS".chars().enumerate() {
                            let nx = ((x as i32) + dx * (i + 1) as i32) as usize;
                            let ny = ((y as i32) + dy * (i + 1) as i32) as usize;
                            if nx >= w || ny >= h {
                                continue 'next_dir;
                            }
                            if grid[ny][nx] != l {
                                continue 'next_dir;
                            }
                        }
                        count += 1;
                    }
                }
            }
        }
    }
    println!("{}", count);
}
