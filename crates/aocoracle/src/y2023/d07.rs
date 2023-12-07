use anyhow::bail;

type Hand = [u8; 5];

fn strength(hand: &Hand) -> [u8; 6] {
    let mut card2count = [0; 15];
    for &card in hand {
        card2count[card as usize] += 1;
    }
    let num_joker = card2count[0];

    let mut result = [0; 6];
    result[0] = if let Some(best) = card2count[1..].iter_mut().max() {
        *best += num_joker;
        let mut count2count = [0; 6];
        for &count in &card2count[1..] {
            count2count[count] += 1;
        }
        if count2count[5] > 0 {
            6
        } else if count2count[4] > 0 {
            5
        } else if count2count[3] > 0 && count2count[2] > 0 {
            4
        } else if count2count[3] > 0 {
            3
        } else if count2count[2] == 2 {
            2
        } else {
            (count2count[2] > 0) as u8
        }
    } else {
        6
    };
    result[1..].copy_from_slice(hand);
    result
}

fn parsed_card(c: u8, jack: u8) -> anyhow::Result<u8> {
    Ok(match c {
        b'2'..=b'9' => c - b'0',
        b'T' => 10,
        b'J' => jack,
        b'Q' => 12,
        b'K' => 13,
        b'A' => 14,
        _ => bail!("Unexpected card {:?}", char::from(c)),
    })
}

fn parsed_hand(hand: &str, jack: u8) -> anyhow::Result<Hand> {
    let hand = hand.as_bytes();
    if hand.len() != 5 {
        bail!("Expected 5 cards but got {:?}", hand);
    }
    let mut result = [0; 5];
    for (i, c) in hand.iter().enumerate() {
        result[i] = parsed_card(*c, jack)?;
    }
    Ok(result)
}

fn parsed_input(input: &str, j: u8) -> anyhow::Result<Vec<(Hand, usize)>> {
    let mut result = Vec::new();
    for line in input.lines() {
        let Some((hand, bid)) = line.split_once(' ') else {
            bail!("Expected a space in line but got {:?}", line);
        };
        let bid = bid.parse()?;
        result.push((parsed_hand(hand, j)?, bid));
    }
    Ok(result)
}

fn total_winnings(mut input: Vec<(Hand, usize)>) -> usize {
    input.sort_by_key(|(h, _)| strength(h));
    input
        .into_iter()
        .enumerate()
        .map(|(i, (_, bid))| bid * (i + 1))
        .sum()
}

pub fn part_1(input: &str) -> anyhow::Result<usize> {
    let input = parsed_input(input, 11)?;
    Ok(total_winnings(input))
}

pub fn part_2(input: &str) -> anyhow::Result<usize> {
    let input = parsed_input(input, 0)?;
    Ok(total_winnings(input))
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
        assert_correct_answer_on_correct_input!(part_1, "00fd4b4ba9668e83", Part::One);
        // > 253652923
    }

    #[test]
    fn part_2_works_on_example() {
        assert_correct_answer_on_correct_input!(part_2, "EXAMPLE", Part::Two);
    }

    #[test]
    fn part_2_works_on_input() {
        assert_correct_answer_on_correct_input!(part_2, "00fd4b4ba9668e83", Part::Two);
    }

    #[test]
    fn returns_error_on_wrong_input() {
        assert_error_on_wrong_input!(part_1, part_2);
    }
}
