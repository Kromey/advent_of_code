use std::{
    cmp::{max, min},
    fmt::Display,
};
use utils::*;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum Tile {
    #[default]
    Air,
    Rock,
    Sand,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Air => write!(f, "."),
            Tile::Rock => write!(f, "#"),
            Tile::Sand => write!(f, "o"),
        }
    }
}

fn main() {
    let input = read_puzzle_input!().unwrap();

    let mut cave: Vec<Vec<Tile>> = Vec::new();

    for line in input.lines() {
        let mut cursor: Option<(usize, usize)> = None;

        for coord in line.split_whitespace().step_by(2) {
            if let [x, y] = coord
                .split(',')
                .map(|v| v.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()[..]
            {
                // Ensure we have enough cave
                if cave.len() <= y {
                    cave.resize(y + 1, Vec::new());
                }
                for row in cave.iter_mut() {
                    if row.len() <= x {
                        row.resize(x + 2, Tile::Air);
                    }
                }

                if let Some(pos) = cursor {
                    let x1 = min(x, pos.0);
                    let x2 = max(x, pos.0);
                    let y1 = min(y, pos.1);
                    let y2 = max(y, pos.1);

                    for i in x1..=x2 {
                        cave.iter_mut()
                            .take(y2 + 1)
                            .skip(y1)
                            .for_each(|t| t[i] = Tile::Rock);
                    }
                }
                cursor = Some((x, y));
            }
        }
    }

    // Part 1: Drop sand until we reach the "abyss"
    let mut abyss = false;
    while !abyss {
        let mut sand = (500, 0);

        loop {
            if let Some(row) = cave.get(sand.1 + 1) {
                let left = row.get(sand.0 - 1).copied().unwrap_or_default();
                let center = row.get(sand.0).copied().unwrap_or_default();
                let right = row.get(sand.0 + 1).copied().unwrap_or_default();

                if center == Tile::Air {
                    sand.1 += 1;
                } else if left == Tile::Air {
                    sand.1 += 1;
                    sand.0 -= 1;
                } else if right == Tile::Air {
                    sand.1 += 1;
                    sand.0 += 1;
                } else {
                    cave[sand.1][sand.0] = Tile::Sand;
                    break;
                }
            } else {
                abyss = true;
                break;
            }
        }
    }

    let count: usize = cave
        .iter()
        .map(|row| row.iter().filter(|&&tile| tile == Tile::Sand).count())
        .sum();
    println!("{count} units of sand have come to rest");

    // Part 2: No infinite abyss, but infinite floor at max_y+2
    cave.push(vec![Tile::Air; 800]);
    cave.push(vec![Tile::Rock; 8000]);
    let mut x1 = 500;
    let mut x2 = 500;
    while cave[0][500] != Tile::Sand {
        let mut sand = (500, 0);

        while let Some(row) = cave.get(sand.1 + 1) {
            let left = row.get(sand.0 - 1).copied().unwrap_or_default();
            let center = row.get(sand.0).copied().unwrap_or_default();
            let right = row.get(sand.0 + 1).copied().unwrap_or_default();

            x1 = min(sand.0 - 1, x1);
            x2 = max(sand.0 + 1, x2);

            if center == Tile::Air {
                sand.1 += 1;
            } else if left == Tile::Air {
                sand.1 += 1;
                sand.0 -= 1;
            } else if right == Tile::Air {
                sand.1 += 1;
                sand.0 += 1;
            } else {
                if cave[sand.1].len() <= sand.0 {
                    cave[sand.1].resize(sand.0 + 5, Tile::Air);
                }
                cave[sand.1][sand.0] = Tile::Sand;
                break;
            }
        }
    }

    let count: usize = cave
        .iter()
        .map(|row| row.iter().filter(|&&tile| tile == Tile::Sand).count())
        .sum();
    println!("{count} units of sand have come to rest");
}
