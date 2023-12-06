use std::collections::VecDeque;

use anyhow::anyhow;
use hashbrown::HashSet;

fn datastream(s: &str) -> anyhow::Result<&[u8]> {
    let re = regex::Regex::new(r"(?m)\A([a-z]+)\n\z").expect("Hard coded regex is valid");
    if re.is_match(s) {
        Ok(s.as_bytes())
    } else {
        Err(anyhow!("Regex \"{re:?}\" could not capture text {s:?}"))
    }
}

pub fn start_of_message(datastream: &[u8], marker_len: usize) -> anyhow::Result<usize> {
    let mut marker = VecDeque::with_capacity(marker_len);
    for (i, char) in datastream.iter().enumerate() {
        if marker.len() == marker_len {
            marker.pop_front();
        }
        marker.push_back(char);
        if marker.iter().collect::<HashSet<_>>().len() == marker_len {
            return Ok(i + 1);
        }
    }
    Err(anyhow!("No marker found"))
}

pub fn part_1(input: &str) -> anyhow::Result<usize> {
    start_of_message(datastream(input)?, 4)
}

pub fn part_2(input: &str) -> anyhow::Result<usize> {
    start_of_message(datastream(input)?, 14)
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
        assert_correct_answer_on_correct_input!("4b7d68c5eeb71eec", Part::One);
    }

    #[test]
    fn part_2_works_on_example() {
        assert_correct_answer_on_correct_input!("EXAMPLE", Part::Two);
    }

    #[test]
    fn part_2_works_on_input() {
        assert_correct_answer_on_correct_input!("4b7d68c5eeb71eec", Part::Two);
    }

    // Fails on 2015/04/3ba7923eae and possibly others
    // Since it is just a line of lower case characters it seems unlikely that it can be rejected.
    #[ignore]
    #[test]
    fn returns_error_on_wrong_input() {
        assert_error_on_wrong_input!(Part::One, Part::Two);
    }
}
