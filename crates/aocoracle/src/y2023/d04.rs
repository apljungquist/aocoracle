use hashbrown::HashSet;

pub fn part_1(input: &str) -> anyhow::Result<i32> {
    let mut sum = 0;
    for line in input.lines() {
        let (answers, guesses) = line.split_once('|').unwrap();
        let (title, answers) = answers.trim().split_once(':').unwrap();
        let answers: HashSet<u32> = answers.trim().split(' ').flat_map(|s|s.parse().ok()).collect();
        let guesses: Vec<u32> = guesses.trim().split(' ').flat_map(|s|s.parse().ok()).collect();
        let num_correct = guesses.iter().filter(|g| answers.contains(g)).count() as u32;
        if let Some(p) = num_correct.checked_sub(1) {
            let value = 2_i32.pow(p);
            sum += value;
        }
    }
    Ok(sum)
}

pub fn part_2(input: &str) -> anyhow::Result<u32> {
    let mut sum = 0;
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
