use std::fs;

struct Subroutine {
    should_pop: bool,
    gate_offset: i64,
    source_offset: i64,
}

const RADIX: u32 = 26;

impl Subroutine {
    fn from_lines(lines: &[&str]) -> Subroutine {
        Subroutine {
            should_pop: match lines[4].split_whitespace().nth(2).unwrap().parse().unwrap() {
                1 => false,
                RADIX => true,
                _ => panic!("Unexpected value for b"),
            },
            gate_offset: lines[5].split_whitespace().nth(2).unwrap().parse().unwrap(),
            source_offset: lines[15]
                .split_whitespace()
                .nth(2)
                .unwrap()
                .parse()
                .unwrap(),
        }
    }

    fn evaluate(&self, w: i64, z: i64) -> (i8, i64) {
        let radix = RADIX as i64;
        let should_push = ((z % radix) + self.gate_offset) != w;
        match (self.should_pop, should_push) {
            (false, false) => (0, z),
            (false, true) => (1, z * radix + self.source_offset + w),
            (true, false) => (-1, z / radix),
            (true, true) => (0, z / radix * radix + self.source_offset + w),
        }
    }
}

fn _subroutines(text: &str) -> Vec<Subroutine> {
    text.lines()
        .collect::<Vec<&str>>()
        .chunks(18)
        .map(Subroutine::from_lines)
        .collect()
}

fn _fmt_int(mut value: u64, radix: u32) -> String {
    let mut chars = Vec::new();
    while value != 0 {
        chars.push(std::char::from_digit(value as u32 % radix, radix).unwrap());
        value /= radix as u64;
    }
    chars.into_iter().rev().collect()
}

fn _first_valid(subroutines: &[Subroutine], old_z: i64, path: u64, digits: &[u64]) -> Option<u64> {
    if subroutines.is_empty() {
        if old_z == 0 {
            return Some(path);
        } else {
            return None;
        }
    }

    for digit in digits {
        let (delta, new_z) = subroutines[0].evaluate(*digit as i64, old_z);
        if subroutines[0].should_pop && 0 <= delta {
            continue;
        }
        match _first_valid(&subroutines[1..], new_z, path * 10 + digit, digits) {
            Some(result) => return Some(result),
            None => continue,
        }
    }
    None
}

pub fn part_1(input: &str) -> u64 {
    let subroutines = _subroutines(input);
    _first_valid(&subroutines[..], 0, 0, &[9, 8, 7, 6, 5, 4, 3, 2, 1][..]).unwrap()
}

pub fn part_2(input: &str) -> u64 {
    let subroutines = _subroutines(input);
    _first_valid(&subroutines[..], 0, 0, &[1, 2, 3, 4, 5, 6, 7, 8, 9][..]).unwrap()
}

fn _from_file<F, T>(func: F, stem: &str) -> T
where
    F: Fn(&str) -> T,
{
    func(&fs::read_to_string(format!("inputs/24/{}.txt", stem)).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_works_on_input() {
        assert_eq!(_from_file(part_1, "input"), 41299994879959);
    }

    #[test]
    fn part_2_works_on_input() {
        assert_eq!(_from_file(part_2, "input"), 11189561113216);
    }
}
