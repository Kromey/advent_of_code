use utils::*;

fn main() {
    // let input = "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010";
    let input = read_puzzle_input!().unwrap();

    let reports: Vec<_> = input.lines().collect();
    let bits = reports[0].len();
    let mut counts = vec![0; bits];

    for &entry in reports.iter() {
        for (pos, bit) in entry.chars().enumerate() {
            if bit == '1' {
                counts[pos] += 1;
            }
        }
    }

    let mut gamma = 0;
    for (pos, &count) in counts.iter().rev().enumerate() {
        if count > reports.len() / 2 {
            gamma += 1 << pos;
        }
    }

    println!("Gamma: {gamma:012b} {gamma}");

    let epsilon = !gamma & ((1 << bits) - 1);
    println!("Epsilon: {epsilon:012b} {epsilon}");

    println!("Power: {}", epsilon * gamma);

    let oxygen = {
        let mut reports: Vec<_> = reports.iter()
            .map(|&report| usize::from_str_radix(report, 2).unwrap())
            .collect();

        for pos in 0..bits {
            let mut count = 0;
            let mask = 1 << (bits - pos - 1);
            for &entry in reports.iter() {
                if entry & mask != 0 {
                    count += 1;
                }
            }

            let filter = if count >= reports.len() - count {
                mask
            } else {
                0
            };

            reports.retain(|report| (report & mask) == filter);
            
            if reports.len() == 1 {
                break;
            }
        }

        reports[0]
    };

    let scrubber = {
        let mut reports: Vec<_> = reports.iter()
            .map(|&report| usize::from_str_radix(report, 2).unwrap())
            .collect();

        for pos in 0..bits {
            let mut count = 0;
            let mask = 1 << (bits - pos - 1);
            for &entry in reports.iter() {
                if entry & mask != 0 {
                    count += 1;
                }
            }

            let filter = if count >= reports.len() - count {
                0
            } else {
                mask
            };

            reports.retain(|report| (report & mask) == filter);
            
            if reports.len() == 1 {
                break;
            }
        }

        reports[0]
    };

    println!("\nOxygen: {oxygen:012b} {oxygen}");
    println!("Scrubber: {scrubber:012b} {scrubber}");
    println!("Life Support: {}", oxygen * scrubber);
}
