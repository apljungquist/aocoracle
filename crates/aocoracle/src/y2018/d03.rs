use hashbrown::HashSet;
use std::collections::HashMap;

use std::str::FromStr;

use itertools::Itertools;

use crate::rect::Rectangle;
use crate::AnyError;

struct Input {
    claims: HashMap<usize, Rectangle<u32>>,
}

impl Input {
    fn part_one(&self) -> usize {
        // Feels wildly inefficient but it works even in debug mode
        let mut contended = HashSet::new();
        for (id1, claim1) in self.claims.iter() {
            for (id2, claim2) in self.claims.iter() {
                if id1 == id2 {
                    continue;
                }
                if let Some(overlap) = claim1.intersection(claim2) {
                    contended.extend(overlap.tiles());
                }
            }
        }
        contended.len()
    }

    fn try_part_two(&self) -> Result<usize, AnyError> {
        let mut candidates: HashSet<_> = self.claims.keys().cloned().collect();
        for (id1, claim1) in self.claims.iter() {
            for (id2, claim2) in self.claims.iter() {
                if id1 != id2 && claim1.intersection(claim2).is_some() {
                    candidates.remove(id1);
                    candidates.remove(id2);
                }
            }
        }
        Ok(candidates.into_iter().exactly_one()?)
    }
}

impl FromStr for Input {
    type Err = AnyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut claims = HashMap::new();
        let re = regex::Regex::new(r"^#(\d+) @ (\d+),(\d+): (\d+)x(\d+)$")
            .expect("Hard coded regex is valid");
        for line in s.lines() {
            let cap = re
                .captures(line)
                .ok_or(format!("Could not capture a claim on line {}", line))?;

            claims.insert(
                cap[1].parse::<usize>()?,
                Rectangle::<u32>::new(
                    cap[2].parse::<u32>()?,
                    cap[3].parse::<u32>()?,
                    cap[4].parse::<u32>()?,
                    cap[5].parse::<u32>()?,
                ),
            );
        }
        Ok(Self { claims })
    }
}

pub fn part_1(input: &str) -> Result<String, AnyError> {
    Ok(Input::from_str(input)?.part_one().to_string())
}

pub fn part_2(input: &str) -> Result<String, AnyError> {
    Ok(Input::from_str(input)?.try_part_two()?.to_string())
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
        assert_correct_answer_on_correct_input!(part_1, "3ba7923eae", Part::One);
    }

    #[test]
    fn part_2_works_on_example() {
        assert_correct_answer_on_correct_input!(part_2, "example", Part::Two);
    }

    #[test]
    fn part_2_works_on_input() {
        assert_correct_answer_on_correct_input!(part_2, "3ba7923eae", Part::Two);
    }

    #[test]
    fn returns_error_on_wrong_input() {
        assert_error_on_wrong_input!(part_1, part_2);
    }
}
