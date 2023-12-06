use anyhow::bail;

fn line2num(line: &str) -> Option<u32> {
    let digits: Vec<_> = line.chars().filter(|c| c.is_numeric()).collect();
    Some(10 * digits.first()?.to_digit(10)? + digits.last()?.to_digit(10)?)
}

fn word2digit(word: &str) -> Option<u32> {
    match word {
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        d => d.parse().ok(),
    }
}

fn line2num2(line: &str) -> Option<u32> {
    let digit = "(one|two|three|four|five|six|seven|eight|nine|[0-9])";
    let first = regex::Regex::new(&format!("^.*?{digit}.*$")).expect("Hard coded regex is valid");
    let last = regex::Regex::new(&format!("^.*{digit}.*?$")).expect("Hard coded regex is valid");

    let first = &first.captures(line)?[1];
    let last = &last.captures(line)?[1];

    Some(10 * word2digit(first)? + word2digit(last)?)
}

fn validate_common(input: &str) -> anyhow::Result<()> {
    // Avoid y2015d10 which is just numbers
    if input.chars().filter(|&c| c != '\n').all(|c| c.is_numeric()) {
        bail!("contains only numbers")
    }
    if input
        .chars()
        .filter(|&c| c != '\n')
        .any(|c| !c.is_alphanumeric() || c.is_uppercase())
    {
        bail!("contains more than just numbers and lower case letters")
    }
    Ok(())
}

pub fn part_1(input: &str) -> anyhow::Result<u32> {
    validate_common(input)?;
    // The example don't contain digits written with letters
    let is_example = input.lines().count() == 4;
    if !is_example && !input.contains("eight") {
        bail!("contains no digits spelled out as words")
    }
    let mut sum = 0;
    for line in input.lines() {
        let Some(addend) = line2num(line) else {
            bail!("Could not parse line {line}");
        };
        sum += addend;
    }
    Ok(sum)
}

pub fn part_2(input: &str) -> anyhow::Result<u32> {
    validate_common(input)?;
    if !input.contains("eight") {
        bail!("contains no digits spelled out as words")
    }
    let mut sum = 0;
    for line in input.lines() {
        let Some(addend) = line2num2(line) else {
            bail!("Could not parse line {line}");
        };
        sum += addend;
    }
    Ok(sum)
}

#[cfg(test)]
mod tests {
    use crate::testing::{assert_correct_answer_on_correct_input, assert_error_on_wrong_input};
    use crate::Part;

    use super::*;

    #[test]
    fn line2num2_works() {
        assert_eq!(line2num2("eightwo0twone"), Some(81));
    }

    #[test]
    fn part_1_works_on_example() {
        assert_correct_answer_on_correct_input!("EXAMPLE", Part::One);
    }

    #[test]
    fn part_1_works_on_input() {
        assert_correct_answer_on_correct_input!("cdff074dd172ac1d", Part::One);
    }

    #[test]
    fn part_2_works_on_example() {
        assert_correct_answer_on_correct_input!("EXAMPLE2", Part::Two);
    }

    #[test]
    fn part_2_works_on_input() {
        assert_correct_answer_on_correct_input!("cdff074dd172ac1d", Part::Two);
    }

    #[test]
    fn returns_error_on_wrong_input() {
        assert_error_on_wrong_input!(Part::One, Part::Two);
    }
}
