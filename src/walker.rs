use crate::{Console, Replacer, Searcher, Settings};
use anyhow::{Context, Result};
use colored::Colorize;
use ignore::{types::TypesBuilder, WalkBuilder};
use std::path::PathBuf;

pub struct Walker {
    old_pattern: String,
    new_pattern: String,
    path: PathBuf,
    settings: Settings,
}

impl Walker {
    pub fn new(
        old_pattern: String,
        new_pattern: String,
        path: PathBuf,
        settings: Settings,
    ) -> Self {
        Self {
            old_pattern,
            new_pattern,
            path,
            settings,
        }
    }

    /// https://docs.rs/ignore/latest/ignore/types/struct.TypesBuilder.html
    fn build_walker(&self) -> Result<ignore::Walk> {
        let mut types_builder = TypesBuilder::new();
        types_builder.add_defaults();

        // The loop runs only if there are selected or ignored file types
        let mut count: u32 = 0;
        for t in &self.settings.selected_file_types {
            // Check if filter is file type or glob pattern
            if t.contains('*') {
                let new_type = format!("type{}", count);
                // Note: .add(name, glob) only returns error with wrong name, hence unwrap()
                types_builder.add(&new_type, t).unwrap();
                types_builder.select(&new_type);
                count += 1;
            } else {
                types_builder.select(t);
            }
        }
        for t in &self.settings.ignored_file_types {
            // Check if filter is file type or glob pattern
            if t.contains('*') {
                let new_type = format!("type{}", count);
                // Note: .add(name, glob) only returns error with wrong name, hence unwrap()
                types_builder.add(&new_type, t).unwrap();
                types_builder.negate(&new_type);
                count += 1;
            } else {
                types_builder.negate(t);
            }
        }

        let types_matcher = types_builder.build()?;

        let mut walk_builder = WalkBuilder::new(&self.path);

        walk_builder.types(types_matcher);

        // If settings.search_hidden is true, we set ignore to false
        if self.settings.search_hidden {
            walk_builder.hidden(false);
        }

        Ok(walk_builder.build())
    }

    pub fn run(&self) -> Result<()> {
        let console = Console::new();
        let walker = self.build_walker()?;
        let searcher = Searcher::new();
        let replacer = Replacer::new(console.clone(), self.settings.clone());

        let mut total_matches = 0;
        for entry in walker {
            let entry = entry
                .with_context(|| "Could not read directory entry. Maybe try with sudo ?".red())?;

            // Check if path is not in the omit list with any
            if self
                .settings
                .omit_pattern
                .iter()
                .any(|omit| entry.path().starts_with(omit))
            {
                continue;
            }

            if let Some(file_type) = entry.file_type() {
                if file_type.is_file() {
                    let file_path = entry.path().to_path_buf();
                    let matches =
                        searcher.lookup(&file_path, &self.old_pattern, &self.settings, &console)?;

                    if !matches.is_empty() {
                        let filename = entry.path().to_string_lossy();
                        console.print_filename(&filename);

                        for (line_number, line) in &matches {
                            // let parts: Vec<&str> = line.split(&self.pattern).collect();
                            // let colored_pattern = &self.pattern.red().to_string();

                            // let colored_content = parts.join(&colored_pattern);

                            replacer.replace(&line, &self.new_pattern, &file_path, *line_number)?;

                            // Increment total matches
                            total_matches += 1;
                        }
                    }
                }
            }
        }

        if total_matches > 0 {
            println!(
                "\n{}",
                format!("{} matches found.", total_matches.to_string().bold(),)
            );
        } else if self.settings.verbose && total_matches == 0 {
            println!(
                "{}",
                format!(
                    "No matches found for \"{}\".",
                    self.old_pattern.red().to_string().bold()
                )
            );
        }
        Ok(())
    }
}
