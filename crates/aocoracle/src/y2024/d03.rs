pub fn part_1(input: &str) -> anyhow::Result<u64> {
    // let mut input = input.to_string();
    let re = regex::Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut sum = 0;
    for cap in re.captures_iter(input.trim()) {
        let lhs:u64 = cap[1].parse()?;
        let rhs:u64 = cap[2].parse()?;
        sum += lhs * rhs;
    }
    Ok(sum)
}

pub fn part_2(input: &str) -> anyhow::Result<u64> {

    // let mut input = input.to_string();
    let mut active = true;
    let re = regex::Regex::new(r"(mul|do|don't)\(((\d{1,3}),(\d{1,3}))?\)").unwrap();
    let mut sum = 0;
    for cap in re.captures_iter(input.trim()) {

        match &cap[1] {
            "do"=> active = true,
            "don't"=> active = false,
            "mul"=>{
                if active {
                    let lhs:u64 = cap[3].parse()?;
                    let rhs:u64 = cap[4].parse()?;
                    sum += lhs * rhs;
                }
            }
            _=> panic!("{cap:?}")
        }
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
        // 242 is too low
    }

    #[test]
    fn returns_error_on_wrong_input() {
        assert_error_on_wrong_input!(part_1, part_2);
    }
}
