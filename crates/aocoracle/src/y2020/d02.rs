use crate::AnyError;
use itertools::Itertools;

struct Policy {
    lo: usize,
    hi: usize,
    ch: char,
}

impl Policy {
    fn validates(self, password: &str) -> bool {
        let mut count = 0;
        for ch in password.chars() {
            if ch == self.ch {
                count += 1;
            }
        }
        self.lo <= count && count <= self.hi
    }
    fn validates2(self, password: &str) -> bool {
        let chars: Vec<char> = password.chars().collect();
        let a = chars[self.lo - 1];
        let b = chars[self.hi - 1];
        if a == b {
            return false;
        }
        if a != self.ch && b != self.ch {
            return false;
        }
        true
    }
}

fn _passwords(text: &str) -> Result<Vec<(Policy, String)>, AnyError> {
    let re = regex::Regex::new(r"^(\d+)-(\d+) (\w): (\w+)$").expect("Hard coded regex is valid");
    let mut result = Vec::new();
    for line in text.lines() {
        let cap = re
            .captures(line)
            .ok_or(format!("Could not capture a password on line {}", line))?;
        result.push((
            Policy {
                lo: cap[1].parse::<usize>()?,
                hi: cap[2].parse::<usize>()?,
                ch: cap[3]
                    .chars()
                    .exactly_one()
                    .expect("Hard coded regex captures exactly one char"),
            },
            cap[4].into(),
        ))
    }
    Ok(result)
}

pub fn part_1(input: &str) -> Result<String, AnyError> {
    let num_valid: u32 = _passwords(input)?
        .into_iter()
        .map(|(policy, password)| policy.validates(&password) as u32)
        .sum();
    Ok(format!("{}", num_valid))
}

pub fn part_2(input: &str) -> Result<String, AnyError> {
    let num_valid: u32 = _passwords(input)?
        .into_iter()
        .map(|(policy, password)| policy.validates2(&password) as u32)
        .sum();
    Ok(format!("{}", num_valid))
}
#[cfg(test)]
mod tests {
    use crate::testing::{assert_correct_answer_on_correct_input, assert_error_on_wrong_input};
    use crate::Part;

    use super::*;

    #[test]
    fn part_1_works_on_example() {
        assert_correct_answer_on_correct_input!("EXAMPLE", Part::One);
    }

    #[test]
    fn part_1_works_on_input() {
        assert_correct_answer_on_correct_input!("6aa335f0a1070945", Part::One);
    }

    #[test]
    fn part_2_works_on_example() {
        assert_correct_answer_on_correct_input!("EXAMPLE", Part::Two);
    }

    #[test]
    fn part_2_works_on_input() {
        assert_correct_answer_on_correct_input!("6aa335f0a1070945", Part::Two);
    }

    #[test]
    fn returns_error_on_wrong_input() {
        assert_error_on_wrong_input!(Part::One, Part::Two);
    }
}
