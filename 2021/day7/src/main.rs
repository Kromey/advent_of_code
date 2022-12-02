use std::cmp::{min, max};

use utils::*;

fn fuel(positions: &[usize], alignment: usize) -> usize {
    positions.iter()
        .map(|&p| fuel_usage(p.abs_diff(alignment)))
        .sum()
}

fn fuel_usage(steps: usize) -> usize {
    (steps * (steps + 1)) / 2
}

fn main() {
    let input = read_puzzle_input!().unwrap();
    // let input = "16,1,2,0,4,2,7,1,2,14";
    let positions: Vec<_> = input.split(',')
        .map(|i| i.trim().parse::<usize>().unwrap())
        .collect();
    
    println!("{positions:?}");

    let (min_pos, max_pos) = positions.iter()
        .map(|&i| (i, i))
        .reduce(|(a, b), (i, j)| (min(a, i), max(b, j)))
        .unwrap();
    let mut new_pos = (max_pos + min_pos) / 2;

    let usage = (
        fuel(&positions, new_pos - 1),
        fuel(&positions, new_pos),
        fuel(&positions, new_pos + 1),
    );

    let mut used_fuel = usage.1;

    if !(usage.1 < usage.0 && usage.1 < usage.2) {
        let direction = if usage.0 < usage.2 {
            -1
        } else {
            1
        };

        while new_pos > 0 && new_pos <= max_pos {
            let trying = (new_pos as isize + direction) as usize;
            let usage = fuel(&positions, trying);

            println!("Fuel usage at {trying}: {usage}");

            if usage > used_fuel {
                println!("Found minimum usage!");
                break;
            }

            used_fuel = usage;
            new_pos = trying;
        }
    }

    println!("Alignment: {new_pos}; fuel: {used_fuel}");
}
