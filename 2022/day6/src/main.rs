use std::collections::HashSet;

use utils::read_puzzle_input;

fn find_marker(data: &str, marker_len: usize) -> Result<usize, String> {
    let received: Vec<_> = data.chars().collect();

    let mut count = marker_len; // We're starting after the first 4 have been received
    for seq in received.windows(marker_len) {
        let set: HashSet<_> = seq.iter().collect();
        if set.len() == seq.len() {
            return Ok(count);
        }
        count += 1;
    }

    Err("Marker not found in input data".to_string())
}

fn main() -> Result<(), String> {
    // let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    let input = read_puzzle_input!().unwrap();

    // Part One: 4-character start-of-packet marker
    println!("Start-of-packet marker received after {} characters", find_marker(&input, 4)?);

    // Part Two: 14-character start-of-message marker
    println!("Start-of-message marker received after {} characters", find_marker(&input, 14)?);

    Ok(())
}
