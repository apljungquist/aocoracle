use anyhow::Context;
use hashbrown::HashMap;

fn parse(input: &str) -> anyhow::Result<Vec<()>> {
    todo!()
}

// A0 B0 C0 D0 E0 F0
// A1 B1 C1 D1 E1 F1
// A2 B2 C2 D2 E2 F2
// A3 B3 C3 D3 E3 F3
// A4 B4 C4 D4 E4 F4

pub fn part_1(input: &str) -> anyhow::Result<usize> {
    let mut grid = HashMap::new();
    for (i, line) in input.lines().enumerate() {
        for (j, ch) in line.chars().enumerate() {
            grid.insert((i, j), ch.to_string());
            // grid.insert((i, j), format!("{}{i} ", char::from(b'A' + j as u8)));
        }
    }
    let h = *grid.keys().map(|(i, _)| i).max().unwrap();
    let w = *grid.keys().map(|(_, j)| j).max().unwrap();
    dbg!(h, w);

    let re = regex::Regex::new(r"XMAS").unwrap();
    let mut counts = Vec::new();

    dbg!("horizontal");
    for i in 0..=h {
        let mut row = String::new();
        for j in 0..=w {
            row.push_str(grid.get(&(i, j)).unwrap());
        }
        counts.push(dbg!(re.captures_iter(dbg!(&row)).count()));
        let rev = row.chars().rev().collect::<String>();
        counts.push(dbg!(re.captures_iter(dbg!(&rev)).count()));
    }

    dbg!("tl br");
    for i in 0..=h {
        let mut row = String::new();
        let mut j = 0;
        while let Some(ch) = grid.get(&(i + j, j)) {
            row.push_str(ch);
            j += 1;
        }
        counts.push(dbg!(re.captures_iter(dbg!(&row)).count()));
        let rev = row.chars().rev().collect::<String>();
        counts.push(dbg!(re.captures_iter(dbg!(&rev)).count()));
    }
    dbg!("tl br 2");
    for j in 1..=w {
        let mut row = String::new();
        let mut i = 0;
        while let Some(ch) = grid.get(&(i, j + i)) {
            row.push_str(ch);
            i += 1;
        }
        counts.push(dbg!(re.captures_iter(dbg!(&row)).count()));
        let rev = row.chars().rev().collect::<String>();
        counts.push(dbg!(re.captures_iter(dbg!(&rev)).count()));
    }

    dbg!("vertical");
    for j in 0..=w {
        let mut row = String::new();
        for i in 0..=h {
            row.push_str(grid.get(&(i, j)).unwrap());
        }
        counts.push(dbg!(re.captures_iter(dbg!(&row)).count()));
        let rev = row.chars().rev().collect::<String>();
        counts.push(dbg!(re.captures_iter(dbg!(&rev)).count()));
    }

    dbg!("tr bl");
    for i in 0..=h {
        let mut row = String::new();
        let mut j = 0;
        while let Some(ch) = grid.get(&(i.wrapping_sub(j), j)) {
            row.push_str(ch);
            j += 1;
        }
        counts.push(dbg!(re.captures_iter(dbg!(&row)).count()));
        let rev = row.chars().rev().collect::<String>();
        counts.push(dbg!(re.captures_iter(dbg!(&rev)).count()));
    }
    dbg!("tr bl 2");
    for j in 1..=w {
        let mut row = String::new();
        let mut i = 0;
        while let Some(ch) = grid.get(&(h.wrapping_sub(i), j + i)) {
            row.push_str(ch);
            i += 1;
        }
        counts.push(dbg!(re.captures_iter(dbg!(&row)).count()));
        let rev = row.chars().rev().collect::<String>();
        counts.push(dbg!(re.captures_iter(dbg!(&rev)).count()));
    }

    Ok(counts.into_iter().sum())
}

fn is_mas(l: &str, c: &str, r: &str) -> bool {
    (l == "M" && c == "A" && r == "S") || (l == "S" && c == "A" && r == "M")
}
fn is_xmas(grid: &HashMap<(usize, usize), String>, i: usize, j: usize) -> bool {
    let Some(c) = grid.get(&(i + 1, j + 1)) else {
        return false;
    };
    let Some(tl) = grid.get(&(i, j)) else {
        return false;
    };
    let Some(tr) = grid.get(&(i, j + 2)) else {
        return false;
    };
    let Some(bl) = grid.get(&(i + 2, j)) else {
        return false;
    };
    let Some(br) = grid.get(&(i + 2, j + 2)) else {
        return false;
    };
    is_mas(tl, c, br) && is_mas(tr, c, bl)
}
pub fn part_2(input: &str) -> anyhow::Result<usize> {
    let mut grid = HashMap::new();
    for (i, line) in input.lines().enumerate() {
        for (j, ch) in line.chars().enumerate() {
            grid.insert((i, j), ch.to_string());
            // grid.insert((i, j), format!("{}{i} ", char::from(b'A' + j as u8)));
        }
    }
    let h = *grid.keys().map(|(i, _)| i).max().unwrap();
    let w = *grid.keys().map(|(_, j)| j).max().unwrap();

    let mut count = 0;
    for i in 0..=h {
        for j in 0..=w {
            if is_xmas(&grid, i, j) {
                count += 1;
            }
        }
    }

    Ok(count)
}

#[cfg(test)]
mod tests {
    use crate::testing::{assert_correct_answer_on_correct_input, assert_error_on_wrong_input};
    use crate::Part;

    use super::*;
    #[test]
    fn part_1_works_on_example_small() {
        assert_correct_answer_on_correct_input!(part_1, "EXAMPLE-S", Part::One);
    }

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
