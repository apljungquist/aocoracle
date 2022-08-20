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
    use super::*;
    use crate::testing::compute_answer;

    #[test]
    fn part_1_works_on_example_s() {
        assert_eq!(compute_answer(file!(), part_1, "example"), "514579");
    }

    #[test]
    fn part_1_works_on_input() {
        assert_eq!(compute_answer(file!(), part_1, "input"), "355875");
    }

    #[test]
    fn part_2_works_on_example_l() {
        assert_eq!(compute_answer(file!(), part_2, "example"), "241861950");
    }

    #[test]
    fn part_2_works_on_input() {
        assert_eq!(compute_answer(file!(), part_2, "input"), "140379120");
    }
}
