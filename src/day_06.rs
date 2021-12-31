use std::collections::HashMap;
use std::fs;
type AnyError = Box<dyn std::error::Error>;
type Census = HashMap<u32, u64>;
fn _census(line: &str) -> Result<Census, AnyError> {
    let mut result = HashMap::new();
    for countdown in line.trim_end().split(',') {
        let countdown = countdown.parse::<u32>()?;
        *(result.entry(countdown).or_insert(0)) += 1;
    }
    Ok(result)
}

fn _next_census(prev: &Census) -> Census {
    let mut result = HashMap::with_capacity(prev.len());
    for (countdown, count) in prev {
        match countdown {
            0 => {
                *(result.entry(6).or_insert(0)) += count;
                *(result.entry(8).or_insert(0)) += count;
            }
            _ => {
                *(result.entry(countdown - 1).or_insert(0)) += count;
            }
        }
    }
    result
}

fn _nth_census(initial: Census, n: u32) -> Census {
    let mut census = initial;
    for _ in 0..n {
        census = _next_census(&census);
    }
    census
}

pub fn part_1(input: &str) -> Result<String, AnyError> {
    let initial = _census(input)?;
    let num_fish = _nth_census(initial, 80).values().sum::<u64>();
    Ok(format!("{}", num_fish))
}

pub fn part_2(input: &str) -> Result<String, AnyError> {
    let initial = _census(input)?;
    let num_fish = _nth_census(initial, 256).values().sum::<u64>();
    Ok(format!("{}", num_fish))
}

fn _from_file<F, T>(func: F, stem: &str) -> T
where
    F: Fn(&str) -> Result<T, AnyError>,
{
    func(&fs::read_to_string(format!("inputs/06/{}.txt", stem)).unwrap()).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_works_on_example() {
        assert_eq!(_from_file(part_1, "example"), "5934");
    }

    #[test]
    fn part_1_works_on_input() {
        assert_eq!(_from_file(part_1, "input"), "372300");
    }

    #[test]
    fn part_2_works_on_example() {
        assert_eq!(_from_file(part_2, "example"), "26984457539");
    }

    #[test]
    fn part_2_works_on_input() {
        assert_eq!(_from_file(part_2, "input"), "1675781200288");
    }
}
