use std::collections::HashSet;

use crate::itersum::Itersum;
use crate::AnyError;

fn _rows(text: &str) -> Result<Vec<Vec<bool>>, AnyError> {
    let mut result = Vec::new();
    for line in text.lines() {
        let mut row = Vec::new();
        for bit in line.chars() {
            row.push(
                match bit {
                    '0' => Some(false),
                    '1' => Some(true),
                    _ => None,
                }
                .ok_or_else(|| format!("Could not match bit {}", bit))?,
            )
        }
        result.push(row);
    }
    Ok(result)
}

fn _transposed(rows: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut result = Vec::new();
    for _ in 0..rows.iter().map(|v| v.len()).max().unwrap() {
        result.push(Vec::new());
    }
    for row in rows.into_iter() {
        for (i, bit) in row.into_iter().enumerate() {
            result[i].push(bit)
        }
    }
    result
}

fn _from_bits(bits: Vec<bool>) -> u32 {
    bits.into_iter()
        .rev()
        .enumerate()
        .map(|(i, b)| if b { u32::pow(2, i as u32) } else { 0 })
        .sum()
}

fn _gamma(cols: &[Vec<bool>]) -> u32 {
    _from_bits(cols.iter().map(|vs| *vs.iter().mode().unwrap()).collect())
}

fn _epsilon(cols: &[Vec<bool>]) -> u32 {
    _from_bits(cols.iter().map(|vs| !*vs.iter().mode().unwrap()).collect())
}

fn _oxygen(cols: &[Vec<bool>]) -> u32 {
    let mut rows: HashSet<usize> = (0..cols.iter().map(|c| c.len()).max().unwrap()).collect();
    for col in cols {
        let target = *col
            .iter()
            .enumerate()
            .filter(|(i, _)| rows.contains(i))
            .map(|(_, v)| v)
            .mode()
            .unwrap_or(&true);
        for (i, actual) in col.iter().enumerate() {
            if *actual != target {
                rows.remove(&i);
            }
        }
        if rows.len() == 1 {
            break;
        }
    }
    assert_eq!(rows.len(), 1);
    let row = rows.drain().next().unwrap();
    _from_bits(cols.iter().map(|v| v[row]).collect())
}

fn _carbon(cols: &[Vec<bool>]) -> u32 {
    let mut rows: HashSet<usize> = (0..cols.iter().map(|c| c.len()).max().unwrap()).collect();
    for col in cols {
        // Since the value is boolean the least common value is whatever value is not the most common.
        // And since we negate the expression we do not need to negate the default value.
        let target = !*col
            .iter()
            .enumerate()
            .filter(|(i, _)| rows.contains(i))
            .map(|(_, v)| v)
            .mode()
            .unwrap_or(&true);
        for (i, actual) in col.iter().enumerate() {
            if *actual != target {
                rows.remove(&i);
            } else {
            }
        }
        if rows.len() == 1 {
            break;
        }
    }
    assert_eq!(rows.len(), 1);
    let row = rows.drain().next().unwrap();
    _from_bits(cols.iter().map(|v| v[row]).collect())
}

pub fn part_1(input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let cols = _transposed(_rows(input)?);
    Ok(format!("{}", _gamma(&cols) * _epsilon(&cols)))
}

pub fn part_2(input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let cols = _transposed(_rows(input)?);
    Ok(format!("{}", _oxygen(&cols) * _carbon(&cols)))
}

fn _oxygen_only(input: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let cols = _transposed(_rows(input)?);
    Ok(_oxygen(&cols))
}

fn _carbon_only(input: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let cols = _transposed(_rows(input)?);
    Ok(_carbon(&cols))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::{actual_answer, assert_returns_error_on_wrong_input, expected_answer};
    use crate::Part;

    fn assert_correct_answer(part: Part, stem: &str) {
        assert_eq!(
            actual_answer(
                file!(),
                match part {
                    Part::One => part_1,
                    Part::Two => part_2,
                },
                stem,
            ),
            expected_answer(file!(), part, stem).unwrap(),
        )
    }

    #[test]
    fn part_1_works_on_example() {
        assert_correct_answer(Part::One, "example");
    }

    #[test]
    fn part_1_works_on_input() {
        assert_correct_answer(Part::One, "6bb0c0bd67");
    }

    #[test]
    fn part_2_works_on_example() {
        assert_correct_answer(Part::Two, "example");
    }

    #[test]
    fn part_2_works_on_input() {
        assert_correct_answer(Part::Two, "6bb0c0bd67");
    }

    #[test]
    fn oxygen_works_on_example() {
        assert_eq!(actual_answer(file!(), _oxygen_only, "example"), 23);
    }

    #[test]
    fn carbon_works_on_example() {
        assert_eq!(actual_answer(file!(), _carbon_only, "example"), 10);
    }

    #[ignore]
    #[test]
    fn returns_error_on_wrong_input() {
        assert_returns_error_on_wrong_input(file!(), &part_1, &part_2);
    }
}
