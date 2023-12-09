use itertools::Itertools;

fn parsed_input(input: &str) -> anyhow::Result<Vec<Vec<i64>>> {
    let mut result = vec![];
    for line in input.lines() {
        let mut numbers = vec![];
        for number in line.split_whitespace() {
            numbers.push(number.parse()?);
        }
        result.push(numbers);
    }
    Ok(result)
}
fn extrapolate(history: &[i64]) -> i64 {
    let mut diffs = vec![history.to_vec()];
    loop {
        let prev = diffs.last().unwrap();
        let curr: Vec<_> = prev.iter().tuple_windows().map(|(&l, &r)| r - l).collect();
        if curr.iter().all(|&v| v == 0) {
            break;
        }
        diffs.push(curr);
    }
    let mut step = 0;
    while let Some(diff) = diffs.pop() {
        step += diff.iter().last().unwrap();
    }
    step
}
pub fn part_1(input: &str) -> anyhow::Result<i64> {
    let histories = parsed_input(input)?;
    Ok(histories.iter().map(|h| extrapolate(h)).sum())
}

pub fn part_2(input: &str) -> anyhow::Result<usize> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use crate::testing::{assert_correct_answer_on_correct_input, assert_error_on_wrong_input};
    use crate::Part;

    use super::*;

    #[test]
    fn extrapolate_works_on_first_degree_example() {
        assert_eq!(extrapolate(&vec![0, 3, 6, 9, 12, 15]), 18);
    }

    #[test]
    fn extrapolate_works_on_third_degree_example() {
        assert_eq!(extrapolate(&vec![10, 13, 16, 21, 30, 45]), 68);
    }

    #[test]
    fn part_1_works_on_example() {
        assert_correct_answer_on_correct_input!(part_1, "EXAMPLE", Part::One);
    }

    #[test]
    fn part_1_works_on_input() {
        assert_correct_answer_on_correct_input!(part_1, "INPUT", Part::One);
        // > 253652923
    }

    #[test]
    fn part_2_works_on_example() {
        assert_correct_answer_on_correct_input!(part_2, "EXAMPLE", Part::Two);
    }

    #[test]
    fn part_2_works_on_input() {
        assert_correct_answer_on_correct_input!(part_2, "INPUT", Part::Two);
    }

    #[test]
    fn returns_error_on_wrong_input() {
        assert_error_on_wrong_input!(part_1, part_2);
    }
}
