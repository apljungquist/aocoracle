use anyhow::bail;
use hashbrown::HashSet;

const A: &str = "\
.##.
#..#
#..#
####
#..#
#..#
";

const B: &str = "\
###.
#..#
###.
#..#
#..#
###.
";

const C: &str = "\
.##.
#..#
#...
#...
#..#
.##.
";

const E: &str = "\
####
#...
###.
#...
#...
####
";

const F: &str = "\
####
#...
###.
#...
#...
#...
";

const G: &str = "\
.##.
#..#
#...
#.##
#..#
.###
";

const H: &str = "\
#..#
#..#
####
#..#
#..#
#..#
";

const J: &str = "\
..##
...#
...#
...#
#..#
.##.
";

const L: &str = "\
#...
#...
#...
#...
#...
####
";

const P: &str = "\
###.
#..#
#..#
###.
#...
#...
";

enum Op {
    Noop,
    AddX(i32),
}

fn ops(s: &str) -> anyhow::Result<Vec<Op>> {
    let mut result = Vec::new();
    for line in s.lines() {
        if line == "noop" {
            result.push(Op::Noop);
        } else if let Some((op, rhs)) = line.split_once(' ') {
            if op != "addx" {
                bail!("Expected addx but got {op}");
            }
            result.push(Op::AddX(rhs.parse()?));
        } else {
            bail!("Could not parse op from line {line}");
        }
    }
    Ok(result)
}

fn simulation(ops: &[Op]) -> Vec<i32> {
    let mut x = 1;
    let mut result = Vec::with_capacity(ops.len());
    for op in ops {
        match op {
            Op::Noop => result.push(x),
            Op::AddX(rhs) => {
                result.push(x);
                result.push(x);
                x += rhs;
            }
        }
    }
    result
}

fn letter_from_str(s: &str) -> anyhow::Result<HashSet<(i32, i32)>> {
    let mut result = HashSet::new();
    for (row, line) in s.lines().enumerate() {
        for (col, pixel) in line.chars().enumerate() {
            match pixel {
                '#' => {
                    result.insert((row as i32, col as i32));
                }
                '.' => {}
                _ => {
                    bail!("Expected pixel to be one of '#', '.' but got {pixel}");
                }
            }
        }
    }
    Ok(result)
}

fn letter_from_display(src: &HashSet<(i32, i32)>, index: i32) -> HashSet<(i32, i32)> {
    let mut result = HashSet::new();
    for dst_row in 0..6 {
        let src_row = dst_row;
        for dst_col in 0..4 {
            let src_col = 5 * index + dst_col;
            if src.contains(&(src_row, src_col)) {
                result.insert((dst_row, dst_col));
            }
        }
    }
    result
}

pub fn part_1(input: &str) -> anyhow::Result<usize> {
    let ops = ops(input)?;
    let cycles = simulation(&ops);
    Ok([20, 60, 100, 140, 180, 220]
        .iter()
        .map(|i| i * cycles[i - 1] as usize)
        .sum())
}

fn fmt_letter(letter: &HashSet<(i32, i32)>) -> String {
    let mut result = String::with_capacity(6 * 5);
    for row in 0..6 {
        for col in 0..4 {
            if letter.contains(&(row, col)) {
                result.push('#');
            } else {
                result.push('.');
            }
        }
        result.push('\n');
    }
    result
}

pub fn part_2(input: &str) -> anyhow::Result<String> {
    let ops = ops(input)?;
    let cycles = simulation(&ops);
    let letters = [
        ('A', letter_from_str(A)?),
        ('B', letter_from_str(B)?),
        ('C', letter_from_str(C)?),
        ('E', letter_from_str(E)?),
        ('F', letter_from_str(F)?),
        ('G', letter_from_str(G)?),
        ('H', letter_from_str(H)?),
        ('J', letter_from_str(J)?),
        ('L', letter_from_str(L)?),
        ('P', letter_from_str(P)?),
    ];

    let display: HashSet<_> = cycles
        .into_iter()
        .enumerate()
        .filter_map(|(i, x)| {
            let row = (i / 40) as i32;
            let col = (i % 40) as i32;
            if col == x - 1 || col == x || col == x + 1 {
                Some((row, col))
            } else {
                None
            }
        })
        .collect();

    let mut result = String::new();
    'outer: for i in 0..8 {
        let displayed = letter_from_display(&display, i);
        for (letter, reference) in letters.iter() {
            if &displayed == reference {
                result.push(*letter);
                continue 'outer;
            }
        }
        bail!(
            "Could not match the {}th letter\n{}",
            i,
            fmt_letter(&displayed)
        );
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use crate::testing::{assert_correct_answer_on_correct_input, assert_error_on_wrong_input};
    use crate::Part;

    use super::*;

    #[test]
    fn part_1_works_on_example() {
        assert_correct_answer_on_correct_input!("EXAMPLE", Part::One);
    }

    #[test]
    fn part_1_works_on_input() {
        assert_correct_answer_on_correct_input!("f3ebc778d81aafa4", Part::One);
    }

    #[test]
    fn part_2_works_on_input() {
        assert_correct_answer_on_correct_input!("f3ebc778d81aafa4", Part::Two);
    }

    #[test]
    fn part_2_works_on_09475985b08b8984() {
        assert_correct_answer_on_correct_input!("09475985b08b8984", Part::Two);
    }

    #[test]
    fn returns_error_on_wrong_input() {
        assert_error_on_wrong_input!(Part::One, Part::Two);
    }
}
