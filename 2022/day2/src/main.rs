use std::cmp::Ordering;
use utils::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Throw {
    Rock,
    Paper,
    Scissors,
}

impl Throw {
    fn score(&self) -> usize {
        use Throw::*;

        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }

    fn beats(&self) -> Self {
        use Throw::*;

        match self {
            Rock => Scissors,
            Scissors => Paper,
            Paper => Rock,
        }
    }

    fn loses_to(&self) -> Self {
        use Throw::*;

        match self {
            Rock => Paper,
            Paper => Scissors,
            Scissors => Rock,
        }
    }
}

impl Ord for Throw {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use Throw::*;
        match (self, other) {
            (Rock, Paper) => Ordering::Less,
            (Rock, Scissors) => Ordering::Greater,
            (Paper, Rock) => Ordering::Greater,
            (Paper, Scissors) => Ordering::Less,
            (Scissors, Rock) => Ordering::Less,
            (Scissors, Paper) => Ordering::Greater,
            (Rock, Rock) | (Paper, Paper) | (Scissors, Scissors) => Ordering::Equal,
        }
    }
}

impl PartialOrd for Throw {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl From<&str> for Throw {
    fn from(ch: &str) -> Self {
        use Throw::*;

        match ch.trim().chars().next().unwrap() {
            'A' => Rock,
            'B' => Paper,
            'C' => Scissors,
            ch => panic!("Unknown character: '{ch}'"),
        }
    }
}

fn main() {
    // let input = "A Y\nB X\nC Z";
    let input = read_puzzle_input!().unwrap();

    let mut score = 0;
    for game in input.lines() {
        let game: Vec<_> = game.split_whitespace().collect();

        let elf = Throw::from(game[0]);
        let me = match game[1] {
            "X" => elf.beats(),
            "Y" => elf,
            "Z" => elf.loses_to(),
            ch => panic!("Unknown throw: '{ch}'"),
        };

        score += me.score();
        score += match me.cmp(&elf) {
            Ordering::Less => 0,
            Ordering::Equal => 3,
            Ordering::Greater => 6,
        };
    }

    println!("Game over! Your score: {score}");
}
