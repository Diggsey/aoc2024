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
            let mut perimiter = 0;
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
                    perimiter += 1;
                }
                if y > 0 && grid[y - 1][x] == current {
                    stack.push((x, y - 1));
                } else {
                    perimiter += 1;
                }
                if x < w - 1 && grid[y][x + 1] == current {
                    stack.push((x + 1, y));
                } else {
                    perimiter += 1;
                }
                if y < h - 1 && grid[y + 1][x] == current {
                    stack.push((x, y + 1));
                } else {
                    perimiter += 1;
                }
            }
            cost += area * perimiter;
        }
    }
    println!("{}", cost);
}
