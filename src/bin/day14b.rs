use scan_fmt::scan_fmt;

const INPUT: &str = include_str!("../../inputs/day14.txt");
const WIDTH: i64 = 101;
const HEIGHT: i64 = 103;

struct Robot {
    px: i64,
    py: i64,
    vx: i64,
    vy: i64,
}

impl Robot {
    fn position_at(&self, ticks: i64) -> (i64, i64) {
        let nx = (self.px + ticks * self.vx).rem_euclid(WIDTH);
        let ny = (self.py + ticks * self.vy).rem_euclid(HEIGHT);
        (nx, ny)
    }
}

fn is_christmas_tree(grid: &[[bool; WIDTH as usize]; HEIGHT as usize]) -> bool {
    grid[44].iter().filter(|&&a| a).count() > 30 && grid[76].iter().filter(|&&a| a).count() > 30
}

fn main() {
    let robots: Vec<_> = INPUT
        .lines()
        .map(|line| {
            let (px, py, vx, vy) = scan_fmt!(line, "p={},{} v={},{}", i64, i64, i64, i64).unwrap();
            Robot { px, py, vx, vy }
        })
        .collect();
    for ticks in 0.. {
        let mut grid = [[false; WIDTH as usize]; HEIGHT as usize];
        for robot in &robots {
            let (nx, ny) = robot.position_at(ticks);
            grid[ny as usize][nx as usize] = true;
        }
        if !is_christmas_tree(&grid) {
            continue;
        }
        let mut result = String::new();
        for row in &grid {
            for &a in row {
                result.push(if a { '#' } else { ' ' });
            }
            result.push('\n');
        }
        println!("{}\n{}", ticks, result);
        break;
    }
}
