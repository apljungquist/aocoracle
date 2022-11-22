use crate::Part;
use glob::glob;
use std::any::type_name;
use std::collections::BTreeMap;
use std::fmt::Debug;
use std::fs;

type Answers = BTreeMap<u16, BTreeMap<u8, BTreeMap<Part, BTreeMap<String, String>>>>;

fn year_day(file: &str) -> (u16, u8) {
    let re = regex::Regex::new(r"y(\d{4})/d(\d{2})").expect("Hard coded regex is valid");
    let cap = re.captures(file).unwrap();
    let year = cap[1].parse::<u16>().unwrap();
    let day = cap[2].parse::<u8>().unwrap();
    (year, day)
}

pub fn read_input(year: u16, day: u8, stem: &str) -> String {
    fs::read_to_string(format!(
        "../../data/inputs/{:04}/{:02}/{}.txt",
        year, day, stem
    ))
    .unwrap()
}

pub fn available_inputs() -> Vec<(u16, u8, String)> {
    let mut result = Vec::new();
    for entry in glob("../../data/inputs/*/*/*.txt").unwrap() {
        let path = entry.unwrap();
        let stem = path.file_stem().unwrap().to_str().unwrap();
        let day = path
            .ancestors()
            .nth(1)
            .unwrap()
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap();
        let year = path
            .ancestors()
            .nth(2)
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

pub fn actual_answer<F, T, U>(file: &str, func: F, stem: &str) -> Result<T, U>
where
    F: Fn(&str) -> Result<T, U>,
{
    let (year, day) = year_day(file);
    func(&read_input(year, day, stem))
}

fn read_answers() -> Answers {
    let text = &fs::read_to_string("../../data/answers.json").unwrap();
    serde_json::from_str(text).unwrap()
}

pub fn expected_answer(year: u16, day: u8, part: Part, stem: &str) -> Option<String> {
    let answers = read_answers();
    Some(answers.get(&year)?.get(&day)?.get(&part)?.get(stem)?.into())
}

pub fn assert_correct_answer_on_correct_input_given_file<F, T, U>(
    file: &str,
    func: F,
    part: Part,
    stem: &str,
) where
    F: Fn(&str) -> Result<T, U>,
    T: ToString,
    U: Debug,
{
    let (year, day) = year_day(file);
    assert_eq!(
        actual_answer(file, func, stem).unwrap().to_string(),
        expected_answer(year, day, part, stem).unwrap(),
    )
}

pub fn assert_error_on_wrong_input_given_file<F, T, U>(file: &str, func: F)
where
    F: Fn(&str) -> Result<T, U>,
{
    let (skip_year, skip_day) = year_day(file);
    for (year, day, stem) in available_inputs() {
        if year == skip_year && day == skip_day {
            continue;
        }
        println!("{} y{} d{} {}", type_name::<F>(), year, day, stem);
        assert!(func(&read_input(year, day, &stem)).is_err());
    }
}

macro_rules! assert_correct_answer_on_correct_input {
    ($func:expr, $stem:expr, $part:expr) => {
        $crate::testing::assert_correct_answer_on_correct_input_given_file(
            file!(),
            $func,
            $part,
            $stem,
        )
    };
}
pub(crate) use assert_correct_answer_on_correct_input;

macro_rules! assert_error_on_wrong_input {
    ($($func:expr),*) => {$(
        $crate::testing::assert_error_on_wrong_input_given_file(file!(), $func);
    )*};
}
pub(crate) use assert_error_on_wrong_input;
