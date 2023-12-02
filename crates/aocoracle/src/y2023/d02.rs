use anyhow::bail;
use hashbrown::HashMap;
use std::str::FromStr;

#[derive(Eq, Hash, PartialEq)]
enum Color {
    Red,
    Green,
    Blue,
}

impl FromStr for Color {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "red" => Color::Red,
            "green" => Color::Green,
            "blue" => Color::Blue,
            c => bail!("Unexpected color {}", c),
        })
    }
}

fn parsed_draw(text: &str) -> anyhow::Result<HashMap<Color, usize>> {
    let mut retval = HashMap::new();
    for pair in text.split(',') {
        let (count, color) = pair
            .trim()
            .split_once(' ')
            .ok_or_else(||anyhow::anyhow!("Could not split pair {pair:?}"))?;
        let count = count.parse()?;
        let color = color.parse()?;
        retval.insert(color, count);
    }
    Ok(retval)
}
fn parsed_game(line: &str) -> anyhow::Result<(usize, Vec<HashMap<Color, usize>>)> {
    let (name, draws) = line
        .split_once(':')
        .ok_or_else(||anyhow::anyhow!("Could not split game {line:?}"))?;
    let (prefix, num) = name
        .trim()
        .split_once(' ')
        .ok_or_else(||anyhow::anyhow!("Could not split name {name:?}"))?;
    if prefix != "Game" {
        bail!("Expected line starting with 'Game' but gout {line:}");
    }
    let num = num.parse()?;

    let mut parsed_draws = Vec::new();
    for draw in draws.split(';') {
        parsed_draws.push(parsed_draw(draw)?);
    }

    Ok((num, parsed_draws))
}

pub fn part_1(input: &str) -> anyhow::Result<usize> {
    let mut sum = 0;
    'line_loop: for line in input.lines() {
        let (game_num, draws) = parsed_game(line)?;
        for draw in draws {
            for (color, count) in draw.into_iter() {
                let plausible = match color {
                    Color::Red => count <= 12,
                    Color::Green => count <= 13,
                    Color::Blue => count <= 14,
                };
                if !plausible {
                    continue 'line_loop;
                }
            }
        }
        sum += game_num;
    }
    Ok(sum)
}

pub fn part_2(input: &str) -> anyhow::Result<usize> {
    let mut sum = 0;
    for line in input.lines() {
        let (_, draws) = parsed_game(line)?;
        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;
        for draw in draws {
            for (color, count) in draw.into_iter() {
                match color {
                    Color::Red => min_red = min_red.max(count),
                    Color::Green => min_green = min_green.max(count),
                    Color::Blue => min_blue = min_blue.max(count),
                }
            }
        }
        let power = min_red * min_green * min_blue;
        sum += power;
    }
    Ok(sum)
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
