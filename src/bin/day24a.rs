use std::collections::HashMap;

const INPUT: &str = include_str!("../../inputs/day24.txt");

#[derive(Copy, Clone)]
enum Gate {
    And,
    Or,
    Xor,
}

impl Gate {
    fn eval(&self, a: bool, b: bool) -> bool {
        match self {
            Self::And => a && b,
            Self::Or => a || b,
            Self::Xor => a ^ b,
        }
    }
}

#[derive(Copy, Clone)]
enum Wire {
    Const(bool),
    Gate(Gate, [&'static str; 2]),
}

fn eval_wire(wires: &mut HashMap<&'static str, Wire>, wire_name: &str) -> Option<bool> {
    Some(match *wires.get(wire_name)? {
        Wire::Const(value) => value,
        Wire::Gate(gate, [a, b]) => {
            let a = eval_wire(wires, a)?;
            let b = eval_wire(wires, b)?;
            let value = gate.eval(a, b);
            *wires.get_mut(wire_name).unwrap() = Wire::Const(value);
            value
        }
    })
}

fn main() {
    let (input_str, gates_str) = INPUT.split_once("\n\n").unwrap();
    let mut wires = input_str
        .lines()
        .map(|line| {
            let (lhs, rhs) = line.split_once(": ").unwrap();
            (lhs, Wire::Const(rhs == "1"))
        })
        .collect::<HashMap<&'static str, Wire>>();

    for gate_str in gates_str.lines() {
        let parts = gate_str.split(' ').collect::<Vec<_>>();
        wires.entry(parts[4]).or_insert_with(|| {
            let gate = match parts[1] {
                "AND" => Gate::And,
                "OR" => Gate::Or,
                "XOR" => Gate::Xor,
                _ => unreachable!(),
            };
            Wire::Gate(gate, [parts[0], parts[2]])
        });
    }
    let mut result: u128 = 0;
    for i in 0.. {
        let wire_name = format!("z{0:02}", i);
        if let Some(value) = eval_wire(&mut wires, &wire_name) {
            result |= (value as u128) << i;
        } else {
            break;
        }
    }
    println!("{}", result);
}
