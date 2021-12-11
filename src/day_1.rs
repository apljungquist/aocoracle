use std::fs;
fn _depths(text: &str) -> Vec<u32> {
    let mut result = Vec::new();
    for line in text.lines() {
        result.push(line.parse::<u32>().unwrap());
    }
    result
}

pub fn part_1(input: &str) -> usize {
    let depths = _depths(input);
    depths.windows(2).filter(|w| w[0] < w[1]).count()
}

pub fn part_2(input: &str) -> usize {
    let depths: Vec<u32> = _depths(input).windows(3).map(|w| w.iter().sum()).collect();
    depths.windows(2).filter(|w| w[0] < w[1]).count()
}

fn _from_file<F, T>(func: F, stem: &str) -> T
where
    F: Fn(&str) -> T,
{
    func(&fs::read_to_string(format!("day/1/{}.txt", stem)).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_works_on_example() {
        assert_eq!(_from_file(part_1, "example"), 7);
    }

    #[test]
    fn part_1_works_on_input() {
        assert_eq!(_from_file(part_1, "input"), 1139);
    }

    #[test]
    fn part_2_works_on_example() {
        assert_eq!(_from_file(part_2, "example"), 5);
    }

    #[test]
    fn part_2_works_on_input() {
        assert_eq!(_from_file(part_2, "input"), 1103);
    }
}
