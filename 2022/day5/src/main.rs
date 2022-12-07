use utils::read_puzzle_input;

type Stacks = Vec<Vec<char>>;

enum CrateMover {
    CM9000,
    CM9001,
}

impl CrateMover {
    fn crate_mover(&self, moves: &str, mut stacks: Stacks) -> Stacks {
        let mut moving = Vec::new();

        for movement in moves.lines() {
            let mut m = movement.split_whitespace().skip(1).step_by(2);

            let count = m.next().unwrap().parse::<usize>().unwrap();
            let from = m.next().unwrap().parse::<usize>().unwrap();
            let to = m.next().unwrap().parse::<usize>().unwrap();

            moving.clear();
            for _ in 0..count {
                let krate = stacks[from - 1].pop().unwrap();
                match self {
                    CrateMover::CM9000 => stacks[to - 1].push(krate),
                    CrateMover::CM9001 => moving.push(krate),
                }
            }
            if !moving.is_empty() {
                stacks[to - 1].extend(moving.iter().rev());
            }
        }

        stacks
    }
}

fn main() {
    // let input = "    [D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3 \n\nmove 1 from 2 to 1\nmove 3 from 1 to 3\nmove 2 from 2 to 1\nmove 1 from 1 to 2";
    let input = read_puzzle_input!().unwrap();

    let mut split = input.split("\n\n");
    let mut initial_stacks = split.next().unwrap().lines().rev();
    let moves = split.next().unwrap();

    let num_stacks = initial_stacks
        .next()
        .unwrap()
        .trim()
        .chars()
        .last()
        .unwrap()
        .to_digit(10)
        .unwrap() as usize;
    let mut stacks = vec![Vec::new(); num_stacks];

    for line in initial_stacks {
        let chs: Vec<_> = line.chars().collect();
        for (stack_idx, krate) in chs.chunks(4).enumerate() {
            if krate[1] != ' ' {
                stacks[stack_idx].push(krate[1]);
            }
        }
    }

    let cm9000 = CrateMover::CM9000.crate_mover(moves, stacks.clone());
    let cm9001 = CrateMover::CM9001.crate_mover(moves, stacks);

    println!("CrateMover 9000 top crates:");
    for stack in cm9000.iter() {
        print!("{}", stack.last().unwrap());
    }
    println!();

    println!("CrateMover 9001 top crates:");
    for stack in cm9001.iter() {
        print!("{}", stack.last().unwrap());
    }
    println!();
}
