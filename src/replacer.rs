use anyhow::{Context, Result};
use std::{
    fs::File,
    io::{BufWriter, Read, Write},
    path::PathBuf,
};

pub struct Replacer {}

impl Replacer {
    pub fn new() -> Self {
        Self {}
    }

    /// Function to open the file and then replace the old line with the new pattern
    pub fn replace(
        &self,
        new_pattern: &str,
        old_pattern: &str,
        file_path: &PathBuf,
        line_number: usize,
    ) -> Result<()> {
        let mut file = File::open(file_path)
            .with_context(|| format!("Could not open {}", file_path.display()))?;

        let mut file_content = String::new();

        file.read_to_string(&mut file_content)?;

        let mut lines: Vec<&str> = file_content.split('\n').collect();

        let updated_line = lines[line_number - 1].replace(old_pattern, new_pattern);
        lines[line_number - 1] = &updated_line;

        let updated_content = lines.join("\n");

        // Write the modified content back to the file
        let mut writer = BufWriter::new(File::create(&file_path)?);

        writer.write_all(updated_content.as_bytes())?;

        Ok(())
    }
}
