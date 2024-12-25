use std::collections::HashMap;

use scan_fmt::scan_fmt;

const INPUT: &str = include_str!("../../inputs/day17.txt");

// b = a % 8
// b ^= 3
// c = a >> b
// a = a >> 3
// b = b ^ c
// b ^= 5
// out b
// if a != 0 goto 0

const ADV: u8 = 0;
const BXL: u8 = 1;
const BST: u8 = 2;
const JNZ: u8 = 3;
const BXC: u8 = 4;
const OUT: u8 = 5;
const BDV: u8 = 6;
const CDV: u8 = 7;

// Recursively search for a sequence of inputs that produces the given output, storing the
// results in `results`.
fn search(target: &[u8], suffix: u16, inputs: &HashMap<u8, Vec<u16>>, results: &mut Vec<u128>) {
    // If there is still output to produce, get the first value we need to output.
    if let &[head, ref tail @ ..] = target {
        let start = results.len();

        // For each possible input that produces the first output, recursively search for the
        // rest of the output.
        for &v in &inputs[&head] {
            // The last 7 bits of the input must match the suffix provided, since that suffix
            // was already used to produce previous output values.
            if v & ((1 << 7) - 1) == suffix {
                // We need to make sure that on recursion we only consider inputs that match
                // the upper bits of the current input, since we already "used" those bits.
                search(tail, v >> 3, inputs, results);
            }
        }

        // Go through all the results produced by the recursive calls and append the suffix.
        for result in &mut results[start..] {
            *result = (*result << 3) | suffix as u128;
        }
    } else {
        // We have no more output to produce, so we can just append the bits that have already
        // been used as input.
        results.push(suffix as u128);
    }
}

fn main() {
    let (_a, initial_b, initial_c, program) = scan_fmt!(
        INPUT,
        "Register A: {}\nRegister B: {}\nRegister C: {}\n\nProgram: {}",
        u128,
        u128,
        u128,
        String
    )
    .unwrap();

    let program: Vec<_> = program
        .split(',')
        .map(|p| p.parse::<u8>().unwrap())
        .collect();

    // Each iteration of the program only looks at the lowest 10 bits of the input,
    // so we can precompute the first output for each possible input.
    let mut inputs = HashMap::<u8, Vec<u16>>::new();
    'next: for initial_a in 0..(1 << 10) {
        // Run the program with this input until it outputs something.
        let mut ip = 0;
        let (mut a, mut b, mut c) = (initial_a, initial_b, initial_c);
        while ip < program.len() - 1 {
            let literal = program[ip + 1] as u128;
            let combo = match literal {
                0..4 => literal,
                4 => a,
                5 => b,
                6 => c,
                _ => unreachable!(),
            };
            match program[ip] {
                ADV => {
                    a >>= combo;
                }
                BXL => {
                    b ^= literal;
                }
                BST => {
                    b = combo % 8;
                }
                JNZ => {
                    if a != 0 {
                        ip = literal as usize;
                        continue;
                    }
                }
                BXC => {
                    b ^= c;
                }
                OUT => {
                    inputs
                        .entry((combo % 8) as u8)
                        .or_default()
                        .push(initial_a as u16);
                    continue 'next;
                }
                BDV => b = a >> combo,
                CDV => c = a >> combo,
                _ => unreachable!(),
            }
            ip += 2;
        }
    }

    // Now we know all the possible ways to produce a single 3-bit output, so do a depth-first
    // search to find an input that produces the complete program.
    // For the first output, we have no constraint on the "upper bits" of the input, so we
    // re-run the search with all possible 7-bit suffixes to make sure we don't miss any.
    let mut results = Vec::new();
    for suffix in 0..(1 << 7) {
        search(&program, suffix, &inputs, &mut results);
    }

    // We need the program to terminate after the final program value has been output, which
    // happens when the remaining value is zero. Each iteration consumes 3 bits so our input
    // value must be no more than 3*[program length] bits long.
    results.retain(|result| result >> (program.len() * 3) == 0);

    // We are most interested in the smallest possible input.
    results.sort();

    for result in results {
        println!("{}", result);
    }
}
