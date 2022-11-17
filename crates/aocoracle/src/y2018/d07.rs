use std::cmp::Reverse;
use std::fmt::Debug;
use std::hash::Hash;
use std::str::FromStr;

use anyhow::{anyhow, bail};
use hashbrown::{HashMap, HashSet};
use itertools::Itertools;

struct Planner<T> {
    ready: HashSet<T>,
    started: HashSet<T>,
    succ2precs: HashMap<T, HashSet<T>>,
}

impl<T> Planner<T>
where
    T: Copy + Eq + Hash + Debug,
{
    pub fn new(mut succ2precs: HashMap<T, HashSet<T>>) -> Self {
        let ready = succ2precs
            .drain_filter(|_, precs| precs.is_empty())
            .map(|(succ, _)| succ)
            .collect();
        Self {
            ready,
            started: HashSet::new(),
            succ2precs,
        }
    }

    pub fn start_task(&mut self, node: &T) -> anyhow::Result<()> {
        if let Some(node) = self.ready.take(node) {
            self.started.insert(node);
        } else {
            bail!("Cannot start task if it is not ready")
        }
        Ok(())
    }

    pub fn complete_task(&mut self, node: &T) -> anyhow::Result<()> {
        if !self.started.remove(node) {
            bail!("Cannot complete task if it is not started")
        }
        println!("Marking {node:?} as done");
        for (succ, precs) in self.succ2precs.iter_mut() {
            if precs.remove(node) {
                println!("Removed prec {:?} to succ {:?}", node, succ);
            }
        }
        for (succ, _) in self.succ2precs.drain_filter(|_, precs| precs.is_empty()) {
            self.ready.insert(succ);
            println!("Moved succ {:?} to ready", succ);
        }
        Ok(())
    }

    pub fn is_done(&self) -> bool {
        self.succ2precs.is_empty() && self.ready.is_empty()
    }
}

impl<T> Planner<T>
where
    T: Copy + Ord,
{
    pub fn try_first_ready(&self) -> anyhow::Result<T> {
        Ok(*self
            .ready
            .iter()
            .max_by_key(|k| Reverse(*k))
            .ok_or_else(|| anyhow!("Graph has at least one cycle"))?)
    }
}

struct Executor {
    now: u16,
    base_duration: u16,
    capacity: usize,
    tasks: HashMap<u8, u16>,
}

fn first<T, U>(tuple: (T, U)) -> T {
    tuple.0
}

impl Executor {
    fn new(base_duration: u16, capacity: usize) -> Self {
        Self {
            now: 0,
            base_duration,
            capacity,
            tasks: HashMap::new(),
        }
    }

    fn is_full(&self) -> bool {
        self.capacity == self.tasks.len()
    }

    fn start_task(&mut self, task: u8) {
        let now = self.now;
        let end_time = now + task as u16 + self.base_duration - 64;
        println!("Starting {task} at {now} lasting until {end_time}");
        self.tasks.insert(task, end_time);
    }
    fn remove_done(&mut self) -> anyhow::Result<Vec<u8>> {
        let now = *self
            .tasks
            .values()
            .min()
            .ok_or_else(|| anyhow!("No tasks done"))?;
        self.now = now;
        let tasks = self
            .tasks
            .drain_filter(|_task, end_time| *end_time == self.now)
            .map(first)
            .collect();
        println!("Stopped {tasks:?} at {now}");
        Ok(tasks)
    }
    fn now(self) -> u16 {
        self.now
    }
}

#[derive(Debug)]
struct Input {
    pub graph: HashMap<u8, HashSet<u8>>,
}

impl Input {
    fn try_part_one(self) -> anyhow::Result<String> {
        let mut planner = Planner::new(self.graph);
        let mut result = String::new();
        while !planner.is_done() {
            let node = planner.try_first_ready()?;
            result.push(node as char);
            planner.start_task(&node).unwrap();
            planner.complete_task(&node).unwrap();
        }
        Ok(result)
    }

    fn try_part_two(self, num_worker: usize, base_duration: u16) -> anyhow::Result<String> {
        let mut planner = Planner::new(self.graph);
        let mut executor = Executor::new(base_duration, num_worker);

        let mut result = String::new();
        while !planner.is_done() {
            println!("Result: {result}");
            while !executor.is_full() {
                if let Ok(node) = planner.try_first_ready() {
                    planner.start_task(&node).unwrap();
                    executor.start_task(node);
                } else {
                    break;
                }
            }

            for task in executor.remove_done()? {
                result.push(task as char);
                println!("Task: {task}");
                planner.complete_task(&task)?;
            }
        }
        Ok(executor.now().to_string())
    }
}

impl FromStr for Input {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut graph: HashMap<u8, HashSet<u8>> = HashMap::new();
        let re =
            regex::Regex::new(r"^Step ([A-Z]) must be finished before step ([A-Z]) can begin.$")
                .expect("Hard coded regex is valid");
        for line in s.lines() {
            let cap = re
                .captures(line)
                .ok_or_else(|| anyhow!("Regex \"{re:?}\" could not capture line {line:?}"))?;
            let predecessor = cap[1]
                .chars()
                .exactly_one()
                .expect("Regex matches exactly one char")
                .try_into()?;
            let successor = cap[2]
                .chars()
                .exactly_one()
                .expect("Regex matches exactly one char")
                .try_into()?;
            graph.entry(predecessor).or_default();
            graph.entry(successor).or_default().insert(predecessor);
        }
        Ok(Self { graph })
    }
}

pub fn part_1(input: &str) -> anyhow::Result<String> {
    Input::from_str(input)?.try_part_one()
}

pub fn _part_2a(input: &str) -> anyhow::Result<String> {
    Input::from_str(input)?.try_part_two(2, 0)
}

pub fn part_2b(input: &str) -> anyhow::Result<String> {
    Input::from_str(input)?.try_part_two(5, 60)
}

#[cfg(test)]
mod tests {
    use crate::testing::{actual_answer2, assert_returns_error_on_wrong_input2, expected_answer};
    use crate::Part;

    use super::*;

    fn assert_correct_answer(part: Part, stem: &str) {
        assert_eq!(
            actual_answer2(
                file!(),
                match part {
                    Part::One => part_1,
                    Part::Two => match stem {
                        "example" => _part_2a,
                        _ => part_2b,
                    },
                },
                stem,
            ),
            expected_answer(file!(), part, stem).unwrap(),
        )
    }

    #[test]
    fn part_1_works_on_example() {
        assert_correct_answer(Part::One, "example");
    }

    #[test]
    fn part_1_works_on_input() {
        assert_correct_answer(Part::One, "3ba7923eae");
    }

    #[test]
    fn part_2_works_on_example() {
        assert_correct_answer(Part::Two, "example");
    }

    #[test]
    fn part_2_works_on_input() {
        assert_correct_answer(Part::Two, "3ba7923eae");
    }

    #[test]
    fn returns_error_on_wrong_input() {
        assert_returns_error_on_wrong_input2(file!(), &part_1, &part_2b);
    }
}
