use num::pow;
use std::collections::HashMap;
use std::mem::swap;

pub fn part_1(input: &str) -> anyhow::Result<u32> {
    let mut sparse = HashMap::new();
    let mut w = 0;
    let mut h = 0;
    for (y, line) in input.lines().enumerate() {
        let y = y as i32;
        h = h.max(y + 1);
        for (x, char) in line.chars().enumerate() {
            let x = x as i32;
            w = w.max(x + 1);
            if char == '@' {
                sparse.insert((x, y), ());
            }
        }
    }

    let mut sum = 0;
    for y in 0..w {
        for x in 0..h {
            if sparse.contains_key(&(x, y)) {
                let mut neighbor_count = 0;
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        if dx == 0 && dy == 0 {
                            continue;
                        }
                        if sparse.contains_key(&(x + dx, y + dy)) {
                            neighbor_count += 1;
                        }
                    }
                }
                if neighbor_count < 4 {
                    // print!("x");
                    sum += 1;
                } else {
                    // print!("@");
                }
            } else {
                // print!(".");
            }
        }
        // println!("");
    }
    Ok(sum)
}

pub fn part_2(input: &str) -> anyhow::Result<u64> {
    let mut sparse = HashMap::new();
    let mut w = 0;
    let mut h = 0;
    for (y, line) in input.lines().enumerate() {
        let y = y as i32;
        h = h.max(y + 1);
        for (x, char) in line.chars().enumerate() {
            let x = x as i32;
            w = w.max(x + 1);
            if char == '@' {
                sparse.insert((x, y), ());
            }
        }
    }

    let initial = sparse.len() as u64;
    loop {
        let mut should_break = true;
        for (x, y) in sparse.keys().into_iter().cloned().collect::<Vec<_>>() {
            let mut neighbor_count = 0;
            for dy in -1..=1 {
                for dx in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    if sparse.contains_key(&(x + dx, y + dy)) {
                        neighbor_count += 1;
                    }
                }
            }
            if neighbor_count < 4 {
                sparse.remove(&(x, y));
                should_break = false;
            }
        }
        if should_break {
            break;
        }
    }
    Ok(initial - sparse.len() as u64)
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
        assert_correct_answer_on_correct_input!(part_1, "52a52b44979fe307", Part::One);
    }

    #[test]
    fn part_2_works_on_example() {
        assert_correct_answer_on_correct_input!(part_2, "EXAMPLE", Part::Two);
    }

    #[test]
    fn part_2_works_on_input() {
        assert_correct_answer_on_correct_input!(part_2, "52a52b44979fe307", Part::Two);
    }

    #[test]
    fn returns_error_on_wrong_input() {
        assert_error_on_wrong_input!(part_1, part_2);
    }
}
