use std::{
    cmp::{max, min},
    collections::HashSet,
};

use utils::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Pos(i32, i32);

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
struct Range(i32, i32);

impl Range {
    fn reduce(ranges: &[Range]) -> Vec<Range> {
        if ranges.len() == 1 {
            return ranges.into();
        }

        let mut range = ranges[0];
        let mut disjoint = Vec::new();
        for other in &ranges[1..] {
            if let Some(r) = range.combine(other) {
                range = r;
            } else {
                disjoint.push(*other)
            }
        }
        disjoint.push(range);

        if disjoint.len() == 1 || disjoint.len() == ranges.len() {
            disjoint
        } else {
            Range::reduce(&disjoint)
        }
    }

    fn combine(&self, other: &Range) -> Option<Range> {
        let combined = Range(min(self.0, other.0), max(self.1, other.1));

        // Ranges overlap if the combined len is less than or equal to the sum of the two lens
        // Ranges are adjacent if the combined len is equal to the sum of the two, +1 to accomodate the lack of overlap
        if combined.len() <= self.len() + other.len() + 1 {
            Some(combined)
        } else {
            None
        }
    }

    fn len(&self) -> u32 {
        self.0.abs_diff(self.1)
    }
}

impl Pos {
    fn dist(&self, other: &Pos) -> i32 {
        (self.0.abs_diff(other.0) + self.1.abs_diff(other.1)) as i32
    }

    fn row_range(&self, range: i32, y: i32) -> Option<Range> {
        let span = range - y.abs_diff(self.1) as i32;

        if span < 0 {
            None
        } else {
            Some(Range(self.0 - span, self.0 + span))
        }
    }
}

fn main() {
    let input = read_puzzle_input!().unwrap();

    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;
    let mut beacons = HashSet::new();
    let sensors: Vec<_> = input
        .lines()
        .map(|line| {
            let coordinates: Vec<_> = line
                .split_whitespace()
                .filter_map(|token| {
                    let val: String = token
                        .chars()
                        .filter(|&c| c.is_ascii_digit() || c == '-')
                        .collect();
                    val.parse::<i32>().ok()
                })
                .collect();

            let sensor = Pos(coordinates[0], coordinates[1]);
            let beacon = Pos(coordinates[2], coordinates[3]);
            let dist = sensor.dist(&beacon);

            min_x = min(min_x, sensor.0 - dist - 5);
            max_x = max(max_x, sensor.0 + dist + 5);

            beacons.insert(beacon);

            (sensor, sensor.dist(&beacon))
        })
        .collect();

    println!(
        "{} beacons detected by {} sensors",
        beacons.len(),
        sensors.len()
    );

    // Part 1: How many positions at y=2_000_000 cannot contain a beacon?
    let target_row = 2_000_000;
    let ranges: Vec<_> = sensors
        .iter()
        .filter_map(|(sensor, range)| sensor.row_range(*range, target_row))
        .collect();
    let reduced = Range::reduce(&ranges);
    let count: u32 = reduced.iter().map(|range| range.len()).sum();
    println!("{count} positions at y={target_row} cannot contain a beacon");

    // Part 2: Find the distress beacon's frequency
    let beacon_range = 4_000_000;
    'search: for y in 0..=beacon_range {
        let ranges: Vec<_> = sensors
            .iter()
            .filter_map(|&(pos, range)| pos.row_range(range, y))
            .collect();
        let mut reduced = Range::reduce(&ranges);
        reduced.sort();

        if reduced.len() > 1 || reduced[0].0 > 0 || reduced[0].1 < beacon_range {
            // Find the gap: could be at either end, or between 2 non-overlapping/non-adjacent ranges
            let x = if reduced.len() == 1 && reduced[0].0 > 0 {
                0
            } else {
                // It's 1 higher than the top of the first range (since we've sorted them)
                reduced[0].1 + 1
            };
            let pos = Pos(x, y);
            let frequency = x as u64 * 4_000_000 + y as u64;
            println!("Found distress beacon at {pos:?} with frequency {frequency}");
            break 'search;
        }
    }
}
