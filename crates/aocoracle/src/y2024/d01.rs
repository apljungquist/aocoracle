use anyhow::Context;
use itertools::Itertools;
use std::str::FromStr;

struct Lists {
    left: Vec<u64>,
    right: Vec<u64>,
}

impl FromStr for Lists {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut left = Vec::new();
        let mut right = Vec::new();
        for line in s.lines() {
            let (l, r) = line
                .split_once("   ")
                .context("Expected location IDs to be separated by '   '")?;
            left.push(l.parse::<_>()?);
            right.push(r.parse::<_>()?);
        }
        Ok(Self { left, right })
    }
}

pub fn part_1(input: &str) -> anyhow::Result<u64> {
    let Lists {
        mut left,
        mut right,
    }: Lists = input.parse()?;
    left.sort_unstable();
    right.sort_unstable();
    let sum = left
        .into_iter()
        .zip(right.into_iter())
        .map(|(l, r)| l.abs_diff(r))
        .sum();
    Ok(sum)
}

pub fn part_2(input: &str) -> anyhow::Result<usize> {
    let Lists {
        mut left,
        mut right,
    }: Lists = input.parse()?;
    left.sort_unstable();
    right.sort_unstable();
    let counts = right.into_iter().counts_by(|x| x);
    let sum = left
        .into_iter()
        .map(|(l)| l as usize * counts.get(&l).unwrap_or(&0))
        .sum();
    Ok(sum)
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
        assert_correct_answer_on_correct_input!(part_1, "f6ebca937a28aa07", Part::One);
    }

    #[test]
    fn part_2_works_on_example() {
        assert_correct_answer_on_correct_input!(part_2, "EXAMPLE", Part::Two);
    }

    #[test]
    fn part_2_works_on_input() {
        assert_correct_answer_on_correct_input!(part_2, "f6ebca937a28aa07", Part::Two);
    }

    #[test]
    fn returns_error_on_wrong_input() {
        assert_error_on_wrong_input!(part_1, part_2);
    }
}
