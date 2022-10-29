use serde::Deserialize;
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use structopt::StructOpt;

#[cfg(test)]
mod testing;
mod y2018;
mod y2020;
mod y2021;

#[derive(Eq, Ord, PartialEq, PartialOrd, Clone, Copy, Deserialize)]
pub enum Part {
    #[serde(alias = "1")]
    One,
    #[serde(alias = "2")]
    Two,
}

impl TryFrom<u8> for Part {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::One),
            2 => Ok(Self::Two),
            _ => Err(format!("Expected integer in range [1, 2] but got {value}")),
        }
    }
}

impl FromStr for Part {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(Self::One),
            "2" => Ok(Self::Two),
            _ => Err(format!("Expected one of '1' or '2' but got {s}")),
        }
    }
}

impl Display for Part {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(
            &match self {
                Self::One => 1,
                Self::Two => 2,
            },
            f,
        )
    }
}

#[derive(StructOpt)]
pub struct Cli {
    #[structopt(long)]
    year: Option<u16>,
    #[structopt(long)]
    day: Option<u8>,
    #[structopt(long)]
    part: Option<Part>,
    #[structopt(long)]
    pub exhaustive: bool,
}

impl Cli {
    pub fn new(year: Option<u16>, day: Option<u8>, part: Option<Part>, exhaustive: bool) -> Self {
        Self {
            year,
            day,
            part,
            exhaustive,
        }
    }
}

pub type AnyError = Box<dyn std::error::Error>;
type Solver = dyn Fn(&str) -> Result<String, AnyError>;

fn _available_solvers() -> BTreeMap<(u16, u8, Part), &'static Solver> {
    let mut functions: BTreeMap<_, &Solver> = BTreeMap::new();
    functions.insert((2018, 1, Part::One), &y2018::d01::part_1);
    functions.insert((2018, 1, Part::Two), &y2018::d01::part_2);
    functions.insert((2020, 1, Part::One), &y2020::d01::part_1);
    functions.insert((2020, 1, Part::Two), &y2020::d01::part_2);
    functions.insert((2020, 2, Part::One), &y2020::d02::part_1);
    functions.insert((2020, 2, Part::Two), &y2020::d02::part_2);
    functions.insert((2021, 1, Part::One), &y2021::d01::part_1);
    functions.insert((2021, 1, Part::Two), &y2021::d01::part_2);
    functions.insert((2021, 2, Part::One), &y2021::d02::part_1);
    functions.insert((2021, 2, Part::Two), &y2021::d02::part_2);
    functions.insert((2021, 3, Part::One), &y2021::d03::part_1);
    functions.insert((2021, 3, Part::Two), &y2021::d03::part_2);
    functions.insert((2021, 4, Part::One), &y2021::d04::part_1);
    functions.insert((2021, 4, Part::Two), &y2021::d04::part_2);
    functions.insert((2021, 5, Part::One), &y2021::d05::part_1);
    functions.insert((2021, 5, Part::Two), &y2021::d05::part_2);
    functions.insert((2021, 6, Part::One), &y2021::d06::part_1);
    functions.insert((2021, 6, Part::Two), &y2021::d06::part_2);
    functions.insert((2021, 7, Part::One), &y2021::d07::part_1);
    functions.insert((2021, 7, Part::Two), &y2021::d07::part_2);
    functions.insert((2021, 8, Part::One), &y2021::d08::part_1);
    functions.insert((2021, 8, Part::Two), &y2021::d08::part_2);
    functions.insert((2021, 20, Part::One), &y2021::d20::part_1);
    functions.insert((2021, 20, Part::Two), &y2021::d20::part_2);
    functions.insert((2021, 21, Part::One), &y2021::d21::part_1);
    functions.insert((2021, 21, Part::Two), &y2021::d21::part_2);
    functions.insert((2021, 22, Part::One), &y2021::d22::part_1);
    functions.insert((2021, 22, Part::Two), &y2021::d22::part_2);
    functions.insert((2021, 23, Part::One), &y2021::d23::part_1);
    functions.insert((2021, 23, Part::Two), &y2021::d23::part_2);
    functions.insert((2021, 24, Part::One), &y2021::d24::part_1);
    functions.insert((2021, 24, Part::Two), &y2021::d24::part_2);
    functions.insert((2021, 25, Part::One), &y2021::d25::part_1);
    functions
}

fn _candidates(args: &Cli) -> Result<BTreeMap<(u16, u8, Part), &Solver>, AnyError> {
    let functions = _available_solvers();
    let mut result = BTreeMap::new();
    let parts: Vec<Part> = match args.part {
        None => vec![Part::One, Part::Two],
        Some(Part::One) => vec![Part::One],
        Some(Part::Two) => vec![Part::Two],
    };

    let days: Vec<u8> = match args.day {
        None => 1..=25,
        Some(day) => day..=day,
    }
    .collect();

    let years: Vec<u16> = match args.year {
        None => 2018..=2021,
        Some(year) => year..=year,
    }
    .collect();

    for year in years {
        for &day in &days {
            for &part in &parts {
                let key = (year, day, part);
                if let Some(&func) = functions.get(&key) {
                    result.insert(key, func);
                }
            }
        }
    }

    if result.is_empty() {
        return Err("No matching candidates".into());
    }

    Ok(result)
}

