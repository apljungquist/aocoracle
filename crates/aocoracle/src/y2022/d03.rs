use std::str::FromStr;

use anyhow::bail;
use hashbrown::HashSet;
use itertools::Itertools;

fn priority(item: u8) -> Option<u32> {
    match item {
        _ if (65..97).contains(&item) => Some(item as u32 - 65 + 27),
        _ if (97..129).contains(&item) => Some(item as u32 - 97 + 1),
        _ => None,
    }
}

struct Rucksack(Vec<u32>);

impl Rucksack {
    fn compartments(&self) -> (&[u32], &[u32]) {
        let middle = self.0.len() / 2;
        (&self.0[..middle], &self.0[middle..])
    }
}

impl FromStr for Rucksack {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut priorities = Vec::with_capacity(s.len());
        for item in s.bytes() {
            if let Some(priority) = priority(item) {
                priorities.push(priority);
            } else {
                bail!("Expected ASCII letter but got {}", item as char);
            }
        }
        Ok(Self(priorities))
    }
}

fn rucksacks(s: &str) -> anyhow::Result<Vec<Rucksack>> {
    let mut result = Vec::new();
    for line in s.lines() {
        result.push(Rucksack::from_str(line)?);
    }
    if result.len() % 3 != 0 {
        bail!(
            "Expected rucksacks to be divisible into groups of three but got {}",
            result.len()
        );
    }
    Ok(result)
}

pub fn part_1(input: &str) -> anyhow::Result<u32> {
    let mut result = 0;
    for rucksack in rucksacks(input)? {
        let (first, second) = rucksack.compartments();
        let first = HashSet::<&u32>::from_iter(first);
        let second = HashSet::<&u32>::from_iter(second);
        if let Ok(common) = (&first & &second).iter().exactly_one() {
            result += **common
        } else {
            bail!("Expected exactly one item in common");
        }
    }
    Ok(result)
}

pub fn part_2(input: &str) -> anyhow::Result<u32> {
    let mut result = 0;
    let rucksacks = rucksacks(input)?;
    for group in rucksacks.chunks(3) {
        let first = HashSet::<&u32>::from_iter(group[0].0.iter());
        let second = HashSet::<&u32>::from_iter(group[1].0.iter());
        let third = HashSet::<&u32>::from_iter(group[2].0.iter());
        if let Ok(common) = (&(&first & &second) & &third).iter().exactly_one() {
            result += **common
        } else {
            bail!("Expected exactly one item in common");
        }
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use crate::testing::{assert_correct_answer_on_correct_input, assert_error_on_wrong_input};
    use crate::Part;

    use super::*;

    #[test]
    fn part_1_works_on_example() {
        assert_correct_answer_on_correct_input!(part_1, "EXAMPLE", Part::One);
    }

    #[test]
    fn part_1_works_on_input() {
        assert_correct_answer_on_correct_input!(part_1, "0ab407c928b6ffa9", Part::One);
    }

    #[test]
    fn part_2_works_on_example() {
        assert_correct_answer_on_correct_input!(part_2, "EXAMPLE", Part::Two);
    }

    #[test]
    fn part_2_works_on_input() {
        assert_correct_answer_on_correct_input!(part_2, "0ab407c928b6ffa9", Part::Two);
    }

    #[test]
    fn returns_error_on_wrong_input() {
        assert_error_on_wrong_input!(part_1, part_2);
    }
}
