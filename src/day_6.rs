use std::collections::HashMap;
use std::fs;

fn _read_input(name: &str) -> String {
    fs::read_to_string(format!("day/6/{}", name)).unwrap()
}

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

pub fn part_1(filename: &str) -> u64 {
    let initial = _census(&_read_input(filename));
    _nth_census(initial, 80).values().sum()
}

pub fn part_2(filename: &str) -> u64 {
    let initial = _census(&_read_input(filename));
    _nth_census(initial, 256).values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_works_on_example() {
        assert_eq!(part_1("example.txt"), 5934);
    }

    #[test]
    fn part_1_works_on_input() {
        assert_eq!(part_1("input.txt"), 372300);
    }

    #[test]
    fn part_2_works_on_example() {
        assert_eq!(part_2("example.txt"), 26984457539);
    }

    #[test]
    fn part_2_works_on_input() {
        assert_eq!(part_2("input.txt"), 1675781200288);
    }
}
