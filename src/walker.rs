use crate::{Console, Searcher, Settings};
use anyhow::{Context, Result};
use ignore::{types::TypesBuilder, WalkBuilder};
use std::path::PathBuf;

#[allow(dead_code)]
pub struct Walker {
    pattern: String,
    path: PathBuf,
    settings: Settings,
    console: Console,
}

impl Walker {
    pub fn new(pattern: String, path: PathBuf, settings: Settings, console: Console) -> Self {
        Self {
            pattern,
            path,
            settings,
            console,
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
        let walker = self.build_walker()?;
        let searcher = Searcher::new();

        for entry in walker {
            let entry = entry.with_context(|| "Could not read directory entry")?;
            let filename = entry.path().to_string_lossy();

            if let Some(file_type) = entry.file_type() {
                if file_type.is_file() {
                    let matches = searcher.lookup(
                        &entry.path().to_path_buf(),
                        &self.pattern,
                        &self.settings,
                        &self.console,
                    )?;

                    if !matches.is_empty() {
                        self.console.print_filename(&filename);
                        for (line_number, line) in &matches {
                            self.console.print_match(line_number, line);
                        }
                    }
                }
            }
        }
        Ok(())
    }
}
