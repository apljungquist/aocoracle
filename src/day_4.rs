use std::collections::{HashMap, HashSet};
use std::fs;

fn _draws(input: &str) -> Vec<u32> {
    input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|v| v.parse::<u32>().unwrap())
        .collect()
}

fn _boards(input: &str) -> Vec<HashMap<(bool, usize), HashSet<u32>>> {
    let mut lines = input.lines();
    lines.next(); // Discard draws
    lines.collect::<Vec<&str>>().chunks(6).map(_board).collect()
}

fn _board(lines: &[&str]) -> HashMap<(bool, usize), HashSet<u32>> {
    let mut result = HashMap::with_capacity(25);
    for (row, line) in lines.iter().enumerate() {
        for (col, cell) in line.split_whitespace().enumerate() {
            result
                .entry((false, row))
                .or_insert_with(|| HashSet::with_capacity(5))
                .insert(cell.parse().unwrap());
            result
                .entry((true, col))
                .or_insert_with(|| HashSet::with_capacity(5))
                .insert(cell.parse().unwrap());
        }
    }
    assert_eq!(result.len(), 10);
    result
}

fn _cross(board: &mut HashMap<(bool, usize), HashSet<u32>>, draw: u32) {
    for candidate in board.values_mut() {
        candidate.remove(&draw);
    }
}

fn _bingo(board: &HashMap<(bool, usize), HashSet<u32>>) -> bool {
    board.values().any(|nums| nums.is_empty())
}

fn _score(board: &HashMap<(bool, usize), HashSet<u32>>) -> u32 {
    board
        .iter()
        .filter_map(|((is_row, _), row)| match is_row {
            false => Some(row.iter().sum::<u32>()),
            true => None,
        })
        .sum()
}

pub fn part_1(input: &str) -> u32 {
    let draws = _draws(input);
    let mut boards = _boards(input);
    for draw in draws.into_iter() {
        for board in boards.iter_mut() {
            _cross(board, draw);
            if _bingo(board) {
                return _score(board) * draw;
            }
        }
    }
    panic!("Reached end of function without finding an answer");
}

pub fn part_2(input: &str) -> u32 {
    let mut draws = _draws(input).into_iter();
    let mut boards = _boards(input);
    while boards.len() > 1 {
        let draw: u32 = draws.next().unwrap();
        for board in boards.iter_mut() {
            _cross(board, draw);
        }
        boards.retain(|b| !_bingo(b));
    }
    assert_eq!(boards.len(), 1);
    for draw in draws {
        for board in boards.iter_mut() {
            _cross(board, draw);
            if _bingo(board) {
                return _score(board) * draw;
            }
        }
    }
    panic!("Reached end of function without finding an answer");
}

fn _from_file<F, T>(func: F, stem: &str) -> T
where
    F: Fn(&str) -> T,
{
    func(&fs::read_to_string(format!("day/4/{}.txt", stem)).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_works_on_example() {
        assert_eq!(_from_file(part_1, "example"), 4512);
    }

    #[test]
    fn part_1_works_on_input() {
        assert_eq!(_from_file(part_1, "input"), 22680);
    }

    #[test]
    fn part_2_works_on_example() {
        assert_eq!(_from_file(part_2, "example"), 1924);
    }

    #[test]
    fn part_2_works_on_input() {
        assert_eq!(_from_file(part_2, "input"), 16168);
    }
}
