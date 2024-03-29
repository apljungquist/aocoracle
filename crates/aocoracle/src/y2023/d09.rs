use anyhow::bail;
use itertools::Itertools;

fn parsed_histories(input: &str) -> anyhow::Result<Vec<Vec<i64>>> {
    let mut histories = vec![];
    for line in input.lines() {
        let mut values = vec![];
        for value in line.split(' ') {
            values.push(value.parse()?);
        }
        if values.len() < 2 {
            bail!("Expected at least two values per line")
        }
        histories.push(values);
    }
    if histories.len() < 2 {
        bail!("Expected at least two histories")
    }
    Ok(histories)
}

fn extrapolation(history: &[i64]) -> (i64, i64) {
    let mut diffs = vec![history.to_vec()];
    loop {
        let prev = diffs
            .last()
            .expect("We start with one element and we never remove any elements");
        let curr: Vec<_> = prev.iter().tuple_windows().map(|(&l, &r)| r - l).collect();
        if curr.iter().all(|&v| v == 0) {
            break;
        }
        diffs.push(curr);
    }
    let mut before = 0;
    let mut after = 0;
    while let Some(diff) = diffs.pop() {
        before = -before
            + diff
                .first()
                .expect("We only push vectors with at least one (non-zero) element");
        after += diff
            .iter()
            .last()
            .expect("We only push vectors with at least one (non-zero) element");
    }
    (before, after)
}

pub fn part_1(input: &str) -> anyhow::Result<i64> {
    let histories = parsed_histories(input)?;
    Ok(histories.iter().map(|h| extrapolation(h).1).sum())
}

pub fn part_2(input: &str) -> anyhow::Result<i64> {
    let histories = parsed_histories(input)?;
    Ok(histories.iter().map(|h| extrapolation(h).0).sum())
}

#[cfg(test)]
mod tests {
    use crate::testing::{assert_correct_answer_on_correct_input, assert_error_on_wrong_input};
    use crate::Part;

    use super::*;

    #[test]
    fn extrapolate_works_on_first_degree_example() {
        assert_eq!(extrapolation(&[0, 3, 6, 9, 12, 15]), (-3, 18));
    }

    #[test]
    fn extrapolate_works_on_third_degree_example() {
        assert_eq!(extrapolation(&[10, 13, 16, 21, 30, 45]), (5, 68));
    }

    #[test]
    fn part_1_works_on_example() {
        assert_correct_answer_on_correct_input!(part_1, "EXAMPLE", Part::One);
    }

    #[test]
    fn part_1_works_on_input() {
        assert_correct_answer_on_correct_input!(part_1, "9a8b8ea7bb09b5a1", Part::One);
        // > 253652923
    }

    #[test]
    fn part_2_works_on_example() {
        assert_correct_answer_on_correct_input!(part_2, "EXAMPLE", Part::Two);
    }

    #[test]
    fn part_2_works_on_input() {
        assert_correct_answer_on_correct_input!(part_2, "9a8b8ea7bb09b5a1", Part::Two);
    }

    #[test]
    fn returns_error_on_wrong_input() {
        assert_error_on_wrong_input!(part_1, part_2);
    }
}
