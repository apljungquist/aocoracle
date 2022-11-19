use std::str::FromStr;

use anyhow::{anyhow, bail};
use hashbrown::HashMap;
use itertools::Itertools;

mod dag {
    use std::fmt::Debug;
    use std::hash::Hash;

    use anyhow::{anyhow, bail};
    use hashbrown::{HashMap, HashSet};

    pub struct Graph<T> {
        succ2precs: HashMap<T, HashSet<T>>,
    }

    impl<T: Copy + Eq + Hash + Debug> Graph<T> {
        pub fn new() -> Self {
            Self {
                succ2precs: HashMap::new(),
            }
        }

        pub fn insert_edge(&mut self, prec: T, succ: T) {
            self.succ2precs.entry(succ).or_default().insert(prec);
            self.succ2precs.entry(prec).or_default();
        }

        pub fn into_planner(self) -> Planner<T> {
            Planner::new(self.succ2precs)
        }
    }

    pub struct Planner<T> {
        succ2precs: HashMap<T, HashSet<T>>,
        ready: HashSet<T>,
        started: HashSet<T>,
    }

    impl<T: Eq + Hash + Debug> Planner<T> {
        pub fn new(mut succ2precs: HashMap<T, HashSet<T>>) -> Self {
            let ready = succ2precs
                .drain_filter(|_, precs| precs.is_empty())
                .map(|(succ, _)| succ)
                .collect();
            Self {
                succ2precs,
                ready,
                started: HashSet::new(),
            }
        }

        pub fn ready(&self) -> &HashSet<T> {
            &self.ready
        }

        pub fn start(&mut self, task: &T) -> anyhow::Result<()> {
            let task = self
                .ready
                .take(task)
                .ok_or_else(|| anyhow!("Cannot start task {task:?} because it is not ready"))?;
            self.started.insert(task);
            Ok(())
        }

        pub fn complete(&mut self, task: &T) -> anyhow::Result<()> {
            if !self.started.remove(task) {
                bail!("Cannot complete task {task:?} because is not started")
            }
            self.succ2precs.values_mut().for_each(|precs| {
                precs.remove(task);
            });
            self.ready.extend(
                self.succ2precs
                    .drain_filter(|_, precs| precs.is_empty())
                    .map(|(succ, _)| succ),
            );
            Ok(())
        }
    }
}

struct Executor {
    now: u16,
    base_duration: u16,
    capacity: usize,
    tasks: HashMap<Task, u16>,
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

    fn start(&mut self, task: Task) -> Result<(), ()> {
        if self.capacity == self.tasks.len() {
            return Err(());
        }
        let end_time = self.now + self.base_duration + task.duration();
        self.tasks.insert(task, end_time);
        Ok(())
    }

    fn drain_completed(&mut self) -> impl Iterator<Item = Task> + '_ {
        if let Some(&now) = self.tasks.values().min() {
            self.now = now;
        }
        self.tasks
            .drain_filter(|_, end_time| *end_time == self.now)
            .map(|(task, _)| task)
    }

    fn now(self) -> u16 {
        self.now
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Task(u8);

impl Task {
    fn duration(&self) -> u16 {
        self.0 as u16 - 64
    }
}

impl TryFrom<char> for Task {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        if !value.is_ascii_uppercase() {
            bail!("Expected ascii upercase char but got {value}");
        }
        Ok(Self(
            value
                .try_into()
                .expect("Uppercase ascii is can be converted to u8"),
        ))
    }
}

impl From<Task> for char {
    fn from(task: Task) -> Self {
        task.0 as char
    }
}

struct Input {
    pub graph: dag::Graph<Task>,
}

impl Input {
    fn try_part_one(self) -> anyhow::Result<String> {
        let mut planner = self.graph.into_planner();
        let mut result = String::new();
        while let Some(&task) = planner.ready().iter().min() {
            planner.start(&task).unwrap();
            planner.complete(&task).unwrap();
            result.push(task.into());
        }
        Ok(result)
    }

    fn try_part_two(self, num_worker: usize, base_duration: u16) -> anyhow::Result<String> {
        let mut planner = self.graph.into_planner();
        let mut executor = Executor::new(base_duration, num_worker);
        loop {
            while let Some(&task) = planner.ready().iter().min() {
                if executor.start(task).is_err() {
                    break;
                }
                planner.start(&task).unwrap();
            }

            let mut completed_tasks = executor.drain_completed().peekable();
            if completed_tasks.peek().is_none() {
                break; // Either done or encountered a cycle
            }
            for task in completed_tasks {
                planner.complete(&task)?;
            }
        }
        Ok(executor.now().to_string())
    }
}

impl FromStr for Input {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut graph = dag::Graph::new();
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
                .try_into()
                .expect("Regex matches valid task");
            let successor = cap[2]
                .chars()
                .exactly_one()
                .expect("Regex matches exactly one char")
                .try_into()
                .expect("Regex matches valid task");
            graph.insert_edge(predecessor, successor);
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
