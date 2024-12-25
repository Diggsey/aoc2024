use std::collections::HashMap;

const INPUT: &str = include_str!("../../inputs/day21.txt");

#[derive(Hash, Eq, PartialEq)]
struct Args {
    // The start position of the target robot
    from_pos: (i64, i64),
    // The desired position of the target robot
    to_pos: (i64, i64),
    // The position of the outer robot
    outer_pos: (i64, i64),
    // The number of robots above us
    steps: i64,
}

fn expand(args: Args, cache: &mut HashMap<Args, i64>) -> i64 {
    if args.from_pos == (-2, 0) {
        return i64::MAX;
    }
    if args.steps > 0 {
        if args.from_pos == args.to_pos {
            return expand(
                Args {
                    from_pos: args.outer_pos,
                    to_pos: (0, 0),
                    outer_pos: (0, 0),
                    steps: args.steps - 1,
                },
                cache,
            );
        }
        if let Some(&result) = cache.get(&args) {
            return result;
        }
        let delta = (
            args.to_pos.0 - args.from_pos.0,
            args.to_pos.1 - args.from_pos.1,
        );
        let mut best = i64::MAX;
        if delta.0 != 0 {
            let loc = if delta.0 > 0 { (0, -1) } else { (-2, -1) };
            let result = expand(
                Args {
                    from_pos: args.outer_pos,
                    to_pos: loc,
                    outer_pos: (0, 0),
                    steps: args.steps - 1,
                },
                cache,
            )
            .saturating_add(expand(
                Args {
                    from_pos: (args.from_pos.0 + delta.0.signum(), args.from_pos.1),
                    to_pos: args.to_pos,
                    outer_pos: loc,
                    steps: args.steps,
                },
                cache,
            ));
            if result < best {
                best = result;
            }
        }
        if delta.1 != 0 {
            let loc = if delta.1 > 0 { (-1, 0) } else { (-1, -1) };
            let result = expand(
                Args {
                    from_pos: args.outer_pos,
                    to_pos: loc,
                    outer_pos: (0, 0),
                    steps: args.steps - 1,
                },
                cache,
            )
            .saturating_add(expand(
                Args {
                    from_pos: (args.from_pos.0, args.from_pos.1 + delta.1.signum()),
                    to_pos: args.to_pos,
                    outer_pos: loc,
                    steps: args.steps,
                },
                cache,
            ));
            if result < best {
                best = result;
            }
        }
        cache.insert(args, best);
        best
    } else {
        1
    }
}

fn main() {
    let mut cache = HashMap::new();
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
            result += expand(
                Args {
                    from_pos: prev_pos,
                    to_pos: target_pos,
                    outer_pos: (0, 0),
                    steps: 26,
                },
                &mut cache,
            );
            prev_pos = target_pos;
        }
        println!("{}, {}", result, number);
        total += result * number;
    }
    println!("{}", total);
}
