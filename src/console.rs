use crate::enums::Operation;
use colored::Colorize;

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
        filename: &str,
        pattern: &str,
        line_number: &usize,
        match_index: usize,
    ) {
        let red_pattern = pattern.red().to_string();
        let red_old_content = old_line.replace(pattern, &red_pattern);
       
        println!(
            "\n[{}] {}\n(line {})\n{} {}",
            match_index,
            filename.bold(),
            line_number,
            "--".red(),
            red_old_content
        );
    }

    pub fn print_changes(
        &self,
        old_line: &str,
        filename: &str,
        pattern: &str,
        new_pattern: &str,
        line_number: &usize,
        match_index: usize,
    ) {
        let parts: Vec<&str> = old_line.split(pattern).collect();

        let red_pattern = pattern.red().to_string();
        let green_pattern = new_pattern.green().to_string();

        let red_old_content = parts.join(&red_pattern);
        let green_new_content = parts.join(&green_pattern);

        println!(
            "\n[{}] {}\n(line {})\n{} {}\n{} {}",
            match_index,
            filename.bold(),
            line_number,
            "--".red(),
            red_old_content,
            "++".green(),
            green_new_content
        );
    }

    /// Warn the user in case he used the --write flag but no match was found
    pub fn warn_bare_written(&self) {
        println!(
            "{}",
            "\nYou used the --write flag but no match was found.
Be careful as this command would write changes to disk without confirmation.
Do not use --write when looking for content to replace."
                .red()
        );
    }

    /// Print the number of matches or replacements found
    pub fn print_matches_counts(
        &self,
        matches_count: usize,
        total_lines_walked: i32,
        operation: Operation,
    ) {
        let matches_plural = if matches_count > 1 { "es" } else { "" };
        let lines_walked_plural = if total_lines_walked > 1 { "s" } else { "" };
        let count = matches_count.to_string().green().bold();

        match operation {
            Operation::Match => {
                if matches_count > 0 {
                    println!("\n{}", format!("{} match{} found.\n{} line{} audited.\n\n> Re-run the command with --write to write changes to disk.", count, matches_plural, total_lines_walked, lines_walked_plural));
                } else {
                    println!("\n{}", "No match found.".red());
                }
            }
            Operation::Replacement => {
                println!(
                    "\n{}",
                    format!(
                        "{} match{} replaced.\n{} line{} audited.",
                        count, matches_plural, total_lines_walked, lines_walked_plural
                    )
                );
            }
            Operation::Lookup => {
                if matches_count > 0 {
                    println!(
                        "\n{}",
                        format!(
                            "{} match{} found.\n{} line{} audited.",
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
