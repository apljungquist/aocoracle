use num::pow;
use std::mem::swap;

pub fn part_1(input: &str) -> anyhow::Result<u32> {
    let mut sum = 0;
    for line in input.lines() {
        let mut vs = line.chars().map(|d| d.to_digit(10).unwrap()).rev();
        let mut b = vs.next().unwrap();
        let mut a = vs.next().unwrap();
        for v in vs {
            if v >= a {
                if a > b {
                    b = a;
                }
                a = v;
            }
        }
        sum += (10 * a + b);
    }
    Ok(sum)
}

pub fn part_2(input: &str) -> anyhow::Result<u64> {
    let mut sum = 0;
    for line in input.lines() {
        let mut vs = line.chars().map(|d| d.to_digit(10).unwrap()).rev();
        let mut chosen = [0; 12];
        for x in chosen.iter_mut().rev() {
            *x = vs.next().unwrap();
        }
        for mut v in vs {
            for i in 0..12 {
                if v >= chosen[i] {
                    swap(&mut chosen[i], &mut v);
                } else {
                    break;
                }
            }
        }
        let mut joltage = 0;
        for (i, x) in chosen.into_iter().rev().enumerate() {
            joltage += pow(10, i) * x as u64;
        }
        sum += joltage;
    }
    Ok(sum)
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
        assert_correct_answer_on_correct_input!(part_1, "5d4bde087ea8aad8", Part::One);
    }

    #[test]
    fn part_2_works_on_example() {
        assert_correct_answer_on_correct_input!(part_2, "EXAMPLE", Part::Two);
    }

    #[test]
    fn part_2_works_on_input() {
        assert_correct_answer_on_correct_input!(part_2, "5d4bde087ea8aad8", Part::Two);
    }

    #[test]
    fn returns_error_on_wrong_input() {
        assert_error_on_wrong_input!(part_1, part_2);
    }
}
