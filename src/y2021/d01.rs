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
    use super::*;
    use crate::testing::compute_answer;

    #[test]
    fn part_1_works_on_example() {
        assert_eq!(compute_answer(file!(), part_1, "example"), "7");
    }

    #[test]
    fn part_1_works_on_input() {
        assert_eq!(compute_answer(file!(), part_1, "input"), "1139");
    }

    #[test]
    fn part_2_works_on_example() {
        assert_eq!(compute_answer(file!(), part_2, "example"), "5");
    }

    #[test]
    fn part_2_works_on_input() {
        assert_eq!(compute_answer(file!(), part_2, "input"), "1103");
    }
}
