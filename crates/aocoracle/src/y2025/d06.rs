use hashbrown::HashMap;

pub fn part_1(input: &str) -> anyhow::Result<usize> {
    let mut numbers: HashMap<(usize, usize), usize> = HashMap::new();
    let mut ops = HashMap::new();
    let mut w = 0;
    let mut h = 0;
    for (i, line) in input.lines().enumerate() {
        for (j, num) in line.split_ascii_whitespace().enumerate() {
            if num == "*" || num == "+" {
                ops.insert(j, num);
            } else {
                h = h.max(i + 1);
                w = w.max(j + 1);
                numbers.insert((i, j), num.parse().unwrap());
            }
        }
    }

    let mut grand_total = 0;
    for j in 0..w {
        match *ops.get(&j).unwrap() {
            "*" => {
                let mut prod = 1;
                for i in 0..h {
                    prod *= numbers.get(&((i, j))).unwrap();
                }
                grand_total += prod;
            }
            "+" => {
                let mut sum = 0;
                for i in 0..h {
                    sum += numbers.get(&(i, j)).unwrap();
                }
                grand_total += sum;
            }
            _ => unreachable!(),
        }
    }

    Ok(grand_total)
}

pub fn part_2(input: &str) -> anyhow::Result<usize> {
    let mut transposed = Vec::new();
    for line in input.lines() {
        transposed.resize(transposed.len().max(line.len()), String::new());
        for (s, c) in transposed.iter_mut().zip(line.chars()) {
            s.push(c);
        }
    }

    transposed.reverse();

    let mut grand_total = 0;
    loop {
        let Some(line) = transposed.pop() else {
            break;
        };
        let op = line.trim().chars().last().unwrap();
        let n = line[..line.len() - 1].trim().parse::<usize>().unwrap();
        let mut nums = vec![n];

        while let Some(line) = transposed.pop() {
            if line.trim().is_empty() {
                break;
            }
            let n = line.trim().parse::<usize>().unwrap();
            nums.push(n);
        }

        match op {
            '*' => {
                let mut prod = 1;
                for n in nums {
                    prod *= n;
                }
                grand_total += prod;
            }
            '+' => {
                let mut sum = 0;
                for n in nums {
                    sum += n;
                }
                grand_total += sum;
            }
            _ => unreachable!("{:?}", op),
        }
    }

    Ok(grand_total)
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
        assert_correct_answer_on_correct_input!(part_1, "0a760efd5f01774a", Part::One);
    }

    #[test]
    fn part_2_works_on_example() {
        assert_correct_answer_on_correct_input!(part_2, "EXAMPLE", Part::Two);
    }

    #[test]
    fn part_2_works_on_input() {
        assert_correct_answer_on_correct_input!(part_2, "0a760efd5f01774a", Part::Two);
    }

    #[test]
    #[ignore]
    fn returns_error_on_wrong_input() {
        assert_error_on_wrong_input!(part_1, part_2);
    }
}
