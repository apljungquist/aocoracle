use anyhow::{anyhow, bail};

fn numbers(s: &str) -> anyhow::Result<Vec<i64>> {
    let re = regex::Regex::new(r"^(-?([1-9]\d*)|0)$").expect("Hard coded regex is valid");
    let mut result = Vec::new();
    for line in s.lines() {
        let cap = re
            .captures(line)
            .ok_or_else(|| anyhow!("Could not capture number on line {}", line))?;
        result.push(cap[1].parse()?);
    }
    let num_zero = result.iter().filter(|n| **n == 0).count();
    if num_zero != 1 {
        bail!("Expected exactly 1 zero but got {num_zero}")
    }
    Ok(result)
}

fn print_numbers(numbers: &[(usize, i64)]) {
    for x in numbers {
        print!("{}, ", x.1);
    }
    println!()
}

fn move_number(mixed: &mut Vec<(usize, i64)>, id: usize) {
    let old = mixed.iter().position(|x| x.0 == id).unwrap();
    let number = mixed.remove(old).1;
    let mut new = (old as i64 + number).rem_euclid(mixed.len() as i64) as usize;
    // Keep the first element the same as in example
    if new == 0 && number < 0 {
        new = mixed.len();
    }
    mixed.insert(new, (id, number));
}

fn part_x(numbers: &[i64], num_round: usize, key: i64) -> anyhow::Result<i64> {
    let ordered: Vec<_> = numbers.iter().map(|x| x * key).collect();
    let mut mixed: Vec<(usize, i64)> = ordered.into_iter().enumerate().collect();
    print_numbers(&mixed);
    for _ in 0..num_round {
        for id in 0..mixed.len() {
            move_number(&mut mixed, id);
        }
        print_numbers(&mixed);
    }
    let i = mixed.iter().position(|(_, n)| *n == 0).unwrap();
    let summands = vec![
        mixed[(i + 1000) % mixed.len()].1,
        mixed[(i + 2000) % mixed.len()].1,
        mixed[(i + 3000) % mixed.len()].1,
    ];
    dbg!(&summands);
    Ok(summands.iter().sum())
}

pub fn part_1(input: &str) -> anyhow::Result<i64> {
    let numbers = numbers(input)?;
    part_x(&numbers, 1, 1)
}

pub fn part_2(input: &str) -> anyhow::Result<i64> {
    let numbers = numbers(input)?;
    let answer = part_x(&numbers, 10, 811589153)?;
    dbg!(answer);
    // 6bb0c0bd67: answer > 190723450955
    // 3ba7923eae: answer != -6161584849576
    Ok(answer)
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
        assert_correct_answer_on_correct_input!(part_1, "6bb0c0bd67", Part::One);
    }

    #[test]
    fn part_2_works_on_example() {
        assert_correct_answer_on_correct_input!(part_2, "example", Part::Two);
    }

    #[test]
    fn part_2_works_on_input() {
        assert_correct_answer_on_correct_input!(part_2, "6bb0c0bd67", Part::Two);
    }

    #[test]
    fn returns_error_on_wrong_input() {
        assert_error_on_wrong_input!(part_1, part_2);
    }

    #[test]
    fn part_1_works_on_3ba7923eae() {
        assert_correct_answer_on_correct_input!(part_1, "3ba7923eae", Part::One);
    }

    #[test]
    fn part_2_works_on_3ba7923eae() {
        assert_correct_answer_on_correct_input!(part_2, "3ba7923eae", Part::Two);
    }

}
