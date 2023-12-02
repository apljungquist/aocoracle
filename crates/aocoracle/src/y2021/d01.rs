fn _depths(text: &str) -> Result<Vec<u32>, Box<dyn std::error::Error>> {
    let mut result = Vec::new();
    for line in text.lines() {
        if line
            .chars()
            .next()
            .ok_or("Cannot parse empty line as depth")?
            == '0'
        {
            return Err("Refusing to parse line starting with 0 as depth".into());
        }
        let depth = line.parse::<u32>()?;
        result.push(depth);
    }
    Ok(result)
}

pub fn part_1(input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let depths = _depths(input)?;
    // With 1 it is still possible to compute an answer but it will always be 0
    if depths.len() < 2 {
        return Err(format!("Expected at least 2 depths, got {}", depths.len()).into());
    }
    Ok(format!(
        "{}",
        depths.windows(2).filter(|w| w[0] < w[1]).count()
    ))
}

pub fn part_2(input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let depths: Vec<u32> = _depths(input)?.windows(3).map(|w| w.iter().sum()).collect();
    // With 3 it is still possible to compute an answer but it will always be 0
    if depths.len() < 4 {
        return Err(format!("Expected at least 4 depths, got {}", depths.len()).into());
    }
    Ok(format!(
        "{}",
        depths.windows(2).filter(|w| w[0] < w[1]).count()
    ))
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
        assert_correct_answer_on_correct_input!(part_1, "765bc2a161e7527e", Part::One);
    }

    #[test]
    fn part_2_works_on_example() {
        assert_correct_answer_on_correct_input!(part_2, "EXAMPLE", Part::Two);
    }

    #[test]
    fn part_2_works_on_input() {
        assert_correct_answer_on_correct_input!(part_2, "765bc2a161e7527e", Part::Two);
    }

    #[ignore]
    #[test]
    fn returns_error_on_wrong_input() {
        assert_error_on_wrong_input!(&part_1, &part_2);
    }
}
