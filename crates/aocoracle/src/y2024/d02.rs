use itertools::Itertools;

fn parse(s: &str) -> anyhow::Result<Vec<Vec<i64>>> {
    let mut reports = Vec::new();
    for line in s.lines() {
        let mut report = Vec::new();
        for level in line.split_whitespace() {
            report.push(level.parse()?);
        }
        reports.push(report);
    }
    Ok(reports)
}

fn is_safe(report: Vec<i64>) -> bool {
    let mut signum = None;
    for (left, right) in report.into_iter().tuple_windows() {
        let diff = right - left;
        if *signum.get_or_insert(diff.signum()) != diff.signum() {
            return false;
        }
        if !(1..=3).contains(&diff.abs()) {
            return false;
        }
    }
    true
}

fn is_safe_with_dampener(report: Vec<i64>) -> bool {
    for i in 0..report.len() {
        let mut tmp = report.clone();
        tmp.remove(i);
        if is_safe(tmp) {
            return true;
        }
    }
    false
}
pub fn part_1(input: &str) -> anyhow::Result<u64> {
    let reports = parse(input)?;
    Ok(reports
        .into_iter()
        .map(|r| if is_safe(r) { 1 } else { 0 })
        .sum())
}

pub fn part_2(input: &str) -> anyhow::Result<i64> {
    let reports = parse(input)?;
    Ok(reports
        .into_iter()
        .map(|r| if is_safe_with_dampener(r) { 1 } else { 0 })
        .sum())
}

#[cfg(test)]
mod tests {
    use crate::testing::{assert_correct_answer_on_correct_input, assert_error_on_wrong_input};
    use crate::Part;

    use super::*;
    #[test]
    fn part_1_works_on_example() {
        assert_correct_answer_on_correct_input!(part_1, "EXAMPLE", Part::One);
    }

    #[test]
    fn part_1_works_on_input() {
        assert_correct_answer_on_correct_input!(part_1, "3d11a16614884d53", Part::One);
    }

    #[test]
    fn part_2_works_on_example() {
        assert_correct_answer_on_correct_input!(part_2, "EXAMPLE", Part::Two);
    }

    #[test]
    fn part_2_works_on_input() {
        assert_correct_answer_on_correct_input!(part_2, "3d11a16614884d53", Part::Two);
        // 242 is too low
    }

    #[test]
    fn returns_error_on_wrong_input() {
        assert_error_on_wrong_input!(part_1, part_2);
    }
}
