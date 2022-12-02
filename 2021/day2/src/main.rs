use utils::*;

fn main() {
    // let input = "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2\n";
    let input = read_puzzle_input!().unwrap();

    let mut pos = 0;
    let mut depth = 0;
    let mut aim = 0;

    let commands: Vec<_> = input.split_whitespace().collect();

    for command in commands.chunks(2) {
        let direction = command[0];
        let magnitude = command[1].parse::<usize>().unwrap();

        match direction {
            "forward" => {
                pos += magnitude;
                depth += aim * magnitude;
            }
            "down" => aim += magnitude,
            "up" => aim -= magnitude,
            _ => unreachable!(),
        }
    }

    println!("Final position: {pos}\nFinal depth: {depth}\nFinal aim: {aim}");
    println!("Result: {}", pos * depth);
}
