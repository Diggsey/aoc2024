use scan_fmt::scan_fmt;

const INPUT: &str = include_str!("../../inputs/day13.txt");

fn main() {
    let total: i64 = INPUT
        .split("\n\n")
        .filter_map(|machine| {
            let (ax, ay, bx, by, mut cx, mut cy) = scan_fmt!(
                machine,
                "Button A: X+{}, Y+{}\nButton B: X+{}, Y+{}\nPrize: X={}, Y={}",
                i64,
                i64,
                i64,
                i64,
                i64,
                i64
            )
            .unwrap();
            cx += 10000000000000;
            cy += 10000000000000;
            let det = bx * ay - by * ax;
            if det == 0 {
                // parallel lines
                panic!("Not handled");
            } else {
                let a_num = cx * by - cy * bx;
                let b_num = cx * ay - cy * ax;
                if a_num % det != 0 || b_num % det != 0 {
                    None
                } else {
                    let a = a_num / -det;
                    let b = b_num / det;
                    if a < 0 || b < 0 {
                        None
                    } else {
                        Some(a * 3 + b)
                    }
                }
            }
        })
        .sum();
    println!("{}", total);
}
