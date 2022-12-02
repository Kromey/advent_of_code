use utils::*;

fn main() {
    // let input = "1000
    // 2000
    // 3000
    
    // 4000
    
    // 5000
    // 6000
    
    // 7000
    // 8000
    // 9000
    
    // 10000";
    let input = read_puzzle_input!().unwrap();

    let mut calories = vec![0_usize];
    let mut elf = 0;

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            elf += 1;
            calories.push(0);
        } else {
            let snack = line.parse::<usize>().unwrap();
            calories[elf] += snack;
        }
    }

    calories.sort();
    println!("Calories of top 3 elves: {:?}", calories.iter().rev().take(3).sum::<usize>());
}
