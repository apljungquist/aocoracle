use itertools::Itertools;

fn parsed_card(c: char) -> u8 {
    match c {
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => c.to_digit(10).unwrap() as u8,
    }
}
fn sort_key(orig_hand: &str) -> (u8, u8, u8, u8, u8, u8) {
    let hand = orig_hand.chars().map(parsed_card).collect::<Vec<_>>();
    let counts = hand.iter().counts().into_iter().map(|(_, v)| v).counts();
    let kind = if counts.contains_key(&4) {
        5
    } else if counts.contains_key(&3) && counts.contains_key(&2) {
        4
    } else if counts.contains_key(&3) {
        3
    } else if counts.get(&2) == Some(&2) {
        2
    } else if counts.contains_key(&2) {
        1
    } else {
        0
    };
    // dbg!((orig_hand, kind));
    (kind, hand[0], hand[1], hand[2], hand[3], hand[4])
}
pub fn part_1(input: &str) -> anyhow::Result<usize> {
    let mut bids = Vec::new();
    for line in input.lines() {
        let (hand, bid) = line.split_once(' ').unwrap();
        let bid = bid.parse::<usize>()?;
        bids.push((sort_key(hand), bid));
    }
    bids.sort_by_key(|(k, _)| *k);
    let mut sum = 0;
    for (i, (_, bid)) in bids.into_iter().enumerate() {
        let rank = i + 1;
        sum += bid * rank;
    }
    Ok(sum)
}

pub fn part_2(input: &str) -> anyhow::Result<usize> {
    Ok(0)
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
        // > 253652923
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
