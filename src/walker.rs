use crate::enums::Operation;
use crate::{Console, Replacer, Searcher, Settings};
use anyhow::{Context, Result};
use colored::Colorize;
use ignore::{types::TypesBuilder, WalkBuilder};
use std::path::{Component, Path, PathBuf};

pub struct Walker {
    old_pattern: String,
    new_pattern: String,
    path: PathBuf,
    settings: Settings,
}

impl Walker {
    pub fn new(
        old_pattern: String,
        new_pattern: Option<String>,
        path: PathBuf,
        settings: Settings,
    ) -> Self {
        let new_pattern = new_pattern.unwrap_or_default(); // ignored in lookup mode anyway
        Self {
            old_pattern,
            new_pattern,
            path,
            settings,
        }
    }

    /// Returns true if `entry_path` should be skipped by `--omit`.
    /// Supports exact/prefix paths (`tests/assets`) and component matches
    /// (`assets/` anywhere in the walked path).
    fn path_matches_omit(entry_path: &Path, omit_path: &Path) -> bool {
        // Direct prefix match for explicit paths (e.g. tests/assets).
        if entry_path.starts_with(omit_path) {
            return true;
        }

        // Component-sequence match for generic omits (e.g. assets/).
        let omit_components: Vec<Component<'_>> = omit_path
            .components()
            .filter(|c| !matches!(c, Component::CurDir))
            .collect();
        if omit_components.is_empty() {
            return false;
        }

        let entry_components: Vec<Component<'_>> = entry_path
            .components()
            .filter(|c| !matches!(c, Component::CurDir))
            .collect();

        if omit_components.len() > entry_components.len() {
            return false;
        }

        entry_components
            .windows(omit_components.len())
            .any(|window| window == omit_components.as_slice())
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

        // Apply CLI omit filters at walker level so omitted directories are
        // not descended into.
        let omit_patterns = self.settings.omit_pattern.clone();
        if !omit_patterns.is_empty() {
            walk_builder.filter_entry(move |entry| {
                !omit_patterns
                    .iter()
                    .any(|omit| Walker::path_matches_omit(entry.path(), omit))
            });
        }

        // If settings.search_hidden is true, we set ignore to false
        if self.settings.search_hidden {
            walk_builder.hidden(false);
        }

        // If the custom .fnrignore file exists, we use it
        walk_builder.add_custom_ignore_filename(".fnrignore");

        // By default the walker used .gitignore if present in the folder
        // and ignore mentionned patterns.
        // We don't want that.
        walk_builder.git_ignore(false);
        walk_builder.git_global(false);
        walk_builder.git_exclude(false);
        walk_builder.ignore(false);

        Ok(walk_builder.build())
    }

    pub fn run(&self) -> Result<()> {
        let console = Console::new();
        let walker = self.build_walker()?;
        let searcher = Searcher::new();
        let replacer = Replacer::new(self.settings.clone());

        let mut total_matches = 0;
        let mut total_lines_walked: i32 = 0;
        // We keep track of matches found for indexes
        let mut match_index = 0;

        for entry in walker {
            let entry = entry.with_context(|| {
                "Could not read directory entry. Maybe try with elevated privileges ?".red()
            })?;

            if let Some(file_type) = entry.file_type() {
                if file_type.is_file() {
                    let file_path = entry.path().to_path_buf();
                    let (matches, lines_walked) =
                        searcher.lookup(&file_path, &self.old_pattern, &self.settings, &console)?;

                    // We increment the total lines walked now, because even without matches
                    // we get the counter
                    total_lines_walked += lines_walked;

                    if matches.is_empty() {
                        continue;
                    }

                    // We increment total_matches with the matches of this file
                    total_matches += matches.len();

                    let filename = entry.path().to_string_lossy();

                    for (line_number, line) in &matches {
                        match_index += 1;
                        // If the query is a lookup, we print the lookup
                        // without the changes
                        if self.settings.lookup {
                            _ = console.print_lookup(
                                line,
                                &filename,
                                &self.old_pattern,
                                &line_number,
                                match_index,
                            );

                            continue;
                        }

                        // If the query is a dry-run, no need to call the replacer
                        if !self.settings.write {
                            _ = console.print_changes(
                                line,
                                &filename,
                                &self.old_pattern,
                                &self.new_pattern,
                                &line_number,
                                match_index,
                            );

                            continue;
                        }

                        // If the query is neither a lookup or a dry-run, we need
                        // to call the replacer
                        replacer.replace(
                            &self.new_pattern,
                            &self.old_pattern,
                            &file_path,
                            *line_number,
                        )?;
                    }
                }
            }
        }

        if !self.settings.write {
            console.print_match_counts(total_matches, total_lines_walked, Operation::Match);
        } else {
            if total_matches == 0 {
                console.warn_bare_written();
            }

            console.print_match_counts(total_matches, total_lines_walked, Operation::Replacement);
        }
        println!("{:?}", &self.settings.select);
        Ok(())
    }
}
