
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").unwrap();
    // let input = "3,4,3,1,2";

    let mut fish = [0_usize; 9];
    
    for age in input.split(',') {
        let age = age.trim().parse::<usize>().unwrap();
        fish[age] += 1;
    }
    
    println!("There are {} fish initially", fish.iter().sum::<usize>());

    for day in 1..=256 {
        // println!("Day {day}:\n\t{fish:?}");
        fish.rotate_left(1);
        // println!("\t{fish:?}");

        fish[6] += fish[8];
        // println!("\t{fish:?}");

        if day % 10 == 0 {
            println!("After {day} days: {} fish", fish.iter().sum::<usize>());
        }
    }

    println!("\nThere are now a total of {} fish", fish.iter().sum::<usize>());
}
