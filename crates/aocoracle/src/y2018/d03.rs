use hashbrown::HashSet;
use std::collections::HashMap;
use std::hash::Hash;
use std::str::FromStr;

use itertools::Itertools;

use crate::AnyError;

#[derive(Clone, Hash)]
struct Claim {
    left: u32,
    top: u32,
    width: u32,
    height: u32,
}

impl Claim {
    fn coordinates(&self) -> Vec<(u32, u32)> {
        let mut result = Vec::with_capacity((self.height * self.width) as usize);
        for x in self.left..self.left + self.width {
            for y in self.top..self.top + self.height {
                result.push((x, y));
            }
        }
        result
    }

    fn has_overlap(&self, other: &Claim) -> bool {
        let top = self.top.max(other.top);
        let right = (self.left + self.width).min(other.left + other.width);
        let bottom = (self.top + self.height).min(other.top + other.width);
        let left = self.left.max(other.left);
        left < right && top < bottom
    }
}

struct Input {
    claims: HashMap<usize, Claim>,
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
                Claim {
                    left: cap[2].parse::<u32>()?,
                    top: cap[3].parse::<u32>()?,
                    width: cap[4].parse::<u32>()?,
                    height: cap[5].parse::<u32>()?,
                },
            );
        }
        Ok(Self { claims })
    }
}

pub fn part_1(input: &str) -> Result<String, AnyError> {
    let input = Input::from_str(input)?;
    // Feels wildly inefficient but it works even in debug mode
    let counts = input
        .claims
        .values()
        .flat_map(|claim| claim.coordinates())
        .counts();
    Ok(counts
        .values()
        .filter(|&&count| count > 1)
        .count()
        .to_string())
}

pub fn part_2(input: &str) -> Result<String, AnyError> {
    let input = Input::from_str(input)?;
    let mut candidates: HashSet<_> = input.claims.keys().cloned().collect();
    for (id1, claim1) in input.claims.iter() {
        for (id2, claim2) in input.claims.iter() {
            if id1 != id2 && claim1.has_overlap(claim2) {
                candidates.remove(id1);
                candidates.remove(id2);
            }
        }
    }
    Ok(candidates.into_iter().exactly_one()?.to_string())
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
