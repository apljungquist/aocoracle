use anyhow::{anyhow, bail};
use hashbrown::{HashMap, HashSet};
use pathfinding::prelude::{bfs, dijkstra};
use std::fmt::{Display, Formatter};
use std::hash::Hash;
use std::str::FromStr;

struct Cave {
    pressures: HashMap<Valve, usize>,
    tunnels: HashMap<Valve, HashSet<Valve>>,
}

impl FromStr for Cave {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = regex::Regex::new(
            r"^Valve ([A-Z]{2}) has flow rate=(\d+); tunnels? leads? to valves? ([A-Z, ]+)$",
        )
        .expect("Hard coded regex is valid");
        let mut pressures = HashMap::new();
        let mut tunnels: HashMap<Valve, HashSet<Valve>> = HashMap::new();
        for line in s.lines() {
            let cap = re
                .captures(line)
                .ok_or_else(|| anyhow!("Could not capture line {}", line))?;
            let src: Valve = cap[1].parse()?;
            pressures.insert(src.clone(), cap[2].parse()?);
            for dst in cap[3].split(',') {
                let dst = dst.trim().parse()?;
                tunnels.entry(src.clone()).or_default().insert(dst);
            }
        }
        Ok(Self { pressures, tunnels })
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Valve(u16);

impl Display for Valve {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0 as u8 as char)?;
        write!(f, "{}", (self.0 >> 8) as u8 as char)?;
        Ok(())
    }
}

