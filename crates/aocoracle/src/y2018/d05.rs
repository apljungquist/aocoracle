use anyhow::anyhow;

fn same_type_opposite_polarity(left: u8, right: u8) -> bool {
    left.abs_diff(right) == 32
}

fn same_type(left: u8, right: u8) -> bool {
    left == right || left.abs_diff(right) == 32
}

fn reduced(mut right: Vec<u8>) -> Vec<u8> {
    right.reverse();
    let mut left = Vec::with_capacity(right.len());
    while let Some(r) = right.pop() {
        if let Some(l) = left.last() {
            if same_type_opposite_polarity(*l, r) {
                left.pop();
                continue;
            }
        }
        left.push(r);
    }
    left
}

fn reduced_without_unit_type(polymer: &[u8], unit_type: u8) -> Vec<u8> {
    let mut polymer = Vec::from(polymer);
    polymer.retain(|&u| !same_type(unit_type, u));
    reduced(polymer)
}

fn polymer_from_str(s: &str) -> anyhow::Result<Vec<u8>> {
    let re = regex::Regex::new(r"(?m)\A([A-Za-z]+)\n\z").expect("Hard coded regex is valid");
    let cap = re
        .captures(s)
        .ok_or_else(|| anyhow!("Regex \"{re:?}\" could not capture text {s:?}"))?;
    Ok(cap[1].bytes().collect())
}

pub fn part_1(input: &str) -> anyhow::Result<usize> {
    let polymer = polymer_from_str(input)?;
    Ok(reduced(polymer).len())
}

pub fn part_2(input: &str) -> anyhow::Result<usize> {
    let polymer = polymer_from_str(input)?;
    let initial = reduced(polymer);
    if initial.is_empty() {
        log::warn!("Polymer fully reduced before part 2");
        return Ok(0);
    }
    Ok((65..=90)
        .map(|u| reduced_without_unit_type(&initial, u).len())
        .min()
        .expect("Hard coded range is not empty"))
}

#[cfg(test)]
mod tests {
    use crate::testing::{assert_correct_answer_on_correct_input, assert_error_on_wrong_input};
    use crate::Part;

    use super::*;

    #[test]
    fn part_1_works_on_example() {
        assert_correct_answer_on_correct_input!(part_1, "example", Part::One);
    }

    #[test]
    fn part_1_works_on_input() {
        assert_correct_answer_on_correct_input!(part_1, "3ba7923eae", Part::One);
    }

    #[test]
    fn part_2_works_on_example() {
        assert_correct_answer_on_correct_input!(part_2, "example", Part::Two);
    }

    #[test]
    fn part_2_works_on_input() {
        assert_correct_answer_on_correct_input!(part_2, "3ba7923eae", Part::Two);
    }

    //Fails on 2015/04/3ba7923eae
    // That is a single line of lower case letters which is unlikely to be an official input to
    // this problem but it nonetheless seems like it should be considered valid.
    #[ignore]
    #[test]
    fn returns_error_on_wrong_input() {
        assert_error_on_wrong_input!(part_1, part_2);
    }
}
