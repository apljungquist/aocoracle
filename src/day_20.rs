use hashbrown::HashMap;
use itertools::Itertools;
use std::fs;

type Img = HashMap<(i32, i32), bool>;
type Key = (bool, bool, bool, bool, bool, bool, bool, bool, bool);
type Lut = HashMap<Key, bool>;

fn _key(int: usize) -> Key {
    assert!(int < 512);
    (
        int & 0b100000000 != 0,
        int & 0b010000000 != 0,
        int & 0b001000000 != 0,
        int & 0b000100000 != 0,
        int & 0b000010000 != 0,
        int & 0b000001000 != 0,
        int & 0b000000100 != 0,
        int & 0b000000010 != 0,
        int & 0b000000001 != 0,
    )
}

fn _pixel(ch: char) -> bool {
    match ch {
        '.' => false,
        '#' => true,
        _ => panic!("Unexpected char '{}'", ch),
    }
}
fn _img(text: &str) -> Img {
    let mut result = HashMap::new();
    let mut lines = text.lines();
    lines.next();
    lines.next();
    for (r, line) in lines.enumerate() {
        for (c, cell) in line.chars().enumerate() {
            result.insert((r as i32, c as i32), _pixel(cell));
        }
    }

    result
}

fn _lut(text: &str) -> HashMap<Key, bool> {
    let mut lines = text.lines();
    let line = lines.next().unwrap();
    line.chars()
        .enumerate()
        .map(|(i, ch)| (_key(i), _pixel(ch)))
        .collect()
}
fn _once_enhanced(img: &Img, lut: &Lut, padding: bool, min: i32, max: i32) -> Img {
    let mut result = HashMap::with_capacity((max - min + 3).pow(2) as usize);
    for r in min - 1..=max + 1 {
        for c in min - 1..=max + 1 {
            let key: Key = (
                *img.get(&(r - 1, c - 1)).unwrap_or(&padding),
                *img.get(&(r - 1, c)).unwrap_or(&padding),
                *img.get(&(r - 1, c + 1)).unwrap_or(&padding),
                *img.get(&(r, c - 1)).unwrap_or(&padding),
                *img.get(&(r, c)).unwrap_or(&padding),
                *img.get(&(r, c + 1)).unwrap_or(&padding),
                *img.get(&(r + 1, c - 1)).unwrap_or(&padding),
                *img.get(&(r + 1, c)).unwrap_or(&padding),
                *img.get(&(r + 1, c + 1)).unwrap_or(&padding),
            );
            result.insert((r, c), *lut.get(&key).unwrap());
        }
    }
    result
}

fn _multi_enhanced(img: &Img, lut: &Lut, num_round: usize) -> Img {
    let odd = *lut
        .get(&(
            false, false, false, false, false, false, false, false, false,
        ))
        .unwrap();
    let even = match odd {
        false => false,
        true => *lut
            .get(&(true, true, true, true, true, true, true, true, true))
            .unwrap(),
    };

    let (min, max) = match img.keys().map(|k| k.0).minmax() {
        itertools::MinMaxResult::NoElements => panic!("No elements"),
        itertools::MinMaxResult::OneElement(only) => (only, only),
        itertools::MinMaxResult::MinMax(min, max) => (min, max),
    };

    let mut result = _once_enhanced(img, lut, even, min, max);
    for i in 1..num_round as i32 {
        result = _once_enhanced(
            &result,
            lut,
            match i % 2 {
                0 => even,
                1 => odd,
                _ => panic!("Oups"),
            },
            min - i,
            max + i,
        );
    }
    result
}

pub fn part_1(input: &str) -> u64 {
    let n = 2;
    let img = _img(input);
    let lut = _lut(input);
    let enhanced = _multi_enhanced(&img, &lut, n);
    enhanced.values().map(|b| *b as u64).sum()
}

pub fn part_2(input: &str) -> u64 {
    let n = 50;
    let img = _img(input);
    let lut = _lut(input);
    let enhanced = _multi_enhanced(&img, &lut, n);
    enhanced.values().map(|b| *b as u64).sum()
}

fn _from_file<F, T>(func: F, stem: &str) -> T
where
    F: Fn(&str) -> T,
{
    func(&fs::read_to_string(format!("inputs/20/{}.txt", stem)).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_works_on_example() {
        assert_eq!(_from_file(part_1, "example"), 35);
    }

    #[test]
    fn part_1_works_on_input() {
        assert_eq!(_from_file(part_1, "input"), 5571);
    }

    #[test]
    fn part_2_works_on_example() {
        assert_eq!(_from_file(part_2, "example"), 3351);
    }

    #[test]
    fn part_2_works_on_input() {
        assert_eq!(_from_file(part_2, "input"), 17965);
    }
}
