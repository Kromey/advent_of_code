use utils::*;

fn main() {
    let input = read_puzzle_input!().unwrap();

    let mut clock = 1;
    let mut reg_x = 1;

    // Part 1: Signal strength at "interesting" clock cycles
    let cycles = [20, 60, 100, 140, 180, 220];
    let mut strengths = 0;

    for op in input
        .trim()
        .lines()
        .flat_map(|line| line.split_whitespace())
    {
        if cycles.contains(&clock) {
            let strength = clock * reg_x;
            println!("{clock}: {reg_x}; {strength}");
            strengths += strength;
        }

        if let Ok(x) = op.parse::<i32>() {
            reg_x += x;
        }
        clock += 1;
    }

    println!("\nTotal Strength: {strengths}");

    // Part 2: Rendering to a CRT
    let mut crt = [false; 40 * 6];
    clock = 0;
    reg_x = 1;

    for op in input
        .trim()
        .lines()
        .flat_map(|line| line.split_whitespace())
    {
        if reg_x.abs_diff(clock % 40) <= 1 {
            crt[clock as usize] = true;
        }

        if let Ok(x) = op.parse::<i32>() {
            reg_x += x;
        }
        clock += 1;
    }

    for (idx, px) in crt.iter().enumerate() {
        if idx % 40 == 0 {
            println!();
        }
        if *px {
            print!("#");
        } else {
            print!(" ");
        }
    }
    println!();
}
