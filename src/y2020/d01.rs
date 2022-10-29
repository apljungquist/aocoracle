use crate::AnyError;

fn _numbers(text: &str) -> Result<Vec<u32>, AnyError> {
    let mut result = Vec::new();
    for line in text.lines() {
        if line.starts_with('0') {
            return Err("Integers do not usually start with 0".into());
        }
        result.push(line.parse::<u32>()?);
    }
    Ok(result)
}

pub fn part_1(input: &str) -> Result<String, AnyError> {
    let numbers = _numbers(input)?;
    for x in numbers.iter() {
        for y in numbers.iter() {
            if x + y == 2020 {
                return Ok(format!("{}", x * y));
            }
        }
    }
    Err("No answer".into())
}

pub fn part_2(input: &str) -> Result<String, AnyError> {
    let numbers = _numbers(input)?;
    for x in numbers.iter() {
        for y in numbers.iter() {
            if x + y >= 2020 {
                continue;
            }
            for z in numbers.iter() {
                if x + y + z == 2020 {
                    return Ok(format!("{}", x * y * z));
                }
            }
        }
    }
    Err("No answer".into())
}

#[cfg(test)]
mod tests {
    use crate::testing::{actual_answer, assert_returns_error_on_wrong_input, expected_answer};
    use crate::Part;

    use super::*;

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
        assert_correct_answer(Part::One, "6bb0c0bd67");
    }
    #[test]
    fn part_2_works_on_example() {
        assert_correct_answer(Part::Two, "example");
    }
    #[test]
    fn part_2_works_on_input() {
        assert_correct_answer(Part::Two, "6bb0c0bd67");
    }

    #[ignore]
    #[test]
    fn returns_error_on_wrong_input() {
        assert_returns_error_on_wrong_input(file!(), &part_1, &part_2);
    }
}
