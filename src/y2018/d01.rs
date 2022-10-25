use std::fs;

type AnyError = Box<dyn std::error::Error>;

fn _changes(text: &str) -> Result<Vec<(i32)>, AnyError> {
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
    unimplemented!()
}

fn _from_file<F, T>(func: F, stem: &str) -> T
where
    F: Fn(&str) -> Result<T, AnyError>,
{
    func(&fs::read_to_string(format!("inputs/y2018/d01/{}.txt", stem)).unwrap()).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_works_on_example_s() {
        assert_eq!(_from_file(part_1, "example"), "3");
    }

    #[test]
    fn part_1_works_on_input() {
        assert_eq!(_from_file(part_1, "6bb0c0bd67"), "502");
    }

    #[test]
    fn part_2_works_on_example_l() {
        assert_eq!(_from_file(part_2, "example"), "1");
    }

    #[test]
    fn part_2_works_on_input() {
        assert_eq!(_from_file(part_2, "6bb0c0bd67"), "272");
    }
}
