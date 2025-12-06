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
        match ops.get(&j).unwrap() {
            &"*" => {
                let mut prod = 1;
                for i in 0..h {
                    prod *= (numbers.get(&((i, j))).unwrap());
                }
                grand_total += prod;
            }
            &"+" => {
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
    let mut transposed: HashMap<usize, String> = HashMap::new();
    for line in input.lines() {
        for (i, c) in line.chars().enumerate() {
            transposed.entry(i).or_default().push(c);
        }
    }

    for i in 0..transposed.len() {
        println!("{}", transposed.get(&i).unwrap());
    }

    let mut grand_total = 0;
    let mut j = 0;
    loop {
        let Some(line) = transposed.remove(&j) else {
            break;
        };
        let op = dbg!(line.trim().chars().last().unwrap());
        let n = dbg!(line[..line.len() - 1].trim())
            .parse::<usize>()
            .unwrap();
        let mut nums = vec![n];
        j += 1;
        while let Some(line) = transposed.remove(&j) {
            println!("{line:?}");
            if line.trim().is_empty() {
                println!("break");
                break;
            }
            let n = dbg!(line.trim()).parse::<usize>().unwrap();
            nums.push(n);
            j += 1;
        }

        match op {
            '*' => {
                let mut prod = 1;
                for n in nums {
                    prod *= n;
                }
                grand_total += dbg!(prod);
            }
            '+' => {
                let mut sum = 0;
                for n in nums {
                    sum += n;
                }
                grand_total += dbg!(sum);
            }
            _ => unreachable!("{:?}", op),
        }
        j += 1;
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
        assert_correct_answer_on_correct_input!(part_1, "INPUT", Part::One);
    }

    #[test]
    fn part_2_works_on_example() {
        assert_correct_answer_on_correct_input!(part_2, "EXAMPLE", Part::Two);
    }

    #[test]
    fn part_2_works_on_input() {
        assert_correct_answer_on_correct_input!(part_2, "INPUT", Part::Two);
    }

    #[test]
    fn returns_error_on_wrong_input() {
        assert_error_on_wrong_input!(part_1, part_2);
    }
}
