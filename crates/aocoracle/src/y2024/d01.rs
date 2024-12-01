use itertools::Itertools;

pub fn part_1(input: &str) -> anyhow::Result<u64> {
    let mut lhs = Vec::new();
    let mut rhs = Vec::new();
    for line in input.lines() {
        let mut numbers = line.split_whitespace();
        let l = numbers.next().unwrap().parse::<i64>()?;
        let r = numbers.next().unwrap().parse::<i64>()?;
        lhs.push(l);
        rhs.push(r);
    }
    lhs.sort();
    rhs.sort();
    let sum = lhs.into_iter().zip(rhs.into_iter()).map(|(l, r)| {l.abs_diff(r)}).sum();
    Ok(sum)
}

pub fn part_2(input: &str) -> anyhow::Result<usize> {
    let mut lhs = Vec::new();
    let mut rhs = Vec::new();
    for line in input.lines() {
        let mut numbers = line.split_whitespace();
        let l = numbers.next().unwrap().parse::<i64>()?;
        let r = numbers.next().unwrap().parse::<i64>()?;
        lhs.push(l);
        rhs.push(r);
    }
    lhs.sort();
    rhs.sort();
    let counts = rhs.into_iter().counts_by(|x| x);
    let sum = lhs.into_iter().map(|(l)| {l as usize * counts.get(&l).unwrap_or(&0)}).sum();
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
