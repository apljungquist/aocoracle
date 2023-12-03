use crate::AnyError;
use hashbrown::HashSet;

fn _changes(text: &str) -> Result<Vec<i32>, AnyError> {
    let re = regex::Regex::new(r"^([-+]\d+)$").expect("Hard coded regex is valid");
    let mut result = Vec::new();
    for line in text.lines() {
        let cap = re
            .captures(line)
            .ok_or(format!("Could not capture a password on line {}", line))?;
        result.push(cap[1].parse::<i32>()?);
    }
    Ok(result)
}

pub fn part_1(input: &str) -> Result<String, AnyError> {
    let frequency: i32 = _changes(input)?.into_iter().sum();
    Ok(format!("{}", frequency))
}

pub fn part_2(input: &str) -> Result<String, AnyError> {
    let mut seen = HashSet::new();

    let mut frequency = 0;
    for delta in _changes(input)?.into_iter().cycle() {
        seen.insert(frequency);
        frequency += delta;
        if seen.contains(&frequency) {
            return Ok(format!("{}", frequency));
        }
    }
    unreachable!(
        "cycle should create an infinite loop and that loop is broken only by the return statement"
    )
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
        assert_correct_answer_on_correct_input!(part_1, "6802c3de5dea7df2", Part::One);
    }

    #[test]
    fn part_2_works_on_example() {
        assert_correct_answer_on_correct_input!(part_2, "EXAMPLE", Part::Two);
    }

    #[test]
    fn part_2_works_on_input() {
        assert_correct_answer_on_correct_input!(part_2, "6802c3de5dea7df2", Part::Two);
    }

    #[test]
    fn returns_error_on_wrong_input() {
        assert_error_on_wrong_input!(part_1, part_2);
    }
}
