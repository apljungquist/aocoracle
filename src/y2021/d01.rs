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
    Ok(format!(
        "{}",
        depths.windows(2).filter(|w| w[0] < w[1]).count()
    ))
}

pub fn part_2(input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let depths: Vec<u32> = _depths(input)?.windows(3).map(|w| w.iter().sum()).collect();
    Ok(format!(
        "{}",
        depths.windows(2).filter(|w| w[0] < w[1]).count()
    ))
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
