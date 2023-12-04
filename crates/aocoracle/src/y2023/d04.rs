use hashbrown::{HashMap, HashSet};

fn parsed_cards(input: &str) -> anyhow::Result<HashMap<usize, u32>> {
    let mut cards = HashMap::with_capacity(input.lines().count());
    for (line_num, line) in input.lines().enumerate() {
        let (answers, guesses) = line.split_once('|').unwrap();
        let (title, answers) = answers.trim().split_once(':').unwrap();
        let answers: HashSet<u32> = answers
            .trim()
            .split(' ')
            .flat_map(|s| s.parse().ok())
            .collect();
        let guesses: Vec<u32> = guesses
            .trim()
            .split(' ')
            .flat_map(|s| s.parse().ok())
            .collect();
        let num_correct = guesses.iter().filter(|g| answers.contains(g)).count() as u32;
        cards.insert(line_num, num_correct);
    }
    Ok(cards)
}

pub fn part_1(input: &str) -> anyhow::Result<i32> {
    let cards = parsed_cards(input)?;
    let mut sum = 0;
    for num_correct in cards.values() {
        if let Some(p) = num_correct.checked_sub(1) {
            let value = 2_i32.pow(p);
            sum += value;
        }
    }
    Ok(sum)
}

fn card_value(cards: &HashMap<usize, u32>, curr: usize) -> u32 {
    let num_correct = *cards.get(&curr).unwrap();
    let first = curr + 1;
    let last = curr + num_correct as usize;
    let mut sum = 0;
    for next in first..=last {
        sum += card_value(cards, next);
    }
    sum + 1
}

pub fn part_2(input: &str) -> anyhow::Result<u32> {
    let cards = parsed_cards(input)?;
    let mut sum = 0;
    for (card_num, num_correct) in cards.iter() {
        let value = card_value(&cards, *card_num);
        println!("{value}");
        sum += value;
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
