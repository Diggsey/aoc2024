const INPUT: &str = include_str!("../../inputs/day21.txt");

// right, up, down, left
fn expand(from_pos: (i64, i64), to_pos: (i64, i64), outer_pos: (i64, i64), steps: i64) -> i64 {
    if from_pos == (-2, 0) {
        return i64::MAX;
    }
    if steps > 0 {
        if from_pos == to_pos {
            return expand(outer_pos, (0, 0), (0, 0), steps - 1);
        }
        let delta = (to_pos.0 - from_pos.0, to_pos.1 - from_pos.1);
        let mut best = i64::MAX;
        if delta.0 != 0 {
            let loc = if delta.0 > 0 { (0, -1) } else { (-2, -1) };
            let result = expand(outer_pos, loc, (0, 0), steps - 1).saturating_add(expand(
                (from_pos.0 + delta.0.signum(), from_pos.1),
                to_pos,
                loc,
                steps,
            ));
            if result < best {
                best = result;
            }
        }
        if delta.1 != 0 {
            let loc = if delta.1 > 0 { (-1, 0) } else { (-1, -1) };
            let result = expand(outer_pos, loc, (0, 0), steps - 1).saturating_add(expand(
                (from_pos.0, from_pos.1 + delta.1.signum()),
                to_pos,
                loc,
                steps,
            ));
            if result < best {
                best = result;
            }
        }
        best
    } else {
        1
    }
}

fn main() {
    let mut total = 0;
    let mut prev_pos = (0, 0);
    for line in INPUT.lines() {
        let number = line[0..line.len() - 1].parse::<i64>().unwrap();
        let mut result = 0;
        for c in line.chars() {
            let target_pos = match c {
                'A' => (0, 0),
                '0' => (-1, 0),
                '1' => (-2, 1),
                '2' => (-1, 1),
                '3' => (0, 1),
                '4' => (-2, 2),
                '5' => (-1, 2),
                '6' => (0, 2),
                '7' => (-2, 3),
                '8' => (-1, 3),
                '9' => (0, 3),
                _ => unreachable!(),
            };
            result += expand(prev_pos, target_pos, (0, 0), 3);
            prev_pos = target_pos;
        }
        println!("{}, {}", result, number);
        total += result * number;
    }
    println!("{}", total);
}
