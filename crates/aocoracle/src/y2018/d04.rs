use anyhow::anyhow;
use std::str::FromStr;

use itertools::Itertools;

use crate::itersum::Itersum;

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
enum Entry {
    Begin(usize),
    Sleep(u8),
    Wake(u8),
}

#[derive(Debug)]
struct Input {
    entries: Vec<Entry>,
}

impl Input {
    fn minutes_asleep(&self) -> Vec<(usize, u8)> {
        let mut result = Vec::new();
        let mut guard = 0;
        let mut guard_asleep_since = 0;
        for entry in self.entries.iter() {
            match entry {
                Entry::Begin(g) => {
                    guard = *g;
                }
                Entry::Sleep(m) => {
                    guard_asleep_since = *m;
                }
                Entry::Wake(m) => {
                    for i in guard_asleep_since..*m {
                        result.push((guard, i));
                    }
                }
            }
        }
        result
    }
    fn try_part_one(&self) -> anyhow::Result<usize> {
        let minutes_asleep = self.minutes_asleep();
        let guard = *minutes_asleep.iter().map(|(g, _)| g).mode()?;
        let minute = minutes_asleep
            .into_iter()
            .filter_map(|(g, m)| if g == guard { Some(m) } else { None })
            .mode()?;
        Ok(guard * minute as usize)
    }

    fn try_part_two(&self) -> anyhow::Result<usize> {
        let (guard, minute) = self.minutes_asleep().into_iter().mode()?;
        Ok(guard * minute as usize)
    }
}

impl FromStr for Input {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut with_time = Vec::new();
        let re = regex::Regex::new(r"^\[(\d{4}-\d{2}-\d{2} \d{2}:(\d{2}))\] (Guard #(\d+) begins shift|falls asleep|wakes up)$")
            .expect("Hard coded regex is valid");
        for line in s.lines() {
            let cap = re
                .captures(line)
                .ok_or_else(|| anyhow!("Could not capture line {line:}"))?;

            let datetime = String::from(&cap[1]);
            let minute = cap[2].parse::<u8>()?;
            let entry = match &cap[3] {
                "falls asleep" => Entry::Sleep(minute),
                "wakes up" => Entry::Wake(minute),
                _ => Entry::Begin(cap[4].parse::<usize>()?),
            };
            with_time.push((datetime, entry));
        }
        Ok(Self {
            entries: with_time.into_iter().sorted().map(|(_, v)| v).collect(),
        })
    }
}

pub fn part_1(input: &str) -> anyhow::Result<String> {
    Ok(Input::from_str(input)?.try_part_one()?.to_string())
}

pub fn part_2(input: &str) -> anyhow::Result<String> {
    Ok(Input::from_str(input)?.try_part_two()?.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::{actual_answer2, assert_returns_error_on_wrong_input2, expected_answer};
    use crate::Part;

    fn assert_correct_answer(part: Part, stem: &str) {
        assert_eq!(
            actual_answer2(
                file!(),
                match part {
                    Part::One => part_1,
                    Part::Two => part_2,
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
        assert_returns_error_on_wrong_input2(file!(), &part_1, &part_2);
    }
}
