use std::cmp::max;

use utils::*;

fn main() {
    // let input = "30373
    // 25512
    // 65332
    // 33549
    // 35390";
    let input = read_puzzle_input!().unwrap();

    let trees: Vec<Vec<u8>> = input
        .trim()
        .lines()
        .map(|line| line.trim().chars().map(|c| c as u8 - b'0').collect())
        .collect();

    let width = trees[0].len();
    let height = trees.len();

    // Part 1: How many are visible?
    let mut visible = 0;

    for x in 0..width {
        for y in 0..height {
            if x == 0 || x == width - 1 || y == 0 || y == height - 1 {
                visible += 1;
                continue;
            }

            let tree = trees[y][x];

            // Check the row
            let seen = trees[y][0..x].iter().all(|t| *t < tree)
                || trees[y][(x + 1)..width].iter().all(|t| *t < tree);
            if seen {
                visible += 1;
                continue;
            }

            // Check the column
            let column: Vec<_> = trees.iter().map(|row| row[x]).collect();
            let seen = column[0..y].iter().all(|t| *t < tree)
                || column[(y + 1)..height].iter().all(|t| *t < tree);
            if seen {
                visible += 1;
            }
        }
    }
    println!("There are {visible} trees visible");

    // Part 2: What's the best scenic score?
    println!();

    let mut best = 0;

    // Trees on the edges have a scenic score of 0, since they have at least 1 viewing distance of 0
    for x in 1..(width - 1) {
        for y in 1..(height - 1) {
            let tree = trees[y][x];

            // Left
            let mut left = 0;
            trees[y].iter().take(x).rev().any(|t| {
                left += 1;
                *t >= tree
            });

            // Right
            let mut right = 0;
            trees[y].iter().skip(x + 1).any(|t| {
                right += 1;
                *t >= tree
            });

            // Check the column
            let column: Vec<_> = trees.iter().map(|row| row[x]).collect();

            // Up
            let mut up = 0;
            column.iter().take(y).rev().any(|t| {
                up += 1;
                *t >= tree
            });

            // Down
            let mut down = 0;
            column.iter().skip(y + 1).any(|t| {
                down += 1;
                *t >= tree
            });

            best = max(best, left * right * up * down);
        }
    }
    println!("The best scenic score is {best}");
}
