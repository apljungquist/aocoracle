use std::str::FromStr;

use anyhow::anyhow;

#[derive(Clone, Copy, Debug)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn score(&self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
}

impl FromStr for Shape {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Self::Rock),
            "B" => Ok(Self::Paper),
            "C" => Ok(Self::Scissors),
            _ => Err(anyhow::anyhow!("Expected one of A,B,C but got {s}")),
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Outcome {
    Lose,
    Draw,
    Win,
}

impl Outcome {
    fn score(&self) -> u32 {
        match self {
            Outcome::Lose => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        }
    }
}

impl FromStr for Outcome {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Self::Lose),
            "Y" => Ok(Self::Draw),
            "Z" => Ok(Self::Win),
            _ => Err(anyhow::anyhow!("Expected one of X,Y,Z but got {s}")),
        }
    }
}

fn rounds(s: &str) -> anyhow::Result<Vec<(Shape, Outcome)>> {
    let re = regex::Regex::new(r"^([ABC]) ([XYZ])$").expect("Hard coded regex is valid");
    let mut result = Vec::new();
    for line in s.lines() {
        let cap = re
            .captures(line)
            .ok_or_else(|| anyhow!("Could not capture play on line {}", line))?;
        result.push((Shape::from_str(&cap[1])?, Outcome::from_str(&cap[2])?))
    }
    Ok(result)
}

fn hero_score_1(villain: Shape, hero: Outcome) -> u32 {
    let shape_score = match hero {
        Outcome::Lose => Shape::Rock,
        Outcome::Draw => Shape::Paper,
        Outcome::Win => Shape::Scissors,
    }
    .score();
    // TODO: Consider looking for a nice way to alias Outcome
    let outcome_score = match (villain, hero) {
        (Shape::Rock, Outcome::Lose) => Outcome::Draw,
        (Shape::Rock, Outcome::Draw) => Outcome::Win,
        (Shape::Rock, Outcome::Win) => Outcome::Lose,
        (Shape::Paper, Outcome::Lose) => Outcome::Lose,
        (Shape::Paper, Outcome::Draw) => Outcome::Draw,
        (Shape::Paper, Outcome::Win) => Outcome::Win,
        (Shape::Scissors, Outcome::Lose) => Outcome::Win,
        (Shape::Scissors, Outcome::Draw) => Outcome::Lose,
        (Shape::Scissors, Outcome::Win) => Outcome::Draw,
    }
    .score();
    shape_score + outcome_score
}

fn hero_score_2(villain: Shape, hero: Outcome) -> u32 {
    let outcome_score = hero.score();
    let shape_score = match (villain, hero) {
        (Shape::Rock, Outcome::Lose) => Shape::Scissors,
        (Shape::Rock, Outcome::Draw) => Shape::Rock,
        (Shape::Rock, Outcome::Win) => Shape::Paper,
        (Shape::Paper, Outcome::Lose) => Shape::Rock,
        (Shape::Paper, Outcome::Draw) => Shape::Paper,
        (Shape::Paper, Outcome::Win) => Shape::Scissors,
        (Shape::Scissors, Outcome::Lose) => Shape::Paper,
        (Shape::Scissors, Outcome::Draw) => Shape::Scissors,
        (Shape::Scissors, Outcome::Win) => Shape::Rock,
    }
    .score();
    shape_score + outcome_score
}

pub fn part_1(input: &str) -> anyhow::Result<u32> {
    // TODO: Consider implementing something like starmap to make this more compact
    Ok(rounds(input)?
        .into_iter()
        .map(|(v, h)| hero_score_1(v, h))
        .sum())
}

pub fn part_2(input: &str) -> anyhow::Result<u32> {
    Ok(rounds(input)?
        .into_iter()
        .map(|(v, h)| hero_score_2(v, h))
        .sum())
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
        assert_correct_answer_on_correct_input!("5edbf7131d1817c6", Part::One);
    }

    #[test]
    fn part_2_works_on_example() {
        assert_correct_answer_on_correct_input!("EXAMPLE", Part::Two);
    }

    #[test]
    fn part_2_works_on_input() {
        assert_correct_answer_on_correct_input!("5edbf7131d1817c6", Part::Two);
    }

    #[test]
    fn returns_error_on_wrong_input() {
        assert_error_on_wrong_input!(Part::One, Part::Two);
    }
}
