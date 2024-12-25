use scan_fmt::scan_fmt;

const INPUT: &str = include_str!("../../inputs/day17.txt");

const ADV: u128 = 0;
const BXL: u128 = 1;
const BST: u128 = 2;
const JNZ: u128 = 3;
const BXC: u128 = 4;
const OUT: u128 = 5;
const BDV: u128 = 6;
const CDV: u128 = 7;

fn main() {
    let (mut a, mut b, mut c, program) = scan_fmt!(
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
        .map(|p| p.parse::<u128>().unwrap())
        .collect();
    let mut ip = 0;
    let mut outputs = Vec::new();
    while ip < program.len() - 1 {
        let literal = program[ip + 1];
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
                outputs.push((combo % 8).to_string());
            }
            BDV => b = a >> combo,
            CDV => c = a >> combo,
            _ => unreachable!(),
        }
        ip += 2;
    }
    println!("{}", outputs.join(","));
}
