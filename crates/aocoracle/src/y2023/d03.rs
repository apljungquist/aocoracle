use hashbrown::{HashMap, HashSet};
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Number {
    value: usize,
    id: usize,
}

pub fn part_1(input: &str) -> anyhow::Result<usize> {
    let re = regex::Regex::new(&format!("([0-9]+|[^0-9])")).expect("Hard coded regex is valid");
    ;

    let mut next_id = 0;
    let mut numbers = HashMap::new();
    let mut symbols = HashMap::new();

    for (row_num, line) in input.lines().enumerate() {
        let row_num = row_num as isize;

        let mut col_num: isize = 0;
        for token in re.captures_iter(line) {
            let token = token.get(1).unwrap().as_str();

            if let Ok(number) = token.parse() {
                for _ in 0..token.len() {
                    numbers.insert((row_num, col_num), Number { id: next_id, value: number });
                    col_num += 1;
                }
                next_id += 1;
            } else if token != "." {
                symbols.insert((row_num, col_num), token.to_string());
                col_num += 1;
            } else {
                col_num += 1;
            }
        }
    }

    let mut used_numbers = HashSet::new();

    // for ((r,c), s) in symbols.iter().sorted() {
    //     println!("{r}, {c}: {s}");
    // }
    // for ((r,c), s) in numbers.iter().sorted() {
    //     println!("{r}, {c}: {}({})", s.value, s.id);
    // }

    let mut sum = 0;
    for ((r, c), s) in symbols.iter().sorted() {
        //         println!("{s} @ {r}, {c}");
        for dr in -1..=1 {
            for dc in -1..=1 {
                if dr == 0 && dc == 0 {
                    continue;
                }
                let row = r + dr;
                let col = c + dc;
                if let Some(number) = numbers.get(&(row, col)) {
                    if used_numbers.insert(number.id) {
                        // println!("{} @ {row}, {col}", number.value);
                        sum += number.value;
                    }
                }
            }
        }
    }
    Ok(sum)
}

pub fn part_2(input: &str) -> anyhow::Result<usize> {

    let re = regex::Regex::new(&format!("([0-9]+|[^0-9])")).expect("Hard coded regex is valid");
    ;

    let mut next_id = 0;
    let mut numbers = HashMap::new();
    let mut symbols = HashMap::new();

    for (row_num, line) in input.lines().enumerate() {
        let row_num = row_num as isize;

        let mut col_num: isize = 0;
        for token in re.captures_iter(line) {
            let token = token.get(1).unwrap().as_str();

            if let Ok(number) = token.parse() {
                for _ in 0..token.len() {
                    numbers.insert((row_num, col_num), Number { id: next_id, value: number });
                    col_num += 1;
                }
                next_id += 1;
            } else if token != "." {
                symbols.insert((row_num, col_num), token.to_string());
                col_num += 1;
            } else {
                col_num += 1;
            }
        }
    }


    let mut sum = 0;
    for ((r, c), s) in symbols.into_iter() {
        if s != "*" {
            continue;
        }
        let mut used_numbers = HashSet::new();
        let mut adjacent_numbers = Vec::new();
        for dr in -1..=1 {
            for dc in -1..=1 {
                if dr == 0 && dc == 0 {
                    continue;
                }
                let row = r + dr;
                let col = c + dc;
                if let Some(number) = numbers.get(&(row, col)) {
                    if used_numbers.insert(number.id) {
                        adjacent_numbers.push(number);
                    }
                }
            }
        }
        if adjacent_numbers.len() == 2 {
            sum += adjacent_numbers[0].value * adjacent_numbers[1].value
        }
    }
    Ok(sum)
}

#[cfg(test)]
mod tests {
    use crate::Part;
    use crate::testing::{assert_correct_answer_on_correct_input, assert_error_on_wrong_input};

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
