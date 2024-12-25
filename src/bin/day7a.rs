const INPUT: &str = include_str!("../../inputs/day7.txt");

fn try_solve(result: i64, current: i64, values: &[i64]) -> bool {
    match values {
        [] => result == current,
        [v, rest @ ..] => {
            try_solve(result, current + v, rest) || try_solve(result, current * v, rest)
        }
    }
}

fn main() {
    let sum: i64 = INPUT
        .lines()
        .map(|line| {
            let (result, values) = line.split_once(": ").unwrap();
            let result = result.parse::<i64>().unwrap();
            let values = values
                .split(' ')
                .map(|v| v.parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            (result, values)
        })
        .filter(|(result, values)| try_solve(*result, 0, values))
        .map(|(result, _)| result)
        .sum();

    println!("{}", sum);
}
