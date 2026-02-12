use crate::enums::Operation;
use colored::Colorize;
use num_format::{Locale, ToFormattedString};

#[derive(Clone)]
pub struct Console {}

impl Console {
    pub fn new() -> Self {
        Self {}
    }

    /// Print an error message to the console
    /// (using stderr)
    pub fn print_error(&self, error: &str, filename: &str) {
        eprintln!(
            "\nAn error occured when reading \"{}\" => {}.",
            filename.bold(),
            error.red()
        );
    }

    pub fn print_lookup(
        &self,
        old_line: &str,
        pattern: &str,
        line_number: &usize,
        match_index: usize,
    ) {
        let red_pattern = pattern.red().to_string();
        let red_old_content = old_line.replace(pattern, &red_pattern);

        println!(
            "  [{}] line {}\n  {}",
            match_index, line_number, red_old_content
        );
    }

    pub fn print_file_header(&self, filename: &str) {
        println!("\n{}", filename.bold());
    }

    pub fn print_changes(
        &self,
        old_line: &str,
        pattern: &str,
        new_pattern: &str,
        line_number: &usize,
        match_index: usize,
        match_must_be_greyed: &bool,
    ) {
        let parts: Vec<&str> = old_line.split(pattern).collect();

        // Greyed style for "not selected"
        let (old_pat, new_pat, minus, plus) = if *match_must_be_greyed {
            (
                pattern.bright_black().to_string(),     // grey old match
                new_pattern.bright_black().to_string(), // grey new match
                "--".bright_black().to_string(),
                "++".bright_black().to_string(),
            )
        } else {
            (
                pattern.red().to_string(),
                new_pattern.green().to_string(),
                "--".red().to_string(),
                "++".green().to_string(),
            )
        };

        let old_content = parts.join(&old_pat);
        let new_content = parts.join(&new_pat);

        println!(
            "  [{}] line {}\n  {} {}\n  {} {}",
            match_index, line_number, minus, old_content, plus, new_content
        );
    }

    /// Warn the user when `--write` is enabled but nothing was replaced.
    pub fn warn_no_replacement_applied(&self, found_matches: usize, used_select: bool) {
        let safety_note = "\nBe careful: this command writes changes to disk without confirmation and cannot be undone.
Do not use --write when looking for content to replace, either perform a dry-run or a lookup.";

        let message = if found_matches == 0 {
            format!("\nYou used --write but no match was found.{}", safety_note)
        } else if used_select {
            format!(
                "\nYou used --write with --select, but none of the found matches were selected.{}",
                safety_note
            )
        } else {
            // Impossible, this case happens when
            // found_matches > 0 && used_select is false
            // if found_matches > 0, no warning
            // We keep this for syntax
            format!(
                "\nYou used --write but no replacement was applied.\nNo file was modified.{}",
                safety_note
            )
        };

        println!("{}", message.red());
    }

    /// Print the number of matches or replacements found
    pub fn print_matches_counts(
        &self,
        matches_count: usize,
        selected_matches_count: usize,
        total_lines_walked: i32,
        operation: Operation,
    ) {
        let matches_plural = if matches_count > 1 { "es" } else { "" };
        let lines_walked_plural = if total_lines_walked > 1 { "s" } else { "" };
        let count = matches_count.to_string().green().bold();
        let formatted_total_lines_walked = total_lines_walked.to_formatted_string(&Locale::en);

        let total_lines_walked = formatted_total_lines_walked.to_string().blue().bold();

        let selected_matches_str = if selected_matches_count > 0 {
            format!(" ({selected_matches_count} selected)")
        } else {
            String::new()
        };

        match operation {
            Operation::Match => {
                if matches_count > 0 {
                    println!(
                        "\n{}",
                        format!(
                            "{} match{} found{}.\n{} line{} scanned.\nTip: use --write to apply.",
                            count,
                            matches_plural,
                            selected_matches_str,
                            total_lines_walked,
                            lines_walked_plural
                        )
                    );
                } else {
                    println!(
                        "\n{}\n{} line{} scanned.",
                        "No match found.".red(),
                        total_lines_walked,
                        lines_walked_plural
                    );
                }
            }
            Operation::Replacement => {
                println!(
                    "\n{}",
                    format!(
                        "{} match{} replaced{}.\n{} line{} scanned.",
                        count,
                        matches_plural,
                        selected_matches_str,
                        total_lines_walked,
                        lines_walked_plural
                    )
                );
            }
            Operation::Lookup => {
                if matches_count > 0 {
                    println!(
                        "\n{}",
                        format!(
                            "{} match{} found.\n{} line{} scanned.",
                            count, matches_plural, total_lines_walked, lines_walked_plural
                        )
                    );
                } else {
                    println!("\n{}", "No match found.".red());
                }
            }
        }
    }
}
