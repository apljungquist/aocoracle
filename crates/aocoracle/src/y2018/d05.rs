use std::str::FromStr;

use anyhow::{anyhow, bail};

#[derive(Debug)]
struct Input {
    polymer: Vec<u8>,
}

fn same_type_opposite_polarity(left: u8, right: u8) -> bool {
    left.abs_diff(right) == 32
}

fn same_type(left: u8, right: u8) -> bool {
    left == right || left.abs_diff(right) == 32
}

fn reduced(mut right: Vec<u8>) -> Vec<u8> {
    right.reverse();
    let mut left = Vec::with_capacity(right.len());
    while !right.is_empty() {
        if left.is_empty() {
            left.push(right.pop().unwrap());
        } else if same_type_opposite_polarity(*left.last().unwrap(), *right.last().unwrap()) {
            left.pop();
            right.pop();
        } else {
            left.push(right.pop().unwrap());
        }
    }
    left
}

fn reduced_without_unit_type(polymer: &[u8], unit_type: u8) -> Vec<u8> {
    let mut polymer = Vec::from(polymer);
    polymer.retain(|&u| !same_type(unit_type, u));
    reduced(polymer)
}

impl Input {
    fn try_part_one(&self) -> anyhow::Result<usize> {
        Ok(reduced(self.polymer.clone()).len())
    }

    fn try_part_two(&self) -> anyhow::Result<usize> {
        let initial = reduced(self.polymer.clone());
        if initial.is_empty() {
            log::warn!("Polymer fully reduced before part 2");
            return Ok(0);
        }
        Ok((65..=90)
            .map(|u| reduced_without_unit_type(&initial, u).len())
            .min()
            .expect("Hard coded range is not empty"))
    }
}

impl FromStr for Input {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = regex::Regex::new(r"^([A-Za-z]+)$").expect("Hard coded regex is valid");
        let mut lines = s.lines();
        let line = lines
            .next()
            .ok_or_else(|| anyhow!("Expected at least one line"))?;
        if lines.next().is_some() {
            bail!("Expected no more than one line");
        }
        let cap = re
            .captures(line)
            .ok_or_else(|| anyhow!("Could not capture line {line}"))?;

        Ok(Self {
            polymer: cap[0].bytes().collect(),
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
    use crate::testing::{actual_answer2, assert_returns_error_on_wrong_input2, expected_answer};
    use crate::Part;

    use super::*;

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

    //Fails on 2015/04/3ba7923eae
    // That is a signle line of lower case letters which is unlikely to be an official input to
    // this problem but it nonetheless seems like it should be considered valid.
    #[ignore]
    #[test]
    fn returns_error_on_wrong_input() {
        assert_returns_error_on_wrong_input2(file!(), &part_1, &part_2);
    }
}
