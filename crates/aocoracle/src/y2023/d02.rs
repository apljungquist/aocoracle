


pub fn part_1(input: &str) -> anyhow::Result<usize> {
    // 12 red cubes,
    // 13 green cubes, and
    // 14 blue cubes
    let mut sum = 0;
    for (i, line) in input.lines().enumerate(){
        let (g, draws) = line.split_once(':').unwrap();
        let mut plausible = true;
        for draw in draws.split(";") {
            for pair in  draw.split(",") {
                let pair = pair.trim();
                let (n, c) = pair.split_once(' ').unwrap();
                let n: u32 = n.trim().parse().unwrap();
                plausible &=  match c.trim() {
                    "red" => n <= 12,
                    "green" => n <= 13,
                    "blue" => n <= 14,
                    _ => panic!("{}", c),
                };
            }
        }
        if plausible {
            println!("{}", i);
            sum += i+1;
        }
    }
    Ok(sum)
}

pub fn part_2(input: &str) -> anyhow::Result<u32> {
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
