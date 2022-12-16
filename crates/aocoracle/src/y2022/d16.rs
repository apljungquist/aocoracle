use anyhow::anyhow;
use hashbrown::{HashMap, HashSet};
use itertools::Itertools;
use pathfinding::prelude::{bfs, dijkstra};
use std::hash::{Hash, Hasher};

fn parsed(s: &str) -> anyhow::Result<(HashMap<String, usize>, HashMap<String, HashSet<String>>)> {
    let re = regex::Regex::new(
        r"^Valve ([A-Z]{2}) has flow rate=(\d+); tunnels? leads? to valves? ([A-Z, ]+)$",
    )
    .expect("Hard coded regex is valid");
    let mut pressures = HashMap::new();
    let mut tunnels: HashMap<String, HashSet<String>> = HashMap::new();
    for line in s.lines() {
        let cap = re
            .captures(line)
            .ok_or_else(|| anyhow!("Could not capture line {}", line))?;
        let src = cap[1].to_string();
        pressures.insert(src.clone(), cap[2].parse()?);
        for dst in cap[3].split(',') {
            let dst = dst.trim().to_string();
            tunnels.entry(src.clone()).or_default().insert(dst);
        }
    }
    Ok((pressures, tunnels))
}

enum Action {
    Move(String),
    Open,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct State {
    valve: String,
    valves_remaining: Vec<String>,
    duration_remaining: usize,
}

impl State {
    fn neighbors(
        &self,
        valves: &HashMap<String, usize>,
        distances: &HashMap<String, HashMap<String, usize>>,
    ) -> Vec<(State, i64)> {
        // let distances: HashMap<String, usize> = self
        //     .valves_remaining
        //     .iter()
        //     .map(|valve| (valve.clone(), distance(tunnels, &self.valve, valve)))
        //     .collect();
        let rate_remaining: usize = self.valves_remaining.iter().map(|v| valves[v]).sum();
        let mut result: Vec<(State, i64)> = self
            .valves_remaining
            .iter()
            .filter(|valve| self.duration_remaining >= distances[&self.valve][*valve])
            .map(|valve| {
                let duration_remaining =
                    self.duration_remaining - distances[&self.valve][valve] as usize;
                (
                    Self {
                        valve: valve.clone(),
                        duration_remaining,
                        valves_remaining: {
                            let mut valves = self.valves_remaining.clone();
                            valves.retain(|v| v != valve);
                            valves
                        },
                    },
                    // -((duration_remaining * valves[valve]) as i64),
                    (rate_remaining * distances[&self.valve][valve]) as i64,
                )
            })
            .collect();
        result.push((
            Self {
                valve: self.valve.clone(),
                duration_remaining: self.duration_remaining - 1,
                valves_remaining: self.valves_remaining.clone(),
            },
            rate_remaining.try_into().unwrap(),
        ));
        result
    }
}

fn distance(edges: &HashMap<String, HashSet<String>>, src: &String, dst: &String) -> usize {
    bfs(src, |t| edges[t].clone(), |t| t == dst).unwrap().len()
}

fn pressure_released(pressures: &HashMap<String, usize>, states: &[State]) -> usize {
    states
        .iter()
        .unique_by(|s| s.valve)
        .map(|s| s.duration_remaining * pressures[&s.valve])
        .sum()
}

pub fn part_1(input: &str) -> anyhow::Result<usize> {
    let (mut pressures, tunnels) = parsed(input)?;
    let valve = "AA".to_string();
    let mut nodes = HashSet::new();
    for (src, dsts) in tunnels.iter() {
        nodes.insert(src.clone());
        nodes.extend(dsts.iter().cloned());
    }
    let mut distances: HashMap<String, HashMap<String, usize>> = HashMap::new();
    for src in nodes.iter() {
        for dst in nodes.iter() {
            distances
                .entry(src.clone())
                .or_default()
                .insert(dst.clone(), distance(&tunnels, src, dst));
        }
    }
    let total = 30 * pressures.values().sum::<usize>();
    let start = State {
        valve,
        duration_remaining: 30,
        valves_remaining: pressures
            .iter()
            .filter_map(|(v, p)| if *p != 0 { Some(v.clone()) } else { None })
            .collect(),
    };
    let (path, cost) = dijkstra(
        &start,
        |s| s.neighbors(&pressures, &distances),
        |s| s.valves_remaining.is_empty() || s.duration_remaining == 0,
    )
    .unwrap();
    dbg!(&path);
    dbg!(total);
    dbg!(&cost);
    let answer = total as i64 - cost;
    let pressure_released = pressure_released(&pressures, &path[..]);
    dbg!(answer);
    dbg!(pressure_released);
    assert!(answer != 1601); // Too low
    assert!(answer != 1628);
    Ok(pressure_released)
}

pub fn part_2(input: &str) -> anyhow::Result<usize> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use crate::testing::{assert_correct_answer_on_correct_input, assert_error_on_wrong_input};
    use crate::Part;

    use super::*;

    #[test]
    fn part_1_works_on_example() {
        assert_correct_answer_on_correct_input!(part_1, "example", Part::One);
    }

    #[test]
    fn part_1_works_on_input() {
        assert_correct_answer_on_correct_input!(part_1, "6bb0c0bd67", Part::One);
    }

    #[test]
    fn part_2_works_on_example() {
        assert_correct_answer_on_correct_input!(part_2, "example", Part::Two);
    }

    #[test]
    fn part_2_works_on_input() {
        assert_correct_answer_on_correct_input!(part_2, "6bb0c0bd67", Part::Two);
    }

    #[test]
    fn returns_error_on_wrong_input() {
        assert_error_on_wrong_input!(part_1, part_2);
    }
}
