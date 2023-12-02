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
    use crate::testing::{assert_correct_answer_on_correct_input, assert_error_on_wrong_input};
    use crate::Part;

    use super::*;

    #[test]
    fn part_1_works_on_example() {
        assert_correct_answer_on_correct_input!(part_1, "EXAMPLE", Part::One);
    }

    #[test]
    fn part_1_works_on_input() {
        assert_correct_answer_on_correct_input!(part_1, "e76707b9f6829f2f", Part::One);
    }

    #[test]
    fn part_2_works_on_example() {
        assert_correct_answer_on_correct_input!(part_2, "EXAMPLE", Part::Two);
    }

    #[test]
    fn part_2_works_on_input() {
        assert_correct_answer_on_correct_input!(part_2, "e76707b9f6829f2f", Part::Two);
    }

    #[ignore]
    #[test]
    fn returns_error_on_wrong_input() {
        assert_error_on_wrong_input!(&part_1, &part_2);
    }
}
