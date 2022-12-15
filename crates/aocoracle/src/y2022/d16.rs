use anyhow::{anyhow, bail};
use hashbrown::{HashMap, HashSet};
use pathfinding::prelude::dijkstra;
use std::fmt::{Display, Formatter};
use std::hash::Hash;
use std::str::FromStr;

struct Cave {
    start: Valve,
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
            pressures.insert(src, cap[2].parse()?);
            for dst in cap[3].split(',') {
                let dst = dst.trim().parse()?;
                tunnels.entry(src).or_default().insert(dst);
            }
        }
        let start = "AA".parse().expect("Hard coded valve is valid");
        Ok(Self {
            start,
            pressures,
            tunnels,
        })
    }
}

impl Cave {
    fn start(&self, duration_remaining: usize) -> State {
        State {
            elf: self.start,
            elephant: self.start,
            duration_remaining,
            valves_remaining: self
                .pressures
                .iter()
                .filter_map(|(v, p)| if *p != 0 { Some(*v) } else { None })
                .collect(),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
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
    fn neighbors1(
        &self,
        valves: &HashMap<Valve, usize>,
        tunnels: &HashMap<Valve, HashSet<Valve>>,
    ) -> Vec<(Self, usize)> {
        let rate_remaining: usize = self.valves_remaining.iter().map(|v| valves[v]).sum();
        let duration_remaining = self.duration_remaining - 1;
        let mut result = Vec::new();
        // elf moves
        for elf in tunnels[&self.elf].iter() {
            result.push((
                Self {
                    elf: *elf,
                    valves_remaining: self.valves_remaining.clone(),
                    duration_remaining,
                    ..*self
                },
                rate_remaining,
            ));
        }
        // elf opens
        result.push((
            Self {
                valves_remaining: {
                    let mut valves = self.valves_remaining.clone();
                    valves.retain(|v| *v != self.elf);
                    valves
                },
                duration_remaining,
                ..*self
            },
            rate_remaining,
        ));
        result
    }

    fn neighbors2(
        &self,
        valves: &HashMap<Valve, usize>,
        tunnels: &HashMap<Valve, HashSet<Valve>>,
    ) -> Vec<(Self, usize)> {
        let rate_remaining = self.valves_remaining.iter().map(|v| valves[v]).sum();
        let duration_remaining = self.duration_remaining - 1;
        let mut result = Vec::new();
        // both move
        for elf in tunnels[&self.elf].iter() {
            for elephant in tunnels[&self.elephant].iter() {
                result.push((
                    Self {
                        elf: *elf,
                        elephant: *elephant,
                        valves_remaining: self.valves_remaining.clone(),
                        duration_remaining,
                    },
                    rate_remaining,
                ));
            }
        }
        // elf moves, elephant opens
        for elf in tunnels[&self.elf].iter() {
            result.push((
                Self {
                    elf: *elf,
                    valves_remaining: {
                        let mut valves = self.valves_remaining.clone();
                        valves.retain(|v| *v != self.elephant);
                        valves
                    },
                    duration_remaining,
                    ..*self
                },
                rate_remaining,
            ));
        }
        // elephant moves, elf opens
        for elephant in tunnels[&self.elephant].iter() {
            result.push((
                Self {
                    elephant: *elephant,
                    valves_remaining: {
                        let mut valves = self.valves_remaining.clone();
                        valves.retain(|v| *v != self.elf);
                        valves
                    },
                    duration_remaining,
                    ..*self
                },
                rate_remaining,
            ));
        }
        // both open
        result.push((
            Self {
                valves_remaining: {
                    let mut valves = self.valves_remaining.clone();
                    valves.retain(|v| *v != self.elf && *v != self.elephant);
                    valves
                },
                duration_remaining,
                ..*self
            },
            rate_remaining,
        ));
        result
    }
}

fn part_x<F, IN>(cave: &Cave, start_duration: usize, successors: F) -> anyhow::Result<usize>
where
    F: FnMut(&State) -> IN,
    IN: IntoIterator<Item = (State, usize)>,
{
    let total_pressure = start_duration * cave.pressures.values().sum::<usize>();
    let start_state = cave.start(start_duration);
    // TODO: Find a better way to search for the solution, perhaps a-star.
    let (_, pressure_not_released) = dijkstra(&start_state, successors, |s| {
        s.valves_remaining.is_empty() || s.duration_remaining == 0
    })
    .unwrap();
    let pressure_released = total_pressure - pressure_not_released;
    Ok(pressure_released)
}

pub fn part_1(input: &str) -> anyhow::Result<usize> {
    let cave = Cave::from_str(input)?;
    part_x(&cave, 30, |s| s.neighbors1(&cave.pressures, &cave.tunnels))
}

pub fn part_2(input: &str) -> anyhow::Result<usize> {
    let cave = Cave::from_str(input)?;
    part_x(&cave, 26, |s| s.neighbors2(&cave.pressures, &cave.tunnels))
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
