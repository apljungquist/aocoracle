use std::fs;

type AnyError = Box<dyn std::error::Error>;

fn _numbers(text: &str) -> Vec<u32> {
    text.lines()
        .map(|line| line.parse::<u32>().unwrap())
        .collect()
}

pub fn part_1(input: &str) -> Result<String, AnyError> {
    let numbers = _numbers(input);
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
    let numbers = _numbers(input);
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

fn _from_file<F, T>(func: F, stem: &str) -> T
where
    F: Fn(&str) -> Result<T, AnyError>,
{
    func(&fs::read_to_string(format!("inputs/y20d01/{}.txt", stem)).unwrap()).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_works_on_example_s() {
        assert_eq!(_from_file(part_1, "example"), "514579");
    }

    #[test]
    fn part_1_works_on_input() {
        assert_eq!(_from_file(part_1, "input"), "355875");
    }

    #[test]
    fn part_2_works_on_example_l() {
        assert_eq!(_from_file(part_2, "example"), "241861950");
    }

    #[test]
    fn part_2_works_on_input() {
        assert_eq!(_from_file(part_2, "input"), "140379120");
    }
}
