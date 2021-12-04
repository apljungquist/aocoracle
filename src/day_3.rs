use std::collections::HashSet;
use std::fs;

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

fn _gamma(cols: &Vec<Vec<bool>>) -> u32 {
    cols.iter()
        .map(|v| v.len() / 2 < v.iter().filter(|b| **b).count())
        .rev()
        .enumerate()
        .map(|(i, b)| if b { u32::pow(2, i as u32) } else { 0 })
        .sum()
}

fn _epsilon(cols: &Vec<Vec<bool>>) -> u32 {
    cols.iter()
        .map(|v| v.len() / 2 > v.iter().filter(|b| **b).count())
        .rev()
        .enumerate()
        .map(|(i, b)| if b { u32::pow(2, i as u32) } else { 0 })
        .sum()
}

fn _carbon(cols: &Vec<Vec<bool>>) -> u32 {
    let mut rows: HashSet<usize> = (0..cols.len()).collect();
    for col in cols {
        let target: bool = rows.len() / 2
            < col
                .iter()
                .enumerate()
                .filter(|(i, _)| rows.contains(i))
                .count();
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
    cols.iter()
        .map(|v| v[row])
        .rev()
        .enumerate()
        .map(|(i, b)| if b { u32::pow(2, i as u32) } else { 0 })
        .sum()
}

fn part_1(filename: &str) -> u32 {
    let cols = _transposed(_rows(&_read_input(filename)));
    _gamma(&cols) * _epsilon(&cols)
}

fn part_2(filename: &str) -> u32 {
    let cols = _transposed(_rows(&_read_input(filename)));
    _carbon(&cols)
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
    fn part_2_works_on_example() {
        assert_eq!(part_2("example.txt"), 230);
    }

    #[test]
    fn part_2_works_on_input() {
        assert_eq!(part_2("input.txt"), 1662846);
    }
}
