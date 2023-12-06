use anyhow::anyhow;

struct Range {
    start: u32,
    end: u32,
}

impl Range {
    fn includes(&self, other: &Self) -> bool {
        self.start <= other.start && other.end <= self.end
    }

    fn intersects(&self, other: &Self) -> bool {
        let start = self.start.max(other.start);
        let end = self.end.min(other.end);
        start <= end
    }
}

fn ranges(s: &str) -> anyhow::Result<Vec<(Range, Range)>> {
    let re =
        regex::Regex::new(r"^(\d+)+-(\d+)+,(\d+)+-(\d+)+$").expect("Hard coded regex is valid");
    let mut result = Vec::new();
    for line in s.lines() {
        let cap = re
            .captures(line)
            .ok_or_else(|| anyhow!("Could not capture ranges on line {}", line))?;
        result.push((
            Range {
                start: cap[1].parse::<u32>()?,
                end: cap[2].parse::<u32>()?,
            },
            Range {
                start: cap[3].parse::<u32>()?,
                end: cap[4].parse::<u32>()?,
            },
        ));
    }
    Ok(result)
}

pub fn part_1(input: &str) -> anyhow::Result<usize> {
    Ok(ranges(input)?
        .iter()
        .filter(|(first, second)| first.includes(second) || second.includes(first))
        .count())
}

pub fn part_2(input: &str) -> anyhow::Result<usize> {
    Ok(ranges(input)?
        .iter()
        .filter(|(first, second)| first.intersects(second))
        .count())
}

#[cfg(test)]
mod tests {
    use crate::testing::{assert_correct_answer_on_correct_input, assert_error_on_wrong_input};
    use crate::Part;

    use super::*;

    #[test]
    fn part_1_works_on_example() {
        assert_correct_answer_on_correct_input!("EXAMPLE", Part::One);
    }

    #[test]
    fn part_1_works_on_input() {
        assert_correct_answer_on_correct_input!("ecf39a26ddeb670e", Part::One);
    }

    #[test]
    fn part_2_works_on_example() {
        assert_correct_answer_on_correct_input!("EXAMPLE", Part::Two);
    }

    #[test]
    fn part_2_works_on_input() {
        assert_correct_answer_on_correct_input!("ecf39a26ddeb670e", Part::Two);
    }

    #[test]
    fn returns_error_on_wrong_input() {
        assert_error_on_wrong_input!(Part::One, Part::Two);
    }
}
