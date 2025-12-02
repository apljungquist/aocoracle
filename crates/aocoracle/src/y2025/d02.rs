struct Range {
    a: u64,
    b: u64,
}

fn is_invalid(s: &str) -> bool {
    let m = s.len() / 2;
    s[..m] == s[m..]
}

pub fn part_1(input: &str) -> anyhow::Result<u64> {
    let mut sum = 0;
    for r in input.trim().split(',') {
        let (a, b) = r.split_once('-').unwrap();
        let a = a.parse::<u64>()?;
        let b = b.parse::<u64>()?;
        for i in a..=b {
            if is_invalid(i.to_string().as_str()) {
                println!("invalid: {}", i);
                sum += i;
            }
        }
    }

    Ok(sum)
}

fn is_invalid2(s: &str) -> bool {
    (s);
    for i in 1..=(s.len() / 2) {
        if s.len() % i != 0 {
            continue;
        }
        (i);
        let mut is_valid = false;
        for j in 1..(s.len() / i) {
            let left = ((j - 1) * i);
            let mid = (j * i);
            let right = ((j + 1) * i);
            if (&s[left..mid]) != (&s[mid..right]) {
                is_valid = true;
                break
            }
        }
        if !is_valid {
            return true;
        }
    }
    false
}

pub fn part_2(input: &str) -> anyhow::Result<u64> {
    let mut sum = 0;
    for r in input.trim().split(',') {
        let (a, b) = r.split_once('-').unwrap();
        let a = a.parse::<u64>()?;
        let b = b.parse::<u64>()?;
        for i in a..=b {
            if is_invalid2(i.to_string().as_str()) {
                // println!("invalid: {}", i);
                sum += i;
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
        assert_correct_answer_on_correct_input!(part_1, "41133bb41f758fd1", Part::One);
    }

    #[test]
    fn part_2_works_on_example() {
        assert_correct_answer_on_correct_input!(part_2, "EXAMPLE", Part::Two);
    }

    #[test]
    fn part_2_works_on_input() {
        assert_correct_answer_on_correct_input!(part_2, "41133bb41f758fd1", Part::Two);
    }

    #[test]
    fn returns_error_on_wrong_input() {
        assert_error_on_wrong_input!(part_1, part_2);
    }
}
