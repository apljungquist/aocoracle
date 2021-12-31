use std::collections::{HashMap, HashSet};
use std::fs;

type AnyError = Box<dyn std::error::Error>;
type Board = HashMap<(bool, usize), HashSet<u32>>;

fn _draws(input: &str) -> Result<Vec<u32>, AnyError> {
    let mut result = Vec::new();
    for v in input
        .lines()
        .next()
        .ok_or("Expected at least 1 line of input, got 0")?
        .split(',')
    {
        result.push(v.parse()?);
    }
    Ok(result)
}

fn _boards(input: &str) -> Result<Vec<Board>, AnyError> {
    let mut lines = input.lines();
    lines.next(); // Discard draws
    let mut result = Vec::new();
    for chunk in lines.collect::<Vec<&str>>().chunks(6) {
        result.push(_board(chunk)?);
    }
    if result.is_empty() {
        return Err(format!("Expected at least 1 board but got {}", result.len()).into());
    }
    Ok(result)
}

fn _board(lines: &[&str]) -> Result<Board, AnyError> {
    let mut result = HashMap::with_capacity(25);
    for (row, line) in lines.iter().enumerate() {
        for (col, cell) in line.split_whitespace().enumerate() {
            result
                .entry((false, row))
                .or_insert_with(|| HashSet::with_capacity(5))
                .insert(cell.parse()?);
            result
                .entry((true, col))
                .or_insert_with(|| HashSet::with_capacity(5))
                .insert(cell.parse()?);
        }
    }
    if result.len() != 10 {
        return Err(format!("Expected 10 rows or columns (5 of each) {}", result.len()).into());
    }
    assert_eq!(result.len(), 10);
    Ok(result)
}

fn _cross(board: &mut Board, draw: u32) {
    for candidate in board.values_mut() {
        candidate.remove(&draw);
    }
}

fn _bingo(board: &Board) -> bool {
    board.values().any(|nums| nums.is_empty())
}

fn _score(board: &Board) -> u32 {
    board
        .iter()
        .filter_map(|((is_row, _), row)| match is_row {
            false => Some(row.iter().sum::<u32>()),
            true => None,
        })
        .sum()
}

pub fn part_1(input: &str) -> Result<String, AnyError> {
    let draws = _draws(input)?;
    let mut boards = _boards(input)?;
    for draw in draws.into_iter() {
        for board in boards.iter_mut() {
            _cross(board, draw);
            if _bingo(board) {
                return Ok(format!("{}", _score(board) * draw));
            }
        }
    }
    panic!("Reached end of function without finding an answer");
}

pub fn part_2(input: &str) -> Result<String, AnyError> {
    let mut draws = _draws(input)?.into_iter();
    let mut boards = _boards(input)?;
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
                return Ok(format!("{}", _score(board) * draw));
            }
        }
    }
    panic!("Reached end of function without finding an answer");
}

fn _from_file<F, T>(func: F, stem: &str) -> T
where
    F: Fn(&str) -> Result<T, AnyError>,
{
    func(&fs::read_to_string(format!("inputs/04/{}.txt", stem)).unwrap()).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_works_on_example() {
        assert_eq!(_from_file(part_1, "example"), "4512");
    }

    #[test]
    fn part_1_works_on_input() {
        assert_eq!(_from_file(part_1, "input"), "22680");
    }

    #[test]
    fn part_2_works_on_example() {
        assert_eq!(_from_file(part_2, "example"), "1924");
    }

    #[test]
    fn part_2_works_on_input() {
        assert_eq!(_from_file(part_2, "input"), "16168");
    }
}
