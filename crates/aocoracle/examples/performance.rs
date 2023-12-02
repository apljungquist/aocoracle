use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::panic;

use aocdata::{read_examples, Example, InputId};
use aoclib::{helper, Cli, SolverId};
use itertools::Itertools;
use log::warn;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Cell {
    FalseNegative,
    FalsePositive,
    TrueNegative,
    TruePositive,
    Substitution,
    Unknown,
}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FalseNegative => write!(f, "D"),
            Self::FalsePositive => write!(f, "I"),
            Self::TrueNegative => write!(f, "N"),
            Self::TruePositive => write!(f, "C"),
            Self::Substitution => write!(f, "S"),
            Self::Unknown => write!(f, "U"),
        }
    }
}

struct AnswerPair {
    actual: Result<String, ()>,
    expected: Option<Result<String, ()>>,
}

impl From<&AnswerPair> for Cell {
    fn from(pair: &AnswerPair) -> Self {
        match (&pair.actual, &pair.expected) {
            (Ok(a), Some(Ok(e))) if a == e => Cell::TruePositive,
            (Ok(_), Some(Ok(_))) => Cell::Substitution,
            (Ok(_), Some(Err(_))) => Cell::FalsePositive,
            (Ok(_), None) => Cell::Unknown,
            (Err(_), Some(Ok(_))) => Cell::FalseNegative,
            (Err(_), Some(Err(_))) => Cell::TrueNegative,
            (Err(_), None) => Cell::Unknown,
        }
    }
}

fn compute_pairs(examples: &[Example]) -> anyhow::Result<Vec<((InputId, SolverId), AnswerPair)>> {
    let mut pairs = Vec::new();
    for example in examples {
        let input_id = example.input_id();
        let solver_id = example.solver_id();
        let cli = Cli::new(None, None, Some(example.part), true);

        // FIXME: Previously a panic would erase only one cell, now it erases the whole row.
        //  So if any one solver panics for an input, we get not results for that input.
        let mut solutions = match panic::catch_unwind(|| helper(&cli, &example.input).unwrap()) {
            Ok(solutions) => solutions,
            Err(e) => {
                warn!("Excluding {input_id} because of a panic {e:?}");
                continue;
            }
        };
        let actual_answer = solutions.remove(&example.solver_id()).ok_or(());
        pairs.push((
            (input_id, solver_id),
            AnswerPair {
                actual: actual_answer,
                expected: example.answer.as_ref().map(|a| Ok(a.clone())),
            },
        ));
        for (solver_id, actual_answer) in solutions {
            pairs.push((
                (example.input_id(), solver_id),
                AnswerPair {
                    actual: Ok(actual_answer),
                    expected: Some(Err(())),
                },
            ));
        }
    }
    Ok(pairs)
}

fn nested<RHT: Ord, CHT: Ord, CT>(
    cells: Vec<((RHT, CHT), CT)>,
) -> BTreeMap<CHT, BTreeMap<RHT, CT>> {
    let mut result: BTreeMap<CHT, BTreeMap<RHT, CT>> = BTreeMap::new();
    for ((row_header, col_header), cell) in cells {
        result
            .entry(col_header)
            .or_default()
            .insert(row_header, cell);
    }
    result
}

fn as_strings<KT: Display, VT: Display>(map: &BTreeMap<KT, VT>) -> BTreeMap<String, String> {
    map.iter()
        .map(|(k, v)| (format!("{k}"), format!("{v}")))
        .collect()
}

fn as_ascii_table<RHT: Display, CHT: Display, CT: Display>(
    cols: &BTreeMap<CHT, BTreeMap<RHT, CT>>,
) -> anyhow::Result<String> {
    if cols.is_empty() {
        return Ok(String::new());
    }
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
    let row_headers_width = row_headers.iter().map(|rh| rh.len()).max().ok_or_else(|| {
        anyhow::anyhow!("Expected at least one row but got {}", row_headers.len())
    })?;
    let col_widths: Vec<usize> = cols
        .iter()
        .map(|(_, cells)| cells.values().map(|cell| cell.len()).max().unwrap_or(0))
        .collect();

    let mut result = String::new();

    // Columns headers (rotated)
    for i in 0..col_headers.iter().map(|s| s.len()).max().unwrap() {
        result.push_str(&format!("{:>1$} ", "", row_headers_width));
        for (col_width, col_header) in col_widths.iter().zip(&col_headers) {
            result.push_str(&format!(
                "| {:^1$} ",
                col_header.chars().nth(i).unwrap_or_default(),
                col_width
            ));
        }
        result.push('\n');
    }

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

fn print_one_table_per_year(cells: &[((InputId, SolverId), AnswerPair)]) {
    for year in 2015..=2023 {
        let mut selected = Vec::new();
        for ((input_id, solver_id), pair) in cells {
            if solver_id.year != year {
                continue;
            }
            selected.push(((input_id.clone(), solver_id.clone()), Cell::from(pair)));
        }
        println!("{}", as_ascii_table(&nested(selected)).unwrap());
    }
}

fn data_path() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .unwrap()
        .join("data")
}

fn main() {
    env_logger::builder().is_test(true).try_init().unwrap();
    let cells = compute_pairs(&read_examples(&data_path(), None, None).unwrap()).unwrap();
    print_one_table_per_year(&cells);
}

#[test]
fn expected_number_of_cells() {
    use Cell::*;
    let pairs = compute_pairs(&read_examples(&data_path(), None, None).unwrap()).unwrap();
    let mut counts = pairs.iter().map(|(_, pair)| Cell::from(pair)).counts();
    let num_false_negative = counts.remove(&FalseNegative).unwrap_or(0);
    let num_false_positive = counts.remove(&FalsePositive).unwrap_or(0);
    let num_substitution = counts.remove(&Substitution).unwrap_or(0);
    let num_true_negative = counts.remove(&TrueNegative).unwrap_or(0);
    let num_true_positive = counts.remove(&TruePositive).unwrap_or(0);
    let num_unknown = counts.remove(&Unknown).unwrap_or(0);
    assert_eq!(
        dbg!((
            num_false_negative,
            num_false_positive,
            num_substitution,
            num_true_negative,
            num_true_positive,
            num_unknown,
        )),
        (33, 174, 4, 0, 215, 660),
    );
}
