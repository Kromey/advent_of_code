use utils::*;
use pathfinding::prelude::astar;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos(usize, usize);

impl Pos {
    /// Get the distance between two points
    fn distance(&self, other: Pos) -> u32 {
        (self.0.abs_diff(other.0) + self.1.abs_diff(other.1)) as u32
    }

    /// Get the distance to the nearest "bottom" elevation
    fn distance_to_bottom(&self, elevations: &[Vec<u8>]) -> u32 {
        elevations.iter().enumerate().map(|(y, row)| {
            row.iter().enumerate().filter(|&(_, e)| *e == 0).map(move |(x, _)| Pos(x, y).distance(*self)).min().unwrap_or(u32::MAX)
        }).min().unwrap()
    }

    /// Find all possible successors where the target is at most 1 step up
    fn successors(&self, elevations: &[Vec<u8>], width: usize, height: usize) -> Vec<(Pos, u32)> {
        let neighbors = self.neighbors(width, height);

        let elevation = elevations[self.1][self.0] as i16;

        neighbors.into_iter().filter(|p| {
            let e = elevations[p.1][p.0] as i16;
            e - elevation <= 1
        }).map(|p| (p, 1)).collect()
    }

    /// Find all possible successors, but only if the target is 1 step down higher
    fn rev_successors(&self, elevations: &[Vec<u8>], width: usize, height: usize) -> Vec<(Pos, u32)> {
        let neighbors = self.neighbors(width, height);

        let elevation = elevations[self.1][self.0] as i16;

        neighbors.into_iter().filter(|p| {
            let e = elevations[p.1][p.0] as i16;
            elevation - e <= 1
        }).map(|p| (p, 1)).collect()
    }

    /// Get up to 4 neighboring points, respecting the map's boundaries
    fn neighbors(&self, width: usize, height: usize) -> Vec<Pos> {
        let mut neighbors = Vec::new();

        if self.0 > 0 {
            neighbors.push(Pos(self.0 - 1, self.1));
        }
        if self.0 < width - 1 {
            neighbors.push(Pos(self.0 + 1, self.1));
        }
        if self.1 > 0 {
            neighbors.push(Pos(self.0, self.1 - 1));
        }
        if self.1 < height - 1 {
            neighbors.push(Pos(self.0, self.1 + 1));
        }

        neighbors
    }
}

fn main() {
    let input = read_puzzle_input!().unwrap();

    let mut start = Pos(0, 0);
    let mut goal = Pos(0, 0);

    let elevations = input.lines().enumerate().map(|(y, line)| {
        line.chars().enumerate().map(|(x, c)| {
            if c == 'S' {
                start = Pos(x, y);
                'a'
            } else if c == 'E' {
                goal = Pos(x, y);
                'z'
            } else {
                c
            }
        }).map(|c| c as u8 - b'a').collect::<Vec<u8>>()
    }).collect::<Vec<_>>();

    let height = elevations.len();
    let width = elevations[0].len();

    // Part 1: Length of the shortest path
    let (path, _) = astar(
        &start,
        |&p| p.successors(&elevations, width, height),
        |&p| p.distance(goal),
        |&p| p == goal,
    ).unwrap();

    // Subtract 1 from the length because the path includes our start
    println!("Reached goal in {} steps", path.len() - 1);

    // Part 2: Shortest path from *any* elevation 'a' to the goal
    // We do this by pathing *from* the goal *to* the first 'a' we can reach
    let (path, _) = astar(
        &goal,
        |&p| p.rev_successors(&elevations, width, height),
        |&p| p.distance_to_bottom(&elevations),
        |&Pos(x, y)| elevations[y][x] == 0,
    ).unwrap();

    // Subtract 1 from the length because the path includes our start
    println!("Reached goal in {} steps", path.len() - 1);
}
