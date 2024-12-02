use itertools::Itertools;

pub fn part_1(input: &str) -> anyhow::Result<i64> {
    let mut num_safe = 0;
    for line in input.lines() {
        let mut is_safe = true;
        let mut sign = None;
        for (l, r) in line
            .split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .tuple_windows()
        {
            if l.abs_diff(r) > 3 {
                is_safe = false;
            }
            if l < r {
                if sign.unwrap_or(1) != 1 {
                    is_safe = false;
                } else {
                    sign = Some(1)
                }
            } else if l > r {
                if sign.unwrap_or(-1) != -1 {
                    is_safe = false;
                } else {
                    sign = Some(-1)
                }
            } else {
                is_safe = false;
            }
        }
        if is_safe {
            num_safe += 1;
        }
    }
    Ok(num_safe)
}

pub fn part_2(input: &str) -> anyhow::Result<i64> {
    Ok(0)
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
        assert_correct_answer_on_correct_input!(part_1, "INPUT", Part::One);
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
