use std::fmt::{Debug, Formatter, Pointer};
use std::str::FromStr;

enum Signal {
    Integer(i32),
    List(Vec<Signal>),
}

impl Signal {
    fn take_signal(tokens: &mut Vec<&str>) -> anyhow::Result<Self> {
        let mut list = Vec::new();
        while let Some(token) = tokens.pop() {
            match token {
                "[" => {
                    list.push(Self::take_signal(tokens)?);
                }
                "]" => {
                    break;
                }
                "," => {}
                x => {
                    list.push(Self::Integer(x.parse()?));
                }
            }
        }
        Ok(Self::List(list))
    }
}

impl Debug for Signal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Signal::Integer(x) => write!(f, "{x}"),
            Signal::List(xs) => write!(f, "{xs:?}"),
        }
    }
}

impl FromStr for Signal {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s
            .replace('[', " [ ")
            .replace("]", " ] ")
            .replace(',', " , ");
        let mut tokens: Vec<_> = s.split_whitespace().rev().collect();
        assert_eq!(tokens.pop(), Some("["));
        Signal::take_signal(&mut tokens)
    }
}

fn pairs(s: &str) -> anyhow::Result<Vec<(Signal, Signal)>> {
    let mut result = Vec::new();
    for pair in s.split("\n\n") {
        let (left, right) = pair.split_once('\n').unwrap();
        let left = left.parse()?;
        let right = right.parse()?;
        result.push((left, right));
    }
    Ok(result)
}

fn in_order(left: &Signal, right: &Signal, indent: &String) -> Option<bool> {
    let mut new_indent = indent.clone();
    new_indent.extend("  ".chars());
    println!("{indent}- Compare {left:?} vs {right:?}");
    match (left, right) {
        (Signal::Integer(l), Signal::Integer(r)) => {
            if l < r {
                println!("{indent} - Left side is smaller, so the inputs are in the right order");
                Some(true)
            } else if r < l {
                println!(
                    "{indent} - Right side is smaller, so the inputs are not in the right order"
                );
                Some(false)
            } else {
                None
            }
        }
        (Signal::Integer(l), Signal::List(_)) => {
            in_order(&Signal::List(vec![Signal::Integer(*l)]), right, &new_indent)
        }
        (Signal::List(_), Signal::Integer(r)) => {
            in_order(left, &Signal::List(vec![Signal::Integer(*r)]), &new_indent)
        }
        (Signal::List(ls), Signal::List(rs)) => {
            for (l, r) in ls.iter().zip(rs) {
                if let Some(result) = in_order(l, r, &new_indent) {
                    return Some(result);
                }
            }
            if ls.len() < rs.len() {
                println!("{indent} - Left side ran out of items, so inputs are in the right order");
                Some(true)
            } else if rs.len() < ls.len() {
                println!(
                    "{indent} - Right side ran out of items, so inputs are not in the right order"
                );
                Some(false)
            } else {
                None
            }
        }
    }
}

pub fn part_1(input: &str) -> anyhow::Result<usize> {
    let pairs = pairs(input)?;
    let mut result = 0;
    for (i, (left, right)) in pairs.into_iter().enumerate() {
        println!("\n== Pair {} ==", i + 1);
        if in_order(&left, &right, &String::new()).unwrap() {
            result += i + 1
        }
    }
    Ok(result)
}

pub fn part_2(input: &str) -> anyhow::Result<usize> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use crate::testing::{assert_correct_answer_on_correct_input, assert_error_on_wrong_input};
    use crate::Part;

    use super::*;

    #[test]
    fn part_1_works_on_example() {
        assert_correct_answer_on_correct_input!(part_1, "example", Part::One);
    }

    #[test]
    fn part_1_works_on_input() {
        assert_correct_answer_on_correct_input!(part_1, "6bb0c0bd67", Part::One);
    }

    #[test]
    fn part_2_works_on_example() {
        assert_correct_answer_on_correct_input!(part_2, "example", Part::Two);
    }

    #[test]
    fn part_2_works_on_input() {
        assert_correct_answer_on_correct_input!(part_2, "6bb0c0bd67", Part::Two);
    }

    #[test]
    fn returns_error_on_wrong_input() {
        assert_error_on_wrong_input!(part_1, part_2);
    }
}
