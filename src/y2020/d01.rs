use crate::AnyError;

fn _numbers(text: &str) -> Result<Vec<u32>, AnyError> {
    let mut result = Vec::new();
    for line in text.lines() {
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
    use crate::testing::{actual_answer, expected_answer};

    use super::*;

    fn assert_correct_answer(part: u8, stem: &str) {
        assert_eq!(
            actual_answer(
                file!(),
                match part {
                    1 => part_1,
                    2 => part_2,
                    _ => panic!(),
                },
                stem
            ),
            expected_answer(file!(), part, stem).unwrap(),
        )
    }

    #[test]
    fn part_1_works_on_example() {
        assert_correct_answer(1, "example");
    }
    #[test]
    fn part_1_works_on_input() {
        assert_correct_answer(1, "6bb0c0bd67");
    }
    #[test]
    fn part_2_works_on_example() {
        assert_correct_answer(2, "example");
    }
    #[test]
    fn part_2_works_on_input() {
        assert_correct_answer(2, "6bb0c0bd67");
    }
}