impl FromStr for Valve {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 2 {
            bail!("Expected exactly 2 bytes but got {}", s.len());
        }
        let s = s.as_bytes();
        Ok(Self((s[0] as u16) + ((s[1] as u16) << 8)))
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct State {
    elf: Valve,
    elephant: Valve,
    valves_remaining: Vec<Valve>,
    duration_remaining: usize,
}

impl State {
    fn neighbors(
        &self,
        valves: &HashMap<Valve, usize>,
        distances: &HashMap<Valve, HashMap<Valve, usize>>,
    ) -> Vec<(State, i64)> {
        let rate_remaining: usize = self.valves_remaining.iter().map(|v| valves[v]).sum();
        let mut result: Vec<(State, i64)> = self
            .valves_remaining
            .iter()
            .filter(|valve| self.duration_remaining >= distances[&self.elf][*valve])
            .map(|valve| {
                let duration_remaining =
                    self.duration_remaining - distances[&self.elf][valve] as usize;
                (
                    Self {
                        elf: valve.clone(),
                        elephant: self.elephant.clone(),
                        duration_remaining,
                        valves_remaining: {
                            let mut valves = self.valves_remaining.clone();
                            valves.retain(|v| v != valve);
                            valves
                        },
                    },
                    (rate_remaining * distances[&self.elf][valve]) as i64,
                )
            })
            .collect();
        result.push((
            Self {
                elf: self.elf.clone(),
                elephant: self.elephant.clone(),
                duration_remaining: self.duration_remaining - 1,
                valves_remaining: self.valves_remaining.clone(),
            },
            rate_remaining.try_into().unwrap(),
        ));
        result
    }
    fn neighbors2(
        &self,
        valves: &HashMap<Valve, usize>,
        tunnels: &HashMap<Valve, HashSet<Valve>>,
    ) -> Vec<(State, i64)> {
        let rate_remaining = self
            .valves_remaining
            .iter()
            .map(|v| valves[v])
            .sum::<usize>() as i64;
        let duration_remaining = self.duration_remaining - 1;
        let mut result = Vec::new();
        for elf in tunnels[&self.elf].iter() {
            for elephant in tunnels[&self.elephant].iter() {
                // Move
                result.push((
                    State {
                        elf: elf.clone(),
                        elephant: elephant.clone(),
                        valves_remaining: self.valves_remaining.clone(),
                        duration_remaining,
                    },
                    rate_remaining,
                ));
            }
        }
        for elf in tunnels[&self.elf].iter() {
            result.push((
                State {
                    elf: elf.clone(),
                    elephant: self.elephant.clone(),
                    valves_remaining: {
                        let mut valves = self.valves_remaining.clone();
                        valves.retain(|v| *v != self.elephant);
                        valves
                    },
                    duration_remaining,
                },
                rate_remaining,
            ));
        }
        for elephant in tunnels[&self.elephant].iter() {
            result.push((
                State {
                    elf: self.elf.clone(),
                    elephant: elephant.clone(),
                    valves_remaining: {
                        let mut valves = self.valves_remaining.clone();
                        valves.retain(|v| *v != self.elf);
                        valves
                    },
                    duration_remaining,
                },
                rate_remaining,
            ));
        }
        result.push((
            State {
                elf: self.elf.clone(),
                elephant: self.elephant.clone(),
                valves_remaining: {
                    let mut valves = self.valves_remaining.clone();
                    valves.retain(|v| *v != self.elf && *v != self.elephant);
                    valves
                },
                duration_remaining,
            },
            rate_remaining,
        ));
        result
    }
}

fn distance(edges: &HashMap<Valve, HashSet<Valve>>, src: &Valve, dst: &Valve) -> usize {
    bfs(src, |t| edges[t].clone(), |t| t == dst).unwrap().len()
}

pub fn part_1(input: &str) -> anyhow::Result<usize> {
    let cave = Cave::from_str(input)?;
    let valve: Valve = "AA".parse().expect("Hard coded valve is valid");
    let mut nodes = HashSet::new();
    for (src, dsts) in cave.tunnels.iter() {
        nodes.insert(src.clone());
        nodes.extend(dsts.iter().cloned());
    }
    let mut distances: HashMap<Valve, HashMap<Valve, usize>> = HashMap::new();
    for src in nodes.iter() {
        for dst in nodes.iter() {
            distances
                .entry(src.clone())
                .or_default()
                .insert(dst.clone(), distance(&cave.tunnels, src, dst));
        }
    }
    let total = 30 * cave.pressures.values().sum::<usize>();
    let start = State {
        elf: valve.clone(),
        elephant: valve,
        duration_remaining: 30,
        valves_remaining: cave
            .pressures
            .iter()
            .filter_map(|(v, p)| if *p != 0 { Some(v.clone()) } else { None })
            .collect(),
    };
    let (_, cost) = dijkstra(
        &start,
        |s| s.neighbors(&cave.pressures, &distances),
        |s| s.valves_remaining.is_empty() || s.duration_remaining == 0,
    )
    .unwrap();
    let answer = total as i64 - cost;
    Ok(answer.try_into()?)
}

pub fn part_2(input: &str) -> anyhow::Result<usize> {
    let cave = Cave::from_str(input)?;
    let valve: Valve = "AA".parse().expect("Hard coded valve is valid");
    let mut nodes = HashSet::new();
    for (src, dsts) in cave.tunnels.iter() {
        nodes.insert(src.clone());
        nodes.extend(dsts.iter().cloned());
    }
    let mut distances: HashMap<Valve, HashMap<Valve, usize>> = HashMap::new();
    for src in nodes.iter() {
        for dst in nodes.iter() {
            distances
                .entry(src.clone())
                .or_default()
                .insert(dst.clone(), distance(&cave.tunnels, src, dst));
        }
    }
    let total = 26 * cave.pressures.values().sum::<usize>();
    let start = State {
        elf: valve.clone(),
        elephant: valve,
        duration_remaining: 26,
        valves_remaining: cave
            .pressures
            .iter()
            .filter_map(|(v, p)| if *p != 0 { Some(v.clone()) } else { None })
            .collect(),
    };
    let (_, cost) = dijkstra(
        &start,
        |s| s.neighbors2(&cave.pressures, &cave.tunnels),
        |s| s.valves_remaining.is_empty() || s.duration_remaining == 0,
    )
    .unwrap();
    let answer = total as i64 - cost;
    Ok(answer.try_into()?)
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

    #[test]
    fn valve_from_string_to_string() {
        for expected in ["AA", "AZ", "ZA", "ZZ"] {
            assert_eq!(Valve::from_str(expected).unwrap().to_string(), expected)
        }
    }
}