pub fn helper(args: &Cli, text: &str) -> Result<BTreeMap<(u16, u8, Part), String>, AnyError> {
    let candidates = _candidates(args)?;
    if candidates.is_empty() {
        return Err("Invalid combination of year, day and part".into());
    }

    let mut result = BTreeMap::new();
    for ((year, day, part), func) in candidates.iter() {
        log::debug!("Trying year {} day {} part {}", year, day, part);
        match func(text) {
            Ok(output) => {
                if !args.exhaustive && !result.is_empty() {
                    return Err("Found more than 1 possible answer".into());
                }
                let entry = result.entry((*year, *day)).or_insert_with(BTreeMap::new);
                entry.insert(*part, output);
            }
            Err(error) => match args.day {
                None => log::debug!("{}", error),
                Some(_) => return Err(error),
            },
        }
    }
    Ok(result
        .into_iter()
        .flat_map(|((y, d), vs)| vs.into_iter().map(move |(p, v)| ((y, d, p), v)))
        .collect())
}

pub fn helper_text(args: &Cli, text: &str) -> Result<Vec<String>, AnyError> {
    let structured = helper(args, text)?;
    Ok(match args.exhaustive {
        false => structured.iter().map(|(_, v)| v.to_string()).collect(),
        true => structured
            .iter()
            .map(|((y, d, p), v)| format!("y{}d{:02}p{}: {}", y, d, p, v))
            .collect(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::{available_answers, expected_answer2, read_input};
    use itertools::Itertools;
    use std::fmt::Display;

    fn as_strings<KT: Display, VT: Display>(map: &BTreeMap<KT, VT>) -> BTreeMap<String, String> {
        map.iter()
            .map(|(k, v)| (format!("{k}"), format!("{v}")))
            .collect()
    }

    fn as_ascii_table<RHT: Display, CHT: Display, CT: Display>(
        cols: &BTreeMap<CHT, BTreeMap<RHT, CT>>,
    ) -> Result<String, AnyError> {
        let mut cols: BTreeMap<String, BTreeMap<String, String>> = cols
            .iter()
            .map(|(col_header, cells)| (format!("{col_header}"), as_strings(cells)))
            .collect();
        let col_headers: Vec<_> = cols.keys().cloned().sorted().collect();
        let row_headers: Vec<_> = cols
            .iter()
            .flat_map(|(_, cells)| cells.keys())
            .unique()
            .cloned()
            .sorted()
            .collect();
        let row_headers_width = row_headers.iter().map(|rh| rh.len()).max().ok_or(format!(
            "Expected at least one row but got {}",
            row_headers.len()
        ))?;
        let col_widths: Vec<usize> = cols
            .iter()
            .map(|(col_header, cells)| {
                cells
                    .values()
                    .map(|cell| cell.len())
                    .max()
                    .unwrap_or(0)
                    .max(col_header.len())
            })
            .collect();

        let mut result = String::new();

        // Columns headers
        result.push_str(&format!("{:>1$} ", "", row_headers_width));
        for (col_width, col_header) in col_widths.iter().zip(col_headers) {
            result.push_str(&format!("| {:^1$} ", col_header, col_width));
        }
        result.push('\n');

        // Horizontal line
        result.push_str(&format!("{:->1$}-", "", row_headers_width));
        for col_width in col_widths.iter() {
            result.push_str(&format!("+-{:-^1$}-", "", col_width));
        }

        // Row headers and cells
        result.push('\n');
        for row_header in row_headers {
            result.push_str(&format!("{:>1$} ", &row_header, row_headers_width));
            for (col_width, cells) in col_widths.iter().zip(cols.values_mut()) {
                result.push_str(&format!(
                    "| {:^1$} ",
                    cells.remove(&row_header).unwrap_or_default(),
                    col_width
                ))
            }
            result.push('\n');
        }

        Ok(result)
    }

    #[test]
    fn every_input_is_solved_by_exactly_one_solver() -> Result<(), AnyError> {
        let mut cols = BTreeMap::new();
        let mut num_false_positive = 0;
        let mut num_false_negative = 0;
        for (year, day, part) in _available_solvers().into_keys() {
            let stems = available_answers().into_iter().filter_map(|(y, d, p, s)| {
                if y == year && d == day && p == part {
                    Some(s)
                } else {
                    None
                }
            });
            for stem in stems {
                let actual_answers = helper(
                    &Cli::new(None, None, Some(part), true),
                    &read_input(year, day, &stem),
                )?;
                let expected_answer = expected_answer2(year, day, part, &stem).unwrap();
                if actual_answers.is_empty() {
                    num_false_negative += 1;
                    let input_key = format!("{year}/{day:02}/{stem}");
                    let solver_key = format!("{year}/{day:02}::{part}");
                    let entry = cols.entry(solver_key).or_insert_with(BTreeMap::new);
                    assert_eq!(
                        entry.insert(input_key, "-".into()),
                        None,
                        "Expected every input-solver pair to occur at most once"
                    );
                }
                for ((y, d, p), actual_answer) in actual_answers {
                    if actual_answer != expected_answer {
                        num_false_positive += 1;
                        let input_key = format!("{year}/{day:02}/{stem}");
                        let solver_key = format!("{y}/{d:02}::{p}");
                        let entry = cols.entry(solver_key).or_insert_with(BTreeMap::new);
                        assert_eq!(
                            entry.insert(input_key, actual_answer),
                            None,
                            "Expected every input-solver pair to occur at most once"
                        );
                    }
                }
            }
        }
        println!("{}", as_ascii_table(&cols)?);
        assert_eq!(
            (num_false_negative, num_false_positive),
            // Should be 0 but by setting it to the current value we
            // * detect regressions, and
            // * are notified if we can tighten the bound.
            (0, 6),
            "Expected no false negatives and false positives but got {} and {} respectively",
            num_false_negative,
            num_false_positive,
        );
        Ok(())
    }
}
