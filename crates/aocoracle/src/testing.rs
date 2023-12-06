use std::fs;

use glob::glob;

use crate::{Part, Solver};

pub(crate) fn year_day(file: &str) -> (u16, u8) {
    let re = regex::Regex::new(r"y(\d{4})/d(\d{2})").expect("Hard coded regex is valid");
    let cap = re.captures(file).unwrap();
    let year = cap[1].parse::<u16>().unwrap();
    let day = cap[2].parse::<u8>().unwrap();
    (year, day)
}

pub fn read_input(year: u16, day: u8, stem: &str) -> String {
    fs::read_to_string(format!(
        "../../data/{:04}/{:02}/inputs/{}.txt",
        year, day, stem
    ))
    .unwrap()
}

pub fn available_inputs() -> Vec<(u16, u8, String)> {
    let mut result = Vec::new();
    for entry in glob("../../data/*/*/inputs/*.txt").unwrap() {
        let path = entry.unwrap();
        let stem = path.file_stem().unwrap().to_str().unwrap();
        let day = path
            .ancestors()
            .nth(2)
            .unwrap()
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap();
        let year = path
            .ancestors()
            .nth(3)
            .unwrap()
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap();
        result.push((
            year.parse().unwrap(),
            day.parse().unwrap(),
            String::from(stem),
        ))
    }
    result
}

pub fn actual_answer(year: u16, day: u8, stem: &str, part: Part) -> anyhow::Result<String> {
    let func = crate::_available_solvers()
        .remove(&(year, day, part))
        .unwrap();
    func(&read_input(year, day, stem))
}

pub fn expected_answer(year: u16, day: u8, stem: &str, part: Part) -> Option<String> {
    fs::read_to_string(format!(
        "../../data/{year:04}/{day:02}/answers/{part:01}/{stem}.txt"
    ))
    .ok()
}

pub fn assert_correct_answer_on_correct_input_given_file(
    file: &str,
    part: Part,
    stem: &str,
    func: Option<Box<Solver>>,
) {
    let (year, day) = year_day(file);
    let func = func.unwrap_or_else(|| {
        crate::_available_solvers()
            .remove(&(year, day, part))
            .unwrap()
    });
    assert_eq!(
        func(&read_input(year, day, stem)).unwrap(),
        expected_answer(year, day, stem, part).unwrap(),
    )
}

pub fn assert_error_on_wrong_input_given_file(file: &str, part: Part) {
    let (skip_year, skip_day) = year_day(file);
    let func = crate::_available_solvers()
        .remove(&(skip_year, skip_day, part))
        .unwrap();
    for (year, day, stem) in available_inputs() {
        if year == skip_year && day == skip_day {
            continue;
        }
        println!("y{} d{} {}", year, day, &stem);
        assert!(func(&read_input(year, day, &stem)).is_err());
    }
}

macro_rules! assert_correct_answer_on_correct_input {
    ($stem:expr, $part:expr) => {
        $crate::testing::assert_correct_answer_on_correct_input_given_file(
            file!(),
            $part,
            $stem,
            None,
        )
    };
    ($stem:expr, $part:expr, $func:expr) => {
        $crate::testing::assert_correct_answer_on_correct_input_given_file(
            file!(),
            $part,
            $stem,
            Some(crate::boxed2($func)),
        )
    };
}
pub(crate) use assert_correct_answer_on_correct_input;

macro_rules! assert_error_on_wrong_input {
    ($($part:expr),*) => {$(
        $crate::testing::assert_error_on_wrong_input_given_file(file!(), $part);
    )*};
}
pub(crate) use assert_error_on_wrong_input;

macro_rules! read_relevant_input {
    ($stem:expr) => {{
        let (year, day) = $crate::testing::year_day(file!());
        $crate::testing::read_input(year, day, $stem)
    }};
}
pub(crate) use read_relevant_input;
