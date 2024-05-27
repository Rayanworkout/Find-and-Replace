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

    /// Method to search for a pattern in a specific file
    /// It builds a list of matches and prints them to the console
    /// If verbose is true, we also print errors to the console
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
                Ok(line) => {
                    if settings.ignore_case {
                        line.to_lowercase()
                    } else {
                        line
                    }
                }
                Err(e) => {
                    let path_str = match path.to_str() {
                        Some(path_str) => path_str,
                        None => {
                            return Err(anyhow::anyhow!("Could not convert path to string."));
                        }
                    };

                    if settings.verbose {
                        console.print_error(e.to_string().as_str(), &path_str);
                    }

                    // If the file is not utf-8 encoded, we early return an empty vector
                    return Ok(Vec::new());
                }
            };

            if line.contains(&pattern) {
                matches.push((index + 1, line));
            }
        }

        Ok(matches)
    }
}
