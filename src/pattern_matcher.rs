use std::{
    fs::File,
    io::{BufRead, BufReader},
    path,
};

use anyhow::{Context, Result};

use crate::{Console, Settings};

pub struct Searcher {}

impl Searcher {
    pub fn new() -> Self {
        Self {}
    }

    pub fn lookup(
        &self,
        path: &path::PathBuf,
        pattern: &str,
        settings: &Settings,
        console: &Console,
    ) -> Result<Vec<(usize, String)>> {
        let file =
            File::open(path).with_context(|| format!("Could not open {}", path.display()))?;
        let reader = BufReader::new(file);

        let mut matches = Vec::new();

        for (index, line) in reader.lines().enumerate() {
            let line = match line {
                Ok(line) => line,
                Err(e) => {
                    console.print_error(e.to_string().as_str());
                    continue;
                }
            };

            let pattern_to_check = if settings.ignore_case {
                pattern.to_lowercase()
            } else {
                pattern.to_string()
            };

            if line.contains(&pattern_to_check) {
                matches.push((index + 1, line));
            }
        }

        for (line_number, line) in &matches {
            println!("{}: {}", line_number, line);
        }

        Ok(matches)
    }
}

// fusil
