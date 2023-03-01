use pathfinding::prelude::astar;
use std::collections::HashMap;
use utils::*;

type Valves = HashMap<String, Valve>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Valve {
    rate: u32,
    tunnels: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State<const N: usize> {
    positions: [String; N],
    flow_rate: u32,
    minutes_remaining: u8,
    closed_valves: Vec<String>,
}

impl<const N: usize> State<N> {
    fn successors(&self, valves: &Valves, max_rate: u32) -> Vec<(State<N>, u32)> {
        if self.minutes_remaining == 0 || self.closed_valves.is_empty() {
            return Default::default();
        }

        let minutes_remaining = self.minutes_remaining - 1;

        self.neighbors(valves)
            .into_iter()
            .filter_map(|positions| {
                // for i in 1..positions.len() {
                //     if positions[i..].contains(&positions[i-1]) {
                //         return None;
                //     }
                // }

                let mut opening = Vec::new();
                for (i, pos) in positions.iter().enumerate() {
                    if pos == &self.positions[i] && self.closed_valves.contains(pos) {
                        opening.push(pos);
                    }
                }
                if !opening.is_empty() {
                    for i in 1..opening.len() {
                        if opening[i..].contains(&opening[i - 1]) {
                            return None;
                        }
                    }
                }

                let flow_rate = self.flow_rate
                    + opening
                        .iter()
                        .map(|&v| valves.get(v).map(|v| v.rate).unwrap_or_default())
                        .sum::<u32>();

                let closed_valves = self
                    .closed_valves
                    .iter()
                    .cloned()
                    .filter(|v| !opening.contains(&v))
                    .collect();

                Some((
                    State {
                        positions,
                        flow_rate,
                        minutes_remaining,
                        closed_valves,
                    },
                    max_rate - flow_rate,
                ))
            })
            .collect()
    }

    fn neighbors(&self, valves: &Valves) -> Vec<[String; N]> {
        let mut all_neighbors = vec![self.positions.clone()];

        for (idx, pos) in self.positions.iter().enumerate() {
            let my_neighbors = valves
                .get(pos)
                .map(|v| v.tunnels.clone())
                .unwrap_or_default();
            // if self.closed_valves.contains(pos) {
            //     my_neighbors.push(pos.clone());
            // }

            let ext: Vec<_> = my_neighbors
                .into_iter()
                .flat_map(|pos| {
                    all_neighbors.iter().cloned().map(move |mut positions| {
                        positions[idx] = pos.clone();
                        positions
                    })
                })
                .collect();
            all_neighbors.extend(ext);
        }

        // all_neighbors.swap_remove(0);

        all_neighbors
    }
}

fn main() {
    // let input = read_puzzle_input!().unwrap();
    // let input = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
    // Valve BB has flow rate=13; tunnels lead to valves CC, AA
    // Valve CC has flow rate=2; tunnels lead to valves DD, BB
    // Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
    // Valve EE has flow rate=3; tunnels lead to valves FF, DD
    // Valve FF has flow rate=0; tunnels lead to valves EE, GG
    // Valve GG has flow rate=0; tunnels lead to valves FF, HH
    // Valve HH has flow rate=22; tunnel leads to valve GG
    // Valve II has flow rate=0; tunnels lead to valves AA, JJ
    // Valve JJ has flow rate=21; tunnel leads to valve II";
    let input = "Valve LA has flow rate=22; tunnels lead to valves KA, MA
    Valve MA has flow rate=24; tunnels lead to valves LA, NA
    Valve NA has flow rate=26; tunnels lead to valves MA, OA
    Valve OA has flow rate=28; tunnels lead to valves NA, PA
    Valve PA has flow rate=30; tunnels lead to valves OA
    Valve AA has flow rate=0; tunnels lead to valves BA
    Valve BA has flow rate=2; tunnels lead to valves AA, CA
    Valve CA has flow rate=4; tunnels lead to valves BA, DA
    Valve DA has flow rate=6; tunnels lead to valves CA, EA
    Valve EA has flow rate=8; tunnels lead to valves DA, FA
    Valve FA has flow rate=10; tunnels lead to valves EA, GA
    Valve GA has flow rate=12; tunnels lead to valves FA, HA
    Valve HA has flow rate=14; tunnels lead to valves GA, IA
    Valve IA has flow rate=16; tunnels lead to valves HA, JA
    Valve JA has flow rate=18; tunnels lead to valves IA, KA
    Valve KA has flow rate=20; tunnels lead to valves JA, LA";

    let mut valves: Valves = HashMap::new();
    let mut max_rate = 0;

    for line in input.lines() {
        let parts: Vec<_> = line.trim().split(';').collect();
        let id = parts[0][6..8].to_string();
        let rate = parts[0][23..].parse::<u32>().unwrap();

        let tunnels: Vec<_> = parts[1]
            .split_whitespace()
            .skip(4)
            .map(|tunnel| tunnel.split(',').next().unwrap().to_string())
            .collect();

        valves.insert(id, Valve { rate, tunnels });
        max_rate += rate;
    }
    let closed_valves: Vec<_> = valves
        .iter()
        .filter_map(|(id, valve)| {
            if valve.rate > 0 {
                Some(id.clone())
            } else {
                None
            }
        })
        .collect();

    println!("Valves: {}", valves.len());
    println!("Max rate: {max_rate}");

    // Part 1: Maximum pressure released
    let start = State {
        positions: ["AA".to_string()],
        flow_rate: 0,
        minutes_remaining: 30,
        closed_valves: closed_valves.clone(),
    };
    let mut path = astar(
        &start,
        |state| state.successors(&valves, max_rate),
        |state| max_rate - state.flow_rate,
        |state| state.minutes_remaining == 1 || state.flow_rate == max_rate,
    )
    .unwrap()
    .0;
    let mut last_state = path.last().unwrap().clone();
    while last_state.minutes_remaining > 1 {
        last_state.minutes_remaining -= 1;
        path.push(last_state.clone());
    }
    for s in path.iter().skip(1) {
        println!(
            "Minutes: {}; At valve {}; flow rate {}",
            s.minutes_remaining, s.positions[0], s.flow_rate
        );
    }
    let total_pressure = path
        .iter()
        .fold(0, |pressure, state| pressure + state.flow_rate);
    println!("Released {total_pressure} pressure");

    // Part 2: Trained an elephant to open valves in just 4 minutes!
    // FIXME: This is currently not working
    // I believe A* is a valid solution, but I think I need a better graph structure so that I can determine
    // distance/range to a closed, working valve
    let start = State {
        positions: ["AA".to_owned(), "AA".to_owned()],
        flow_rate: 0,
        minutes_remaining: 26,
        closed_valves,
    };
    println!("Initial neighbors: {:?}", start.neighbors(&valves));
    println!(
        "Initial successors: {:?}",
        start.successors(&valves, max_rate)
    );
    let mut path = astar(
        &start,
        |state| state.successors(&valves, max_rate),
        |state| max_rate - state.flow_rate,
        |state| state.minutes_remaining == 1 || state.flow_rate == max_rate,
    )
    .unwrap()
    .0;
    let mut last_state = path.last().unwrap().clone();
    while last_state.minutes_remaining > 1 {
        last_state.minutes_remaining -= 1;
        path.push(last_state.clone());
    }
    for s in path.iter().skip(1) {
        println!(
            "Minutes: {}; At valves {:?}; flow rate {}",
            s.minutes_remaining, s.positions, s.flow_rate
        );
    }
    let total_pressure = path
        .iter()
        .fold(0, |pressure, state| pressure + state.flow_rate);
    println!("Released {total_pressure} pressure");
}
