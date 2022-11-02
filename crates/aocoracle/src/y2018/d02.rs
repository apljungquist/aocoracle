use crate::AnyError;
use hashbrown::HashSet;
use itertools::Itertools;
use std::str::FromStr;

struct Input {
    pub ids: Vec<String>,
}

impl FromStr for Input {
    type Err = AnyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut expected_len = None;
        let mut ids = Vec::new();
        let re = regex::Regex::new(r"^([a-z]+)$").expect("Hard coded regex is valid");
        for line in s.lines() {
            let id: String = re
                .captures(line)
                .ok_or(format!("Could not capture an id on line {}", line))?[1]
                .into();
            if let Some(expected) = expected_len {
                let actual = id.len();
                if actual != expected {
                    return Err(format!(
                        "Expected all ids to be the same length ({expected}) but got {actual}"
                    )
                    .into());
                }
            } else {
                expected_len = Some(id.len());
            }
            ids.push(id);
        }

        if ids.len() < 2 {
            // Avoid FP for 2015/04/3ba7923eae
            return Err(format!(
                "Problem requires at least 2 ids to be solved but got {}",
                ids.len()
            )
            .into());
        }
        Ok(Self { ids })
    }
}

fn exactly_one_corrected(s1: &str, s2: &str) -> Option<String> {
    let len = s1.len();
    assert_eq!(len, s2.len());
    let mut num_err = 0;
    let mut result = String::with_capacity(len - 1);
    for (c1, c2) in s1.chars().zip(s2.chars()) {
        if c1 == c2 {
            result.push(c1);
        } else if num_err == 0 {
            num_err += 1;
        } else {
            return None;
        }
    }
    if num_err == 0 {
        return None;
    }
    Some(result)
}

pub fn part_1(input: &str) -> Result<String, AnyError> {
    let input = Input::from_str(input)?;
    let counts: Vec<HashSet<usize>> = input
        .ids
        .iter()
        .map(|id| id.chars().counts().into_values().collect())
        .collect();
    let num_2 = counts.iter().filter(|count| count.contains(&2)).count();
    let num_3 = counts.iter().filter(|count| count.contains(&3)).count();
    let result = num_2 * num_3;
    Ok(result.to_string())
}

pub fn part_2(input: &str) -> Result<String, AnyError> {
    let input = Input::from_str(input)?;
    Ok(input
        .ids
        .into_iter()
        .tuple_combinations()
        .flat_map(|(s1, s2)| exactly_one_corrected(&s1, &s2))
        .exactly_one()?)
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
        assert_correct_answer(Part::Two, "example2");
    }

    #[test]
    fn part_2_works_on_input() {
        assert_correct_answer(Part::Two, "3ba7923eae");
    }

    #[ignore] // Cannot think of a way to invalidate 2015/05/3ba7923eae
    #[test]
    fn returns_error_on_wrong_input() {
        assert_returns_error_on_wrong_input(file!(), &part_1, &part_2);
    }
}
