use hashbrown::HashMap;
use std::cmp::Ordering;
use std::str::FromStr;

use itertools::Itertools;

use crate::AnyError;

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

fn unambiguous_argmax<KT, VT, Iter>(mut items: Iter) -> Result<KT, AnyError>
where
    VT: Copy + Ord,
    Iter: Iterator<Item = (KT, VT)>,
{
    // Can be simplified if avoiding ambiguity is not important:
    // items.max_by_key(|(_, v)| *v).map(|(k, _)| k)
    let mut ambiguous = false;
    let mut best = items.next().ok_or_else(|| String::from("Empty"))?;
    for item in items {
        match best.1.cmp(&item.1) {
            Ordering::Less => {
                best = item;
                ambiguous = false;
            }
            Ordering::Equal => {
                ambiguous = true;
            }
            Ordering::Greater => {}
        }
    }
    if ambiguous {
        return Err("Ambiguous".into());
    }
    Ok(best.0)
}

impl Input {
    fn minutes_asleep(&self) -> Result<Vec<(usize, u8)>, AnyError> {
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
            return Err("Expected at least one guard to go asleep".into());
        }
        Ok(result)
    }
    fn try_part_one(&self) -> Result<usize, AnyError> {
        let minutes_asleep = self.minutes_asleep()?;
        let total_sleep = minutes_asleep.iter().map(|(id, _)| id).counts();
        let chosen_id = **unambiguous_argmax(total_sleep.iter())?;
        let chosen_minute = unambiguous_argmax(
            minutes_asleep
                .into_iter()
                .filter_map(|(id, minute)| if id == chosen_id { Some(minute) } else { None })
                .counts()
                .into_iter(),
        )? as usize;
        Ok(chosen_id * chosen_minute)
    }

    fn try_part_two(&self) -> Result<usize, AnyError> {
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
    type Err = AnyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut with_time = Vec::new();
        let re = regex::Regex::new(r"^\[(\d{4}-\d{2}-\d{2} \d{2}:(\d{2}))\] (Guard #(\d+) begins shift|falls asleep|wakes up)$")
            .expect("Hard coded regex is valid");
        for line in s.lines() {
            let cap = re
                .captures(line)
                .ok_or(format!("Could not capture line {}", line))?;

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

pub fn part_1(input: &str) -> Result<String, AnyError> {
    Ok(Input::from_str(input)?.try_part_one()?.to_string())
}

pub fn part_2(input: &str) -> Result<String, AnyError> {
    Ok(Input::from_str(input)?.try_part_two()?.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::{actual_answer, assert_returns_error_on_wrong_input, expected_answer};
    use crate::Part;

    fn assert_correct_answer(part: Part, stem: &str) {
        assert_eq!(
            actual_answer(
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
        assert_returns_error_on_wrong_input(file!(), &part_1, &part_2);
    }
}
