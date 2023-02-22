use std::collections::HashSet;
use utils::*;

struct Rope<const N: usize> {
    knots: [(i32, i32); N],
    tail_visited: HashSet<(i32, i32)>,
}

impl<const N: usize> Rope<N> {
    const START: (i32, i32) = (0, 0);

    fn new() -> Rope<N> {
        Rope::<N> {
            knots: [Self::START; N],
            tail_visited: HashSet::from([Self::START]),
        }
    }

    fn walk<S: AsRef<str>>(&mut self, step: S) {
        let mut step = step.as_ref().split_whitespace();

        let dir = step.next().unwrap();
        let count = step.next().unwrap().parse::<u32>().unwrap();

        for _ in 0..count {
            let head = &mut self.knots[0];

            match dir {
                "U" => head.1 += 1,
                "D" => head.1 -= 1,
                "L" => head.0 -= 1,
                "R" => head.0 += 1,
                s => panic!("{s} is not a valid step direction!"),
            }

            self.update_knots();
        }
    }

    fn update_knots(&mut self) {
        for k in 1..N {
            let head = self.knots[k - 1];
            let knot = &mut self.knots[k];

            let dx = head.0 - knot.0;
            let dy = head.1 - knot.1;

            if dx.abs() > 1 || dy.abs() > 1 {
                knot.0 += dx.clamp(-1, 1);
                knot.1 += dy.clamp(-1, 1);
            }
        }

        self.tail_visited.insert(self.knots[N - 1]);
    }

    fn tail_visited(&self) -> usize {
        self.tail_visited.len()
    }
}

fn main() {
    // let input = "R 4
    // U 4
    // L 3
    // D 1
    // R 4
    // D 1
    // L 5
    // R 2";
    let input = read_puzzle_input!().unwrap();

    // Part 1: 2-knot rope
    let mut rope1 = Rope::<2>::new();
    // Part 2: 10-knot rope
    let mut rope2 = Rope::<10>::new();

    for line in input.lines() {
        rope1.walk(line);
        rope2.walk(line);
    }

    println!("2-knot tail visited {} locations", rope1.tail_visited());
    println!("10-knot tail visited {} locations", rope2.tail_visited());
}
