use std::collections::HashMap;
use std::fs;

fn _census(line: &str) -> HashMap<u32, u64> {
    let mut result = HashMap::new();
    for countdown in line.trim_end().split(',').map(|s| {
        s.parse::<u32>()
            .unwrap_or_else(|_| panic!("Expected an int but got '{}'", s))
    }) {
        *(result.entry(countdown).or_insert(0)) += 1;
    }
    result
}

fn _next_census(prev: &HashMap<u32, u64>) -> HashMap<u32, u64> {
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

fn _nth_census(initial: HashMap<u32, u64>, n: u32) -> HashMap<u32, u64> {
    let mut census = initial;
    for _ in 0..n {
        census = _next_census(&census);
    }
    census
}

pub fn part_1(input: &str) -> u64 {
    let initial = _census(input);
    _nth_census(initial, 80).values().sum()
}

pub fn part_2(input: &str) -> u64 {
    let initial = _census(input);
    _nth_census(initial, 256).values().sum()
}

fn _from_file<F, T>(func: F, stem: &str) -> T
where
    F: Fn(&str) -> T,
{
    func(&fs::read_to_string(format!("day/6/{}.txt", stem)).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_works_on_example() {
        assert_eq!(_from_file(part_1, "example"), 5934);
    }

    #[test]
    fn part_1_works_on_input() {
        assert_eq!(_from_file(part_1, "input"), 372300);
    }

    #[test]
    fn part_2_works_on_example() {
        assert_eq!(_from_file(part_2, "example"), 26984457539);
    }

    #[test]
    fn part_2_works_on_input() {
        assert_eq!(_from_file(part_2, "input"), 1675781200288);
    }
}
