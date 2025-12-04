use hashbrown::HashMap;

fn parse(input: &str) -> HashMap<(usize, usize), String> {
    let mut grid = HashMap::new();
    for (i, line) in input.lines().enumerate() {
        for (j, ch) in line.chars().enumerate() {
            grid.insert((i, j), ch.to_string());
        }
    }
    grid
}

pub fn part_1(input: &str) -> anyhow::Result<usize> {
    let grid = parse(input);
    let h = *grid.keys().map(|(i, _)| i).max().unwrap();
    let w = *grid.keys().map(|(_, j)| j).max().unwrap();

    let mut lines = Vec::new();

    // Horizontal
    for i in 0..=h {
        let mut line = String::new();
        for j in 0..=w {
            line.push_str(grid.get(&(i, j)).unwrap());
        }
        lines.push(line);
    }
    // Diagonal tl -> br
    for i in 0..=h {
        let mut line = String::new();
        let mut j = 0;
        while let Some(ch) = grid.get(&(i + j, j)) {
            line.push_str(ch);
            j += 1;
        }
        lines.push(line);
    }
    for j in 1..=w {
        let mut line = String::new();
        let mut i = 0;
        while let Some(ch) = grid.get(&(i, j + i)) {
            line.push_str(ch);
            i += 1;
        }
        lines.push(line);
    }
    // Vertical
    for j in 0..=w {
        let mut line = String::new();
        for i in 0..=h {
            line.push_str(grid.get(&(i, j)).unwrap());
        }
        lines.push(line);
    }
    // Diagonal bl -> tr
    for i in 0..=h {
        let mut line = String::new();
        let mut j = 0;
        while let Some(ch) = grid.get(&(i.wrapping_sub(j), j)) {
            line.push_str(ch);
            j += 1;
        }
        lines.push(line);
    }
    for j in 1..=w {
        let mut line = String::new();
        let mut i = 0;
        while let Some(ch) = grid.get(&(h.wrapping_sub(i), j + i)) {
            line.push_str(ch);
            i += 1;
        }
        lines.push(line);
    }

    let forward = regex::Regex::new(r"XMAS").unwrap();
    let backward = regex::Regex::new(r"SAMX").unwrap();

    Ok(lines
        .into_iter()
        .map(|line| forward.captures_iter(&line).count() + backward.captures_iter(&line).count())
        .sum())
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
    let grid = parse(input);
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
        assert_correct_answer_on_correct_input!(part_1, "84e50060685ee1f0", Part::One);
    }

    #[test]
    fn part_2_works_on_example() {
        assert_correct_answer_on_correct_input!(part_2, "EXAMPLE", Part::Two);
    }

    #[test]
    fn part_2_works_on_input() {
        assert_correct_answer_on_correct_input!(part_2, "84e50060685ee1f0", Part::Two);
    }

    #[test]
    fn returns_error_on_wrong_input() {
        assert_error_on_wrong_input!(part_1, part_2);
    }
}
