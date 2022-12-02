use utils::*;

fn main() {
    // let input = "199\n200\n208\n210\n200\n207\n240\n269\n260\n263";
    let input = read_puzzle_input!().unwrap();

    let depths: Vec<_> = input.lines()
        .map(|line| line.trim().parse::<usize>().unwrap())
        .collect();

    let mut increases = 0;
    let mut previous_depth = depths.iter().take(3).sum::<usize>();

    for window in depths.windows(3) {
        let depth = window.iter().sum::<usize>();
        if depth > previous_depth {
            increases += 1;
        }
        previous_depth = depth;
    }

    println!("Depth increases {increases} times");
}
