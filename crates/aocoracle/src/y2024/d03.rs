use anyhow::{bail, Context};

#[derive(Debug)]
enum Op {
    Do,
    DoNot,
    Mul(i64, i64),
}
fn parse(input: &str) -> anyhow::Result<Vec<Op>> {
    let re = regex::Regex::new(r"(mul|do|don't)\(((\d{1,3}),(\d{1,3}))?\)")
        .expect("hard coded regex is valid");
    let mut instructions: Vec<Op> = Vec::new();
    for cap in re.captures_iter(input) {
        match &cap[1] {
            "do" => instructions.push(Op::Do),
            "don't" => instructions.push(Op::DoNot),
            "mul" => instructions.push(Op::Mul(
                cap.get(3).context("no left operand")?.as_str().parse()?,
                cap.get(4).context("no right operand")?.as_str().parse()?,
            )),
            _ => unreachable!(),
        }
    }
    if instructions.is_empty() {
        bail!("no instructions found");
    }
    Ok(instructions)
}

pub fn part_1(input: &str) -> anyhow::Result<i64> {
    let input = parse(input)?;
    let mut sum = 0;
    for op in input {
        if let Op::Mul(first, second) = op {
            sum += first * second;
        }
    }
    Ok(sum)
}

pub fn part_2(input: &str) -> anyhow::Result<i64> {
    let input = parse(input)?;
    let mut active = true;
    let mut sum = 0;
    for instruction in input {
        match instruction {
            Op::Do => active = true,
            Op::DoNot => active = false,
            Op::Mul(first, second) => {
                if active {
                    sum += first * second
                }
            }
        }
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
        assert_correct_answer_on_correct_input!(part_1, "725feb3ad71b0ac1", Part::One);
    }

    #[test]
    fn part_2_works_on_example() {
        assert_correct_answer_on_correct_input!(part_2, "EXAMPLE2", Part::Two);
    }

    #[test]
    fn part_2_works_on_input() {
        assert_correct_answer_on_correct_input!(part_2, "725feb3ad71b0ac1", Part::Two);
    }

    #[test]
    fn returns_error_on_wrong_input() {
        assert_error_on_wrong_input!(part_1, part_2);
    }
}
