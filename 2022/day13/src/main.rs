use std::cmp::{Ord, Ordering};

use serde_json::Value;
use utils::*;

fn compare_data(left: &Value, right: &Value) -> Ordering {
    match (left, right) {
        (Value::Number(a), Value::Number(b)) => a.as_i64().unwrap().cmp(&b.as_i64().unwrap()),
        (Value::Array(arr_a), Value::Array(arr_b)) => {
            for (a, b) in arr_a.iter().zip(arr_b.iter()) {
                match compare_data(a, b) {
                    Ordering::Equal => continue,
                    res => return res,
                }
            }

            arr_a.len().cmp(&arr_b.len())
        }
        (Value::Array(_), Value::Number(_)) => {
            compare_data(left, &Value::from(vec![right.as_i64().unwrap()]))
        }
        (Value::Number(_), Value::Array(_)) => {
            compare_data(&Value::from(vec![left.as_i64().unwrap()]), right)
        }
        (_, _) => unreachable!("{left:?} vs {right:?}"),
    }
}

fn main() {
    let input = read_puzzle_input!().unwrap();

    let mut lines = input.lines();
    let mut pair_idx = 0;
    let mut sum = 0;

    loop {
        let left: Value = serde_json::from_str(lines.next().unwrap()).unwrap();
        let right: Value = serde_json::from_str(lines.next().unwrap()).unwrap();

        pair_idx += 1;

        if compare_data(&left, &right) == Ordering::Less {
            sum += pair_idx
        }

        if lines.next().is_none() {
            break;
        }
    }

    // Part 1: Just the sum of the indices of correctly-ordered pairs
    println!("Sum of correctly ordered pair indices: {sum}");

    // Part 2: Sort the packets and determine the decoder key
    let mut packets: Vec<_> = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| serde_json::from_str::<Value>(line).unwrap())
        .collect();

    // Add the divider packets
    packets.push(serde_json::from_str("[[2]]").unwrap());
    packets.push(serde_json::from_str("[[6]]").unwrap());

    packets.sort_by(compare_data);

    let mut key = 1;
    let divider_a: Value = serde_json::from_str("[[2]]").unwrap();
    let divider_b: Value = serde_json::from_str("[[6]]").unwrap();
    for (idx, packet) in packets.into_iter().enumerate() {
        // println!("{idx:2}: {packet:?}");
        if packet == divider_a || packet == divider_b {
            key *= idx + 1;
        }
    }
    println!("Decoder key for signal is {key}");
}
