use utils::*;

type Worry = u64;

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add(Worry),
    Mul(Worry),
    Pow,
}

impl Operation {
    fn apply(&self, rhs: Worry) -> Worry {
        match self {
            Self::Add(add) => rhs + add,
            Self::Mul(mul) => rhs * mul,
            Self::Pow => rhs.pow(2),
        }
    }
}

impl<S: AsRef<str>> From<S> for Operation {
    fn from(value: S) -> Self {
        let mut op = value.as_ref()[23..].split_whitespace();

        if op.next().unwrap() == "+" {
            let add = op.next().unwrap().parse::<Worry>().unwrap();
            Self::Add(add)
        } else if let Ok(mul) = op.next().unwrap().parse::<Worry>() {
            Self::Mul(mul)
        } else {
            Self::Pow
        }
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<Worry>,
    operation: Operation,
    test: Worry,
    throw_true: usize,
    throw_false: usize,
}

impl Monkey {
    fn inspect_items(&mut self, relief: Worry) -> Vec<(usize, Worry)> {
        let mut throws = Vec::new();

        for mut item in self.items.drain(0..) {
            item = self.operation.apply(item);
            item /= relief;

            let target = if item % self.test == 0 {
                self.throw_true
            } else {
                self.throw_false
            };

            throws.push((target, item));
        }

        throws
    }
}

fn main() {
    let input = read_puzzle_input!().unwrap();

    let mut lines = input.lines().filter(|line| !line.trim().is_empty());
    let mut monkeys = Vec::new();

    loop {
        if lines.next().is_none() {
            break;
        }

        let items_line = lines.next().unwrap();
        let op_line = lines.next().unwrap();
        let test_line = lines.next().unwrap();
        let true_line = lines.next().unwrap();
        let false_line = lines.next().unwrap();

        let items: Vec<_> = items_line[18..].split(',').map(|item| item.trim().parse::<Worry>().unwrap()).collect();
        let operation = Operation::from(op_line);
        let test = test_line.split_whitespace().last().unwrap().parse::<Worry>().unwrap();
        let throw_true = true_line.split_whitespace().last().unwrap().parse::<usize>().unwrap();
        let throw_false = false_line.split_whitespace().last().unwrap().parse::<usize>().unwrap();

        monkeys.push(Monkey {
            items,
            operation,
            test,
            throw_true,
            throw_false,
        });
    }
    let mut monkeys2 = monkeys.clone();
    // Worry levels get extremely high in part 2, so we find the common multiple of all the monkeys' tests
    // Then instead of continuing to use the "raw" worry, we store it mod the LCM
    let lcm = monkeys.iter().map(|m| m.test).product::<Worry>();

    // Part 1: Find the 2 busiest monkeys
    let mut business = vec![0; monkeys.len()];
    for _ in 0..20 {
        for monkey_id in 0..monkeys.len() {
            let throws = monkeys[monkey_id].inspect_items(3);
            business[monkey_id] += throws.len();

            for (target, item) in throws {
                monkeys[target].items.push(item % lcm);
            }
        }
    }
    business.sort();
    let shenanigans = business.into_iter().rev().take(2).product::<usize>();
    println!("Monkey business level: {shenanigans}");

    // Part 2: Electric boogaloo, but 10_000 rounds and we don't divide our worry by 3
    business = vec![0; monkeys.len()];
    for _ in 0..10_000 {
        for monkey_id in 0..monkeys2.len() {
            let throws = monkeys2[monkey_id].inspect_items(1);
            business[monkey_id] += throws.len();

            for (target, item) in throws {
                monkeys2[target].items.push(item % lcm);
            }
        }
    }
    business.sort();
    let shenanigans = business.into_iter().rev().take(2).product::<usize>();
    println!("Monkey business level: {shenanigans}");
}
