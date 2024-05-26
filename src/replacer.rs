use anyhow::{Context, Result};
use std::{
    fs::File,
    io::{BufWriter, Read, Write},
    path::PathBuf,
};

use crate::{Console, Settings};

pub struct Replacer {
    console: Console,
    settings: Settings,
}

impl Replacer {
    pub fn new(console: Console, settings: Settings) -> Self {
        Self { console, settings }
    }

    /// Function to open the file and then replace the old line with the new pattern
    pub fn replace(
        &self,
        old_line: &str,
        new_pattern: &str,
        old_pattern: &str,
        file_path: &PathBuf,
        line_number: usize,
        filename: &str,
    ) -> Result<()> {
        let mut file = File::open(file_path)
            .with_context(|| format!("Could not open {}", file_path.display()))?;

        let mut file_content = String::new();

        file.read_to_string(&mut file_content)?;

        let mut lines: Vec<&str> = file_content.split('\n').collect();

        if self.settings.write {
            let updated_line = lines[line_number - 1].replace(old_pattern, new_pattern);
            lines[line_number - 1] = &updated_line;

            let updated_content = lines.join("\n");

            // Write the modified content back to the file
            let mut writer = BufWriter::new(File::create(&file_path)?);

            writer.write_all(updated_content.as_bytes())?;
        }
        _ = &self
            .console
            .print_changes(old_line, &filename, &old_pattern, &new_pattern);

        Ok(())
    }
}
