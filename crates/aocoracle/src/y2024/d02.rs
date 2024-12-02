use itertools::Itertools;

fn problem(line:&[i64]) -> Option<(usize, usize)> {
    let mut sign = None;
    for j in 1..line.len() {
        let i = j - 1;
        let l = line[i];
        let r = line[j];

        if l.abs_diff(r) > 3 {
            return Some((i,j))
        }
        if l < r {
            if sign.unwrap_or(1) != 1 {
                return Some((i,j))
            } else {
                sign = Some(1)
            }
        } else if l > r {
            if sign.unwrap_or(-1) != -1 {
                return Some((i,j))
            } else {
                sign = Some(-1)
            }
        } else {
            return Some((i,j))
        }
    }
    None
}
pub fn part_1(input: &str) -> anyhow::Result<i64> {
    let mut num_safe = 0;
    for line in input.lines() {
        let line: Vec<_> =  line
            .split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect();
        if problem(&line).is_none() {num_safe +=1}
    }
    Ok(num_safe)
}


pub fn part_2(input: &str) -> anyhow::Result<i64> {
    let mut num_safe = 0;
    for line in input.lines() {
        let mut line: Vec<_> =  line
            .split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect();
        if let Some(_) = problem(&line) {
            for i in 0..line.len() {
                let mut tmp = line.clone();
                tmp.remove(i);
                if problem(&tmp).is_none() {
                    num_safe += 1;
                    break
                }
            }
        } else {
            num_safe += 1;
        }
    }
    Ok(num_safe)
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
        // 242 is too low
    }

    #[test]
    fn returns_error_on_wrong_input() {
        assert_error_on_wrong_input!(part_1, part_2);
    }
}
