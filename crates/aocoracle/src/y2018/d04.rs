use hashbrown::HashMap;

use anyhow::{anyhow, bail};
use std::str::FromStr;

use itertools::Itertools;

use crate::itersum;

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
    fn minutes_asleep(&self) -> anyhow::Result<Vec<(usize, u8)>> {
        let mut result = Vec::new();
        let mut guard_id = 0;
        let mut guard_asleep_since = 0;
        for entry in self.entries.iter() {
            match entry {
                Entry::Begin(id) => {
                    guard_id = *id;
                }
                Entry::Sleep(minute) => {
                    guard_asleep_since = *minute;
                }
                Entry::Wake(minute) => {
                    for i in guard_asleep_since..*minute {
                        result.push((guard_id, i));
                    }
                }
            }
        }
        if result.is_empty() {
            bail!("Expected at least one guard to go asleep");
        }
        Ok(result)
    }
    fn try_part_one(&self) -> anyhow::Result<usize> {
        let minutes_asleep = self.minutes_asleep()?;
        let total_sleep = minutes_asleep.iter().map(|(id, _)| id).counts();
        let chosen_id = **itersum::unambiguous_argmax(total_sleep.iter()).map_err(|e| match e {
            itersum::AggregationError::TooFew => {
                panic!("minutes_asleep guarantees there is at least one guard")
            }
            itersum::AggregationError::TooMany => {
                anyhow::Error::from(e).context("Failed to pick a single guard")
            }
        })?;
        let chosen_minute = itersum::unambiguous_argmax(
            minutes_asleep
                .into_iter()
                .filter_map(|(id, minute)| if id == chosen_id { Some(minute) } else { None })
                .counts()
                .into_iter(),
        )
        .map_err(|e| match e {
            itersum::AggregationError::TooFew => {
                panic!("minutes_asleep guarantees there is at least one minute")
            }
            itersum::AggregationError::TooMany => {
                anyhow::Error::from(e).context("Failed to pick a single minute")
            }
        })? as usize;
        Ok(chosen_id * chosen_minute)
    }

    fn try_part_two(&self) -> anyhow::Result<usize> {
        let minutes_asleep = self.minutes_asleep()?;
        let mut sleep_minutes = HashMap::new();
        for (guard, minute) in minutes_asleep.into_iter() {
            let entry = sleep_minutes.entry(guard).or_insert_with(HashMap::new);
            *entry.entry(minute).or_insert(0) += 1;
        }
        let sleep_minutes = sleep_minutes;
        let guard2chosen_minute = sleep_minutes
            .into_iter()
            .map(|(id, counts)| {
                (
                    id,
                    counts.into_iter().max_by_key(|(_, count)| *count).unwrap(),
                )
            })
            .collect::<HashMap<_, _>>();
        let chosen = guard2chosen_minute
            .into_iter()
            .max_by_key(|(_, (_, count))| *count)
            .expect("minutes_asleep guarantees there is at least one guard");
        Ok(chosen.0 * (chosen.1 .0 as usize))
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
