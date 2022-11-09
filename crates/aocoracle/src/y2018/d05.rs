use std::str::FromStr;

use anyhow::anyhow;

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
    while let Some(r) = right.pop() {
        if let Some(l) = left.last() {
            if same_type_opposite_polarity(*l, r) {
                left.pop();
                continue;
            }
        }
        left.push(r);
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
        let re = regex::Regex::new(r"(?m)\A([A-Za-z]+)\n\z").expect("Hard coded regex is valid");
        let cap = re
            .captures(s)
            .ok_or_else(|| anyhow!("Regex \"{re:?}\" could not capture text {s:?}"))?;
        Ok(Self {
            polymer: cap[1].bytes().collect(),
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
    // That is a single line of lower case letters which is unlikely to be an official input to
    // this problem but it nonetheless seems like it should be considered valid.
    #[ignore]
    #[test]
    fn returns_error_on_wrong_input() {
        assert_returns_error_on_wrong_input2(file!(), &part_1, &part_2);
    }
}
