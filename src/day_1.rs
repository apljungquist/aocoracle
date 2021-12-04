use std::fs;

fn _read_input(day: u8, name: &str) -> String {
    fs::read_to_string(format!("day/{}/{}", day, name)).unwrap()
}

fn _depths(text: &str) -> Vec<u32> {
    let mut result = Vec::new();
    for line in text.lines() {
        result.push(line.parse::<u32>().unwrap());
    }
    result
}

fn part_1(filename: &str) -> usize {
    let depths = _depths(&_read_input(1, filename));
    depths.windows(2).filter(|w| w[0] < w[1]).count()
}

fn part_2(filename: &str) -> usize {
    let depths: Vec<u32> = _depths(&_read_input(1, filename))
        .windows(3)
        .map(|w| w.into_iter().sum())
        .collect();
    depths.windows(2).filter(|w| w[0] < w[1]).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_works_on_example() {
        assert_eq!(part_1("example.txt"), 7);
    }

    #[test]
    fn part_1_works_on_input() {
        assert_eq!(part_1("input.txt"), 1139);
    }

    #[test]
    fn part_2_works_on_example() {
        assert_eq!(part_2("example.txt"), 5);
    }

    #[test]
    fn part_2_works_on_input() {
        assert_eq!(part_2("input.txt"), 1103);
    }
}
