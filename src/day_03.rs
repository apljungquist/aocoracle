use std::collections::{HashMap, HashSet};
use std::fs;
use std::hash::Hash;
type AnyError = Box<dyn std::error::Error>;

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

#[derive(Debug)]
enum AggregationError {
    TooFew,
    TooMany,
}

fn _mode<T: Copy + Eq + Hash>(values: impl Iterator<Item = T>) -> Result<T, AggregationError> {
    let mut counts = HashMap::new();
    for v in values {
        let count = counts.entry(v).or_insert(0);
        // This bothers me; surely I should not be able to increment an immutable value...
        *count += 1;
    }
    let best = counts.iter().max_by_key(|&(_, count)| count);
    let best = match best {
        Some(v) => v,
        None => return Err(AggregationError::TooFew),
    };

    for (k, v) in counts.iter() {
        if best.0 != k && best.1 == v {
            return Err(AggregationError::TooMany);
        }
    }

    Ok(*best.0)
}

fn _from_bits(bits: Vec<bool>) -> u32 {
    bits.into_iter()
        .rev()
        .enumerate()
        .map(|(i, b)| if b { u32::pow(2, i as u32) } else { 0 })
        .sum()
}

fn _gamma(cols: &[Vec<bool>]) -> u32 {
    _from_bits(cols.iter().map(|vs| *_mode(vs.iter()).unwrap()).collect())
}

fn _epsilon(cols: &[Vec<bool>]) -> u32 {
    _from_bits(cols.iter().map(|vs| !*_mode(vs.iter()).unwrap()).collect())
}

fn _oxygen(cols: &[Vec<bool>]) -> u32 {
    let mut rows: HashSet<usize> = (0..cols.iter().map(|c| c.len()).max().unwrap()).collect();
    for col in cols {
        let target = *_mode(
            col.iter()
                .enumerate()
                .filter(|(i, _)| rows.contains(i))
                .map(|(_, v)| v),
        )
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
        let target = !*_mode(
            col.iter()
                .enumerate()
                .filter(|(i, _)| rows.contains(i))
                .map(|(_, v)| v),
        )
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
fn _from_file<F, T>(func: F, stem: &str) -> T
where
    F: Fn(&str) -> Result<T, Box<dyn std::error::Error>>,
{
    func(&fs::read_to_string(format!("inputs/03/{}.txt", stem)).unwrap()).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_works_on_example() {
        assert_eq!(_from_file(part_1, "example"), "198");
    }

    #[test]
    fn part_1_works_on_input() {
        assert_eq!(_from_file(part_1, "input"), "2954600");
    }

    #[test]
    fn oxygen_works_on_example() {
        assert_eq!(_from_file(_oxygen_only, "example"), 23);
    }
    #[test]
    fn carbon_works_on_example() {
        assert_eq!(_from_file(_carbon_only, "example"), 10);
    }

    #[test]
    fn part_2_works_on_example() {
        assert_eq!(_from_file(part_2, "example"), "230");
    }

    #[test]
    fn part_2_works_on_input() {
        assert_eq!(_from_file(part_2, "input"), "1662846");
    }
}
