use std::collections::HashSet;

use utils::read_puzzle_input;

fn find_marker(data: &str, marker_len: usize) -> Result<usize, String> {
    for (i, seq) in data
        .chars()
        .collect::<Vec<_>>()
        .windows(marker_len)
        .enumerate()
    {
        if seq.iter().collect::<HashSet<_>>().len() == marker_len {
            return Ok(i + marker_len);
        }
    }

    Err("Marker not found in input data".to_string())
}

fn main() -> Result<(), String> {
    // let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    let input = read_puzzle_input!().unwrap();

    // Part One: 4-character start-of-packet marker
    println!(
        "Start-of-packet marker received after {} characters",
        find_marker(&input, 4)?
    );

    // Part Two: 14-character start-of-message marker
    println!(
        "Start-of-message marker received after {} characters",
        find_marker(&input, 14)?
    );

    Ok(())
}
