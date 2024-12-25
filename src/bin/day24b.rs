use std::{cell::Cell, collections::BTreeSet};

use itertools::Itertools;

const INPUT: &str = include_str!("../../inputs/day24.txt");

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Gate {
    And,
    Or,
    Xor,
}

#[derive(Debug, Clone)]
struct Wire {
    name: Cell<&'static str>,
    gate: Gate,
    inputs: [&'static str; 2],
}

fn swap_wires(
    swapped: &mut BTreeSet<&'static str>,
    wires: &[Wire],
    a: &'static str,
    b: &'static str,
) {
    swapped.insert(a);
    swapped.insert(b);
    for wire in wires {
        if wire.name.get() == a {
            wire.name.set(b);
        } else if wire.name.get() == b {
            wire.name.set(a);
        }
    }
}

fn inputs_equal(x: [&'static str; 2], y: [&'static str; 2]) -> bool {
    x[0] == y[0] && x[1] == y[1] || x[0] == y[1] && x[1] == y[0]
}

fn identify_half_adder(
    swapped: &mut BTreeSet<&'static str>,
    wires: &[Wire],
    a: &'static str,
    b: &'static str,
    output: &'static str,
) -> &'static str {
    let mut found_output = false;
    // First, find a gate that XORs the inputs, as this will be the output.
    for wire in wires {
        if inputs_equal(wire.inputs, [a, b]) && wire.gate == Gate::Xor {
            // This must be the direct output. If it's not then swap it.
            if wire.name.get() != output {
                swap_wires(swapped, wires, wire.name.get(), output);
            }
            found_output = true;
            break;
        }
    }
    assert!(found_output);

    // Next, find a gate that ANDs the inputs, as this will be the carry.
    // We can't know whether the carry needs to be swapped or not here.
    // That will be determined by next adder.
    for wire in wires {
        if inputs_equal(wire.inputs, [a, b]) && wire.gate == Gate::And {
            return wire.name.get();
        }
    }
    panic!("Carry not found for {:?} and {:?}", a, b);
}

fn identify_full_adder(
    swapped: &mut BTreeSet<&'static str>,
    wires: &[Wire],
    a: &'static str,
    b: &'static str,
    c: &'static str,
    output: &'static str,
) -> &'static str {
    // First, find a gate that XORs the inputs, as this will be the intermediate.
    // We don't know if we need to swap it yet.
    let mut intermediate_from_inputs = None;
    for wire in wires {
        if inputs_equal(wire.inputs, [a, b]) && wire.gate == Gate::Xor {
            intermediate_from_inputs = Some(wire.name.get());
            break;
        }
    }
    let intermediate_from_inputs = intermediate_from_inputs.unwrap();

    // Next, try to verify that the carry and intermediate are correct by looking
    // for the gate that computes the result for this adder.
    let mut intermediate = intermediate_from_inputs;
    let mut carry = c;
    for wire in wires {
        // This gate must be an XOR gate.
        if wire.gate != Gate::Xor {
            continue;
        }

        // We know what the inputs and outputs *should* be, but one of them might be swapped,
        // so settle for an XOR gate where at least two of the conditions are met.
        let has_intermediate = wire.inputs.contains(&intermediate_from_inputs);
        let has_carry = wire.inputs.contains(&c);
        let has_output = wire.name.get() == output;
        let score = has_intermediate as u8 + has_carry as u8 + has_output as u8;
        if score >= 2 {
            if !has_intermediate {
                // If this gate didn't include the intermediate then that was probably swapped.
                intermediate = wire.inputs.iter().find(|&&x| x != c).unwrap();
                swap_wires(swapped, wires, intermediate_from_inputs, intermediate);
            } else if !has_carry {
                // If this gate didn't include the carry then that was probably swapped.
                carry = wire
                    .inputs
                    .iter()
                    .find(|&&x| x != intermediate_from_inputs)
                    .unwrap();
                swap_wires(swapped, wires, c, carry);
            } else if !has_output {
                // If this gate didn't produce the expected output then that was probably swapped.
                swap_wires(swapped, wires, output, wire.name.get());
            }
            break;
        }
    }

    // The carry is computed by ORing two values.
    // The first value is the AND of the inputs.
    let mut carry1_from_inputs = None;
    for wire in wires {
        if inputs_equal(wire.inputs, [a, b]) && wire.gate == Gate::And {
            carry1_from_inputs = Some(wire.name.get());
            break;
        }
    }
    let carry1_from_inputs = carry1_from_inputs.unwrap();
    // The second value is the AND of the intermediate and the incoming carry.
    let mut carry2_from_inputs = None;
    for wire in wires {
        if inputs_equal(wire.inputs, [intermediate, carry]) && wire.gate == Gate::And {
            carry2_from_inputs = Some(wire.name.get());
            break;
        }
    }
    let carry2_from_inputs = carry2_from_inputs.unwrap();

    // Find any OR gate which includes the carry1 or carry2.
    for wire in wires {
        if wire.gate != Gate::Or {
            continue;
        }
        let has_carry1 = wire.inputs.contains(&carry1_from_inputs);
        let has_carry2 = wire.inputs.contains(&carry2_from_inputs);
        if has_carry1 || has_carry2 {
            if !has_carry1 {
                // If carry1 was missing then it was probably swapped.
                let carry1 = wire
                    .inputs
                    .iter()
                    .find(|&&x| x != carry2_from_inputs)
                    .unwrap();
                swap_wires(swapped, wires, carry1_from_inputs, carry1);
            } else if !has_carry2 {
                // If carry2 was missing then it was probably swapped.
                let carry2 = wire
                    .inputs
                    .iter()
                    .find(|&&x| x != carry1_from_inputs)
                    .unwrap();
                swap_wires(swapped, wires, carry2_from_inputs, carry2);
            }
            // The output of this OR gate is the carry for the next adder.
            // Again, we don't know if it needs to be swapped yet.
            return wire.name.get();
        }
    }
    panic!(
        "Carry not found for {:?} and {:?}",
        carry1_from_inputs, carry2_from_inputs
    );
}

fn main() {
    let (input_str, gates_str) = INPUT.split_once("\n\n").unwrap();

    let mut a_inputs = Vec::new();
    let mut b_inputs = Vec::new();
    for line in input_str.lines() {
        let (lhs, _rhs) = line.split_once(": ").unwrap();
        if lhs.starts_with("x") {
            a_inputs.push(lhs);
        } else if lhs.starts_with("y") {
            b_inputs.push(lhs);
        }
    }

    let mut wires = Vec::new();
    for gate_str in gates_str.lines() {
        let parts = gate_str.split(' ').collect::<Vec<_>>();
        let gate = match parts[1] {
            "AND" => Gate::And,
            "OR" => Gate::Or,
            "XOR" => Gate::Xor,
            _ => unreachable!(),
        };
        wires.push(Wire {
            name: Cell::new(parts[4]),
            gate,
            inputs: [parts[0], parts[2]],
        });
    }

    let mut swapped = BTreeSet::new();
    let mut carry = identify_half_adder(&mut swapped, &wires, a_inputs[0], b_inputs[0], "z00");
    for i in 1..a_inputs.len() {
        carry = identify_full_adder(
            &mut swapped,
            &wires,
            a_inputs[i],
            b_inputs[i],
            carry,
            format!("z{:02}", i).leak(),
        );
    }
    println!("{}", swapped.into_iter().join(","));
}
