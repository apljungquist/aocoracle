use crate::AnyError;
use glob::glob;
use std::collections::BTreeMap;
use std::fs;

type Answers = BTreeMap<u16, BTreeMap<u8, BTreeMap<u8, BTreeMap<String, String>>>>;

fn _year_day(file: &str) -> (u16, u8) {
    let re = regex::Regex::new(r"y(\d{4})/d(\d{2})").expect("Hard coded regex is valid");
    let cap = re.captures(file).unwrap();
    let year = cap[1].parse::<u16>().unwrap();
    let day = cap[2].parse::<u8>().unwrap();
    (year, day)
}

pub fn read_input(year: u16, day: u8, stem: &str) -> String {
    fs::read_to_string(format!("data/inputs/{:04}/{:02}/{}.txt", year, day, stem)).unwrap()
}

pub fn available_inputs() -> Vec<(u16, u8, String)> {
    let mut result = Vec::new();
    for entry in glob("data/inputs/*/*/*.txt").unwrap() {
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

pub fn actual_answer<F, T>(file: &str, func: F, stem: &str) -> T
where
    F: Fn(&str) -> Result<T, AnyError>,
{
    let (year, day) = _year_day(file);
    func(&read_input(year, day, stem)).unwrap()
}

fn read_answers() -> Answers {
    let text = &fs::read_to_string("data/answers.json").unwrap();
    serde_json::from_str(text).unwrap()
}

pub fn expected_answer2(year: u16, day: u8, part: u8, stem: &str) -> Option<String> {
    let answers = read_answers();
    Some(answers.get(&year)?.get(&day)?.get(&part)?.get(stem)?.into())
}

pub fn expected_answer(file: &str, part: u8, stem: &str) -> Option<String> {
    let (year, day) = _year_day(file);
    expected_answer2(year, day, part, stem)
}

pub fn available_answers() -> Vec<(u16, u8, u8, String)> {
    let mut result = Vec::new();
    let answers = read_answers();
    for (year, days) in answers.into_iter() {
        for (day, parts) in days.into_iter() {
            for (part, names) in parts.into_iter() {
                for name in names.keys() {
                    result.push((year, day, part, name.clone()));
                }
            }
        }
    }
    result
}
