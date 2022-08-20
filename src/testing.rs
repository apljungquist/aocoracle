use crate::AnyError;
use std::fs;

pub fn compute_answer<F, T>(file: &str, func: F, stem: &str) -> T
where
    F: Fn(&str) -> Result<T, AnyError>,
{
    let re = regex::Regex::new(r"y(\d{4})/d(\d{2})").expect("Hard coded regex is valid");
    let cap = re.captures(file).unwrap();
    let year = cap[1].parse::<u16>().unwrap();
    let day = cap[2].parse::<u8>().unwrap();

    func(&fs::read_to_string(format!("inputs/y{:04}/d{:02}/{}.txt", year, day, stem)).unwrap())
        .unwrap()
}
