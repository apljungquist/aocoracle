use anyhow::{anyhow, bail};

fn checked_pow_mul_add(base: i64, exp: u32, mul_by: i64, add_to: i64) -> Option<i64> {
    base.checked_pow(exp)?
        .checked_mul(mul_by)?
        .checked_add(add_to)
}

trait Snafu: Sized {
    type Err;

    fn from_snafu(s: &str) -> Result<Self, Self::Err>;
    fn to_snafu(&self) -> String;
}

impl Snafu for i64 {
    type Err = anyhow::Error;

    fn from_snafu(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with('0') {
            bail!("Expected snafu number without leading zeros but got {s}");
        }
        let mut result = 0;
        for (i, ch) in s.chars().rev().enumerate() {
            result = checked_pow_mul_add(
                5,
                i as u32,
                match ch {
                    '2' => 2,
                    '1' => 1,
                    '0' => 0,
                    '-' => -1,
                    '=' => -2,
                    _ => bail!("Expected one of '2', '1', '0'', '-', or '=' but got {ch}"),
                },
                result,
            )
            .ok_or_else(|| anyhow!("Overflow"))?;
        }
        Ok(result)
    }

    fn to_snafu(&self) -> String {
        let mut rem = *self;
        let mut result = Vec::new();
        while rem != 0 {
            result.push(match rem % 5 {
                0 => '0',
                1 => '1',
                2 => '2',
                3 => {
                    rem += 2;
                    '='
                }
                4 => {
                    rem += 1;
                    '-'
                }
                _ => unreachable!(),
            });
            rem /= 5;
        }
        result.reverse();
        result.iter().collect()
    }
}

fn fuel_requirements(s: &str) -> anyhow::Result<Vec<i64>> {
    let mut result = Vec::new();
    for line in s.lines() {
        result.push(i64::from_snafu(line)?);
    }
    Ok(result)
}

pub fn part_1(input: &str) -> anyhow::Result<String> {
    let fuel_requirements = fuel_requirements(input)?;
    Ok(fuel_requirements.iter().sum::<i64>().to_snafu())
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
    fn returns_error_on_wrong_input() {
        assert_error_on_wrong_input!(part_1);
    }

    #[test]
    fn snafu_conversion() {
        for expected in [1, 2, 3, 4, 5, 6, 7, 8, 9, 34061028947237] {
            assert_eq!(i64::from_snafu(&expected.to_snafu()).unwrap(), expected);
        }
    }
}
