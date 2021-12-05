use std::collections::{HashMap, HashSet};
use std::fs;
use std::hash::Hash;

fn _read_input(name: &str) -> String {
    fs::read_to_string(format!("day/3/{}", name)).unwrap()
}

fn _rows(text: &str) -> Vec<Vec<bool>> {
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
                .unwrap(),
            )
        }
        result.push(row);
    }
    result
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
        .map(|x| {
            println!("{}", x);
            x
        })
        .sum()
}

fn _gamma(cols: &Vec<Vec<bool>>) -> u32 {
    _from_bits(cols.iter().map(|vs| *_mode(vs.iter()).unwrap()).collect())
}

fn _epsilon(cols: &Vec<Vec<bool>>) -> u32 {
    _from_bits(cols.iter().map(|vs| !*_mode(vs.iter()).unwrap()).collect())
}

fn _oxygen(cols: &Vec<Vec<bool>>) -> u32 {
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

fn _carbon(cols: &Vec<Vec<bool>>) -> u32 {
    let mut rows: HashSet<usize> = (0..cols.iter().map(|c| c.len()).max().unwrap()).collect();
    println!("rows: {:?}", rows);
    for col in cols {
        // Since the value is boolean the least common value is whatever value is not the most common.
        // And since we negate the expression we do not need to negate the default value.
        let target = !*_mode(
            col.iter()
                .enumerate()
                .filter(|(i, _)| rows.contains(i))
                .map(|(_, v)| v)
                .map(|x| {
                    println!("{:?}", x);
                    x
                }),
        )
        .unwrap_or(&true);
        println!("target: {}", target);
        for (i, actual) in col.iter().enumerate() {
            if *actual != target {
                println!("removing {} {}", i, actual);
                rows.remove(&i);
            } else {
                println!("keeping {} {}", i, actual);
            }
        }
        println!("{:?}", rows);
        if rows.len() == 1 {
            break;
        }
    }
    assert_eq!(rows.len(), 1);
    let row = rows.drain().next().unwrap();
    _from_bits(cols.iter().map(|v| v[row]).collect())
}

fn _oxygen_from_file(filename: &str) -> u32 {
    let cols = _transposed(_rows(&_read_input(filename)));
    _oxygen(&cols)
}
fn _carbon_from_file(filename: &str) -> u32 {
    let cols = _transposed(_rows(&_read_input(filename)));
    _carbon(&cols)
}

fn part_1(filename: &str) -> u32 {
    let cols = _transposed(_rows(&_read_input(filename)));
    _gamma(&cols) * _epsilon(&cols)
}

fn part_2(filename: &str) -> u32 {
    let cols = _transposed(_rows(&_read_input(filename)));
    _oxygen(&cols) * _carbon(&cols)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_works_on_example() {
        assert_eq!(part_1("example.txt"), 198);
    }

    #[test]
    fn part_1_works_on_input() {
        assert_eq!(part_1("input.txt"), 2954600);
    }

    #[test]
    fn oxygen_works_on_example() {
        assert_eq!(_oxygen_from_file("example.txt"), 23);
    }
    #[test]
    fn carbon_works_on_example() {
        assert_eq!(_carbon_from_file("example.txt"), 10);
    }

    #[test]
    fn part_2_works_on_example() {
        assert_eq!(part_2("example.txt"), 230);
    }

    #[test]
    fn part_2_works_on_input() {
        assert_eq!(part_2("input.txt"), 1662846);
    }
}
