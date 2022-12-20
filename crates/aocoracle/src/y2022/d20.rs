fn numbers(s: &str) -> anyhow::Result<Vec<i64>> {
    // let total = s.lines().map(|l| l.parse::<i64>().unwrap()).count();
    // let uniq = s.lines().map(|l| l.parse::<i64>().unwrap()).unique().count();
    // assert_eq!(total,uniq);
    dbg!(s
        .lines()
        .map(|l| l.parse::<i64>().unwrap())
        .filter(|x| *x == 0)
        .count());
    Ok(s.lines().map(|l| l.parse().unwrap()).collect())
}

fn print_numbers(numbers: &[(i64, bool)]) {
    for (x, _) in numbers {
        print!("{}, ", x);
    }
    println!()
}

fn index(i: i64, len: usize) -> usize {
    let len: i64 = len as i64;
    (((i % len) + len) % len) as usize
}

fn move_number(mixed: &mut Vec<(i64, bool)>, number: i64) {
    let old = mixed
        .iter()
        .position(|(n, is_moved)| !*is_moved && *n == number)
        .unwrap();
    mixed.remove(old);
    let mut new = index(old as i64 + number, mixed.len());
    // Keep the order the same as in example
    if new == 0 && number < 0 {
        new = mixed.len();
    }
    mixed.insert(new, (number, true));
}

fn part_x(numbers: &[i64], num_round: usize, key: i64) -> anyhow::Result<i64> {
    let multiplied: Vec<_> = numbers.iter().map(|x| x * key).collect();
    let mut mixed: Vec<(i64, bool)> = multiplied.iter().map(|n| (*n, false)).collect();
    print_numbers(&mixed);
    for _ in 0..num_round {
        mixed.iter_mut().for_each(|x| x.1 = false);
        for x in multiplied.iter() {
            move_number(&mut mixed, *x);
        }
        print_numbers(&mixed);
    }
    let i = mixed.iter().position(|(n, _)| *n == 0).unwrap();
    let summands = vec![
        mixed[index(i as i64 + 1000, mixed.len())].0,
        mixed[index(i as i64 + 2000, mixed.len())].0,
        mixed[index(i as i64 + 3000, mixed.len())].0,
    ];
    dbg!(&summands);
    Ok(summands.iter().sum())
}

pub fn part_1(input: &str) -> anyhow::Result<i64> {
    let numbers = numbers(input)?;
    part_x(&numbers, 1, 1)
}

pub fn part_2(input: &str) -> anyhow::Result<i64> {
    let numbers = numbers(input)?;
    let answer = part_x(&numbers, 10, 811589153)?;
    dbg!(answer);
    assert!(answer == 1623178306 || 190723450955 < answer);
    Ok(answer)
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

    #[test]
    fn index_works() {
        let xs: Vec<_> = (0..10).collect();
        assert_eq!(xs[index(0, xs.len())], 0);
        assert_eq!(xs[index(9, xs.len())], 9);
        assert_eq!(xs[index(-1, xs.len())], 9);
        assert_eq!(xs[index(-11, xs.len())], 9);
        assert_eq!(xs[index(10, xs.len())], 0);
    }
}
