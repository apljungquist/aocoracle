pub fn part_1(input: &str) -> anyhow::Result<u64> {
    let mut count = 0;
    let mut pos = 50;
    for line in input.lines() {
        let dir = line.chars().next().unwrap();
        let mag = i64::from_str_radix(&line[1..], 10)?;
        match dir {
            'L' => pos -= mag,
            'R' => pos += mag,
            _ => panic!(),
        }
        while pos < 0 {
            pos += 100;
        }
        while pos >= 100 {
            pos -= 100;
        }
        if pos == 0 {
            count += 1;
        }
    }
    Ok(count)
}

pub fn part_2(input: &str) -> anyhow::Result<usize> {
    let mut count = 0;
    let mut pos = 50;
    for line in input.lines() {
        let dir = line.chars().next().unwrap();
        let mag = i64::from_str_radix(&line[1..], 10)?;
        for _ in 0..mag {
            match dir {
                'L' => pos -= 1,
                'R' => pos += 1,
                _ => panic!(),
            }
            while pos < 0 {
                pos += 100;
            }
            while pos >= 100 {
                pos -= 100;
            }
            if pos == 0 {
                count += 1;
            }
        }
        // match dir {
        //     'L' => pos -= mag,
        //     'R' => pos += mag,
        //     _ => panic!(),
        // }
        // while pos < 0 {
        //     pos += 100;
        //     if prev_pos != 0 && pos != 0{
        //         println!("click");
        //         count += 1;
        //     }
        // }
        // while pos >= 100 {
        //     pos -= 100;
        //     if prev_pos != 0 && pos != 0{
        //         println!("click");
        //         count += 1;
        //     }
        // }
        // if pos == 0 {
        //     println!("click");
        //     count += 1;
        // }
        // println!("{line}: {prev_pos} -> {pos}");
    }
    Ok(count)
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
        assert_correct_answer_on_correct_input!(part_1, "e17582decc1882ab", Part::One);
    }

    #[test]
    fn part_2_works_on_example() {
        assert_correct_answer_on_correct_input!(part_2, "EXAMPLE", Part::Two);
    }

    #[test]
    fn part_2_works_on_input() {
        assert_correct_answer_on_correct_input!(part_2, "e17582decc1882ab", Part::Two);
    }

    #[test]
    fn returns_error_on_wrong_input() {
        assert_error_on_wrong_input!(part_1, part_2);
    }
}
