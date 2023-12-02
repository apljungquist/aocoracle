use std::fmt::Formatter;
use std::path::Path;

use log::debug;

use aoclib::{Part, SolverId};

type Year = u16;
type Day = u8;

#[derive(Debug)]
pub struct Answer {
    pub year: Year,
    pub day: Day,
    pub name: String,
    pub part: Part,
    content: String,
}

impl Answer {
    fn try_new(data: &Path, input: &Input, part: Part) -> Option<Self> {
        let path = data.join(format!(
            "{:04}/{:02}/answers/{:01}/{}.txt",
            input.year, input.day, part, input.name
        ));
        let content = match std::fs::read_to_string(&path) {
            Ok(c) => c,
            Err(e) => {
                debug!(
                    "No answer found for input {} at {path:?} because {e}",
                    input.id(),
                );
                return None;
            }
        };
        Some(Self {
            year: input.year,
            day: input.day,
            name: input.name.clone(),
            part,
            content,
        })
    }

    pub fn id(&self) -> String {
        format!(
            "{:04}:{:02}:{}:{}",
            self.year, self.day, self.name, self.part
        )
    }

    // I haven't yet decided if it makes sense to read the content eagerly.
    // If I decide against it then having infallible access to the content will not be possible.
    pub fn read(&self) -> anyhow::Result<&str> {
        Ok(&self.content)
    }
}

#[derive(Clone, Debug)]
pub struct Input {
    pub year: Year,
    pub day: Day,
    pub name: String,
    content: String,
}

impl Input {
    fn try_new(path: &Path) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let mut ancestors = path.ancestors();
        let name = ancestors
            .next()
            .ok_or_else(|| anyhow::anyhow!("Expected path to contain a name but got {path:?}"))?
            .file_stem()
            .ok_or_else(|| anyhow::anyhow!("Expected path to contain name but got {path:?}"))?
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("Expected path to be valid UTF-8 but got {path:?}"))?
            .to_string();
        ancestors
            .next()
            .ok_or_else(|| anyhow::anyhow!("Expected path to contain 'inputs' but got {path:?}"))?;
        let day = ancestors
            .next()
            .ok_or_else(|| anyhow::anyhow!("Expected path to contain a day but got {path:?}"))?
            .file_name()
            .ok_or_else(|| anyhow::anyhow!("Expected path to contain day but got {path:?}"))?
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("Expected path to be valid UTF-8 but got {path:?}"))?
            .parse()?;
        let year = ancestors
            .next()
            .ok_or_else(|| anyhow::anyhow!("Expected path to contain a year but got {path:?}"))?
            .file_name()
            .ok_or_else(|| anyhow::anyhow!("Expected path to contain year but got {path:?}"))?
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("Expected path to be valid UTF-8 but got {path:?}"))?
            .parse()?;
        Ok(Self {
            year,
            day,
            name,
            content,
        })
    }

    pub fn id(&self) -> InputId {
        InputId {
            year: self.year,
            day: self.day,
            name: self.name.clone(),
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct InputId {
    year: Year,
    day: Day,
    name: String,
}

impl std::fmt::Display for InputId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}:{}", self.year % 100, self.day, self.name)
    }
}

pub struct Example {
    pub year: Year,
    pub day: Day,
    pub name: String,
    pub part: Part,
    pub input: String,
    pub answer: Option<String>,
}

impl Example {
    pub fn solver_id(&self) -> SolverId {
        SolverId::try_new(self.year, self.day, self.part).unwrap()
    }

    pub fn input_id(&self) -> InputId {
        InputId {
            year: self.year,
            day: self.day,
            name: self.name.clone(),
        }
    }
}

pub fn read_examples(
    data: &Path,
    year: Option<Year>,
    day: Option<Day>,
) -> anyhow::Result<Vec<Example>> {
    let mut result = Vec::new();
    for year_path in data.read_dir()? {
        let year_path = year_path?.path();
        if !year_path.is_dir() {
            continue;
        }
        for day_path in year_path.read_dir()? {
            let day_path = day_path?.path();
            if !day_path.is_dir() {
                continue;
            }
            for input_path in day_path.join("inputs").read_dir()? {
                let input_path = input_path?.path();
                let input = Input::try_new(&input_path)?;

                if let Some(year) = year {
                    if input.year != year {
                        continue;
                    }
                }
                if let Some(day) = day {
                    if input.day != day {
                        continue;
                    }
                }

                for part in [Part::One, Part::Two] {
                    let answer = Answer::try_new(data, &input, part);
                    result.push(Example {
                        year: input.year,
                        day: input.day,
                        name: input.name.clone(),
                        part,
                        input: input.content.clone(),
                        answer: answer.map(|a| a.content),
                    });
                }
            }
        }
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::read_examples;

    fn data_path() -> std::path::PathBuf {
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .ancestors()
            .nth(2)
            .unwrap()
            .join("data")
    }

    #[test]
    fn finds_expected_examples() {
        let examples = read_examples(&data_path(), None, None).unwrap();
        // As of writing this is the number of examples and I don't expect it to get smaller
        assert!(211 + 172 + 4 + 2 <= examples.len());
    }
}
