use std::collections::HashSet;
use utils::*;

fn item_priority(item: char) -> usize {
    const LOWER_CASE: usize = 'a' as usize - 1;
    const UPPER_CASE: usize = 'A' as usize - 27;

    // Kudos to @jonmsawyer for showing me how straightforward this actually is
    if ('a'..='z').contains(&item) {
        item as usize - LOWER_CASE
    } else if ('A'..='Z').contains(&item) {
        item as usize - UPPER_CASE
    } else {
        panic!("Invalid item: '{item}'");
    }
}

fn main() {
    // let input = "vJrwpWtwJgWrhcsFMMfFFhFp
    // jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
    // PmmdzqPrVvPwwTWBwg
    // wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
    // ttgJtRGJQctTZtZT
    // CrZsJsPPZsGzwwsLwLmpwMDw";
    let input = read_puzzle_input!().unwrap();

    println!("a: {}", item_priority('a'));
    println!("z: {}", item_priority('z'));
    println!("A: {}", item_priority('A'));
    println!("Z: {}", item_priority('Z'));

    println!("Duplicate items:");
    let mut priorities = 0;
    for rucksack in input.lines() {
        let rucksack = rucksack.trim();
        let size = rucksack.chars().count();

        let compartmenta: HashSet<char> = rucksack.chars().take(size / 2).collect();
        let compartmentb: HashSet<char> = rucksack.chars().skip(size / 2).collect();

        for item in compartmenta.intersection(&compartmentb) {
            let priority = item_priority(*item);
            print!("{item}: {priority}; ");
            priorities += priority;
        }
    }

    println!("\nSum of priorities: {priorities}");

    println!("\nElf team badges:"); // On to part two!
    priorities = 0;
    let elves: Vec<_> = input.lines().collect();
    for team in elves.chunks_exact(3) {
        let badge = team
            .iter()
            .map(|elf| elf.chars().collect::<HashSet<_>>())
            .reduce(|common, elf| common.intersection(&elf).copied().collect())
            .unwrap()
            .into_iter()
            .next()
            .unwrap();

        let priority = item_priority(badge);
        print!("{badge}: {priority}; ");
        priorities += priority;
    }
    println!("\nTotal badge priorities: {priorities}");
}
