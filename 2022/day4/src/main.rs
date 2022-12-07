use utils::read_puzzle_input;

fn main() {
    // let input = "2-4,6-8
    // 2-3,4-5
    // 5-7,7-9
    // 2-8,3-7
    // 6-6,4-6
    // 2-6,4-8";
    let input = read_puzzle_input!().unwrap();

    let elves: Vec<_> = input
        .lines()
        .map(|line| {
            line.trim()
                .split(',')
                .map(|elf| {
                    let val = elf
                        .split('-')
                        .map(|i| i.parse::<usize>().unwrap())
                        .collect::<Vec<_>>();
                    (val[0], val[1])
                })
                .collect::<Vec<_>>()
        })
        .collect();

    let mut contained = 0;
    for elves in elves.iter() {
        let elf_a = elves[0];
        let elf_b = elves[1];

        // While they overlap, Clippy is objectively wrong that these two conditions are the same
        #[allow(clippy::if_same_then_else)]
        if elf_a.0 >= elf_b.0 && elf_a.1 <= elf_b.1 {
            contained += 1;
        } else if elf_a.0 <= elf_b.0 && elf_a.1 >= elf_b.1 {
            contained += 1;
        }
    }

    println!("{contained} ranges fully enclose their partner");

    println!(); // On to part 2!

    let mut overlapped = 0;
    for elves in elves.iter() {
        let elf_a = elves[0];
        let elf_b = elves[1];

        let range_a = elf_a.0..=elf_a.1;
        let range_b = elf_b.0..=elf_b.1;

        if range_a.contains(&elf_b.0)
            || range_a.contains(&elf_b.1)
            || range_b.contains(&elf_a.0)
            || range_b.contains(&elf_a.1)
        {
            overlapped += 1;
        }
    }

    println!("{overlapped} ranges overlap their partner");
}
