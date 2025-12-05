pub fn part_1(input: &str) -> anyhow::Result<u32> {
    let mut lines = input.lines();

    let mut fresh = Vec::new();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        let (a, b) = line.split_once("-").unwrap();
        let a = a.parse::<i64>()?;
        let b = b.parse::<i64>()?;
        fresh.push(a..=b);
    }

    let mut available = Vec::new();
    for line in lines {
        let a = line.parse::<i64>()?;
        available.push(a);
    }

    let mut sum = 0;
    for x in available {
        for r in fresh.iter() {
            if r.contains(&x) {
                sum += 1;
                break;
            }
        }
    }
    Ok(sum)
}
pub fn part_2(input: &str) -> anyhow::Result<i128> {
    let mut lines = input.lines();

    let mut fresh = Vec::new();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        let (a, b) = line.split_once("-").unwrap();
        let a = a.parse::<i128>()?;
        let b = b.parse::<i128>()?;
        fresh.push((a, b));
    }

    let mut seen = Vec::new();
    'outer: while let Some((first, last)) = fresh.pop() {
        for (lo, hi) in seen.iter().cloned() {
            if lo <= first && last <= hi {
                continue 'outer; // The range is contained in a previous range
            }
            if first < lo && (lo <= last && last <= hi) {
                // println!("a");
                fresh.push((first, lo - 1));
                continue 'outer;
            }
            if (lo <= first && first <= hi) && hi < last {
                // println!("b");
                fresh.push((hi + 1, last));
                continue 'outer;
            }
            if first < lo && hi < last {
                println!("c");
                fresh.push(((first, lo - 1)));
                fresh.push(((hi + 1, last)));
                continue 'outer;
            }
            // println!("d");
        }
        seen.push((first, last));
    }

    // 474235903668691                                                                 481728314013923
    //                                 476951396635787 477940327871613
    // 474235903668691 476951396635786                                 477940327871614 481728314013923

    // dbg!(&seen);

    let mut sum = 0;
    for (first, last) in seen {
        let n = last - first + 1;
        sum += n;
    }

    assert_ne!(sum, 367378579850346);
    assert_ne!(sum, 358626364849641);
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
    }

    #[test]
    fn returns_error_on_wrong_input() {
        assert_error_on_wrong_input!(part_1, part_2);
    }
}
