use anyhow::{bail, Context};
use hashbrown::{HashMap, HashSet};

/// Return a vector of the number of correct answers for each card.
///
/// Note that the cards are 0-indexed from this point.
fn parsed_cards(input: &str) -> anyhow::Result<Vec<u8>> {
    let mut cards = Vec::with_capacity(input.lines().count());
    for (i, line) in input.lines().enumerate() {
        let (answers, guesses) = line
            .split_once('|')
            .ok_or_else(|| anyhow::anyhow!("Expected exactly one '|' but got {line:?}"))?;
        let (title, answers) = answers
            .trim()
            .split_once(':')
            .ok_or_else(|| anyhow::anyhow!("Expected exactly one ':' but got {answers:?}"))?;
        let card_num = title.strip_prefix("Card ").ok_or_else(|| {
            anyhow::anyhow!("Expected title to start with 'Card ' but got {title:?}")
        })?;
        let card_num: usize = card_num
            .trim()
            .parse()
            .with_context(|| format!("{title:?}"))?;
        if card_num != i + 1 {
            bail!("Expected card number to be {i} but got {card_num}");
        }
        let answers: HashSet<usize> = answers
            .split_whitespace()
            .flat_map(|s| s.parse().ok())
            .collect();
        let guesses: Vec<usize> = guesses
            .split_whitespace()
            .flat_map(|s| s.parse().ok())
            .collect();
        let num_correct = guesses
            .iter()
            .filter(|g| answers.contains(g))
            .count()
            .try_into()?;
        cards.push(num_correct);
    }
    Ok(cards)
}

fn cards_count(cache: &mut HashMap<usize, usize>, cards: &[u8], curr: usize) -> usize {
    let num_correct = *cards.get(curr).unwrap();
    let first = curr + 1;
    let last = curr + num_correct as usize;

    let mut count = 1;
    for next in first..=last {
        if let Some(v) = cache.get(&next) {
            count += v;
        } else {
            count += cards_count(cache, cards, next);
        }
    }
    cache.insert(curr, count);
    count
}

pub fn part_1(input: &str) -> anyhow::Result<u32> {
    let cards = parsed_cards(input)?;

    let mut sum = 0;
    for num_correct in cards {
        if let Some(p) = num_correct.checked_sub(1) {
            sum += 2_u32.pow(p.into());
        }
    }
    Ok(sum)
}

pub fn part_2(input: &str) -> anyhow::Result<usize> {
    let mut cache = HashMap::new();
    let cards = parsed_cards(input)?;

    let mut sum = 0;
    for card_num in 0..cards.len() {
        let value = cards_count(&mut cache, &cards, card_num);
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
        assert_correct_answer_on_correct_input!(part_1, "a0bd9806241e64ed", Part::One);
    }

    #[test]
    fn part_2_works_on_example() {
        assert_correct_answer_on_correct_input!(part_2, "EXAMPLE", Part::Two);
    }

    #[test]
    fn part_2_works_on_input() {
        assert_correct_answer_on_correct_input!(part_2, "a0bd9806241e64ed", Part::Two);
    }

    #[test]
    fn returns_error_on_wrong_input() {
        assert_error_on_wrong_input!(part_1, part_2);
    }
}
