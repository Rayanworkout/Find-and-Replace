use colored::Colorize;
use crate::enums::Operation;

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
            "\nAn error occured when reading \"{}\" => {}",
            filename.bold(),
            error.red()
        );
    }

    pub fn print_changes(&self, old_line: &str, filename: &str, pattern: &str, new_pattern: &str) {
        let parts: Vec<&str> = old_line.split(pattern).collect();

        let red_pattern = pattern.red().to_string();
        let green_pattern = new_pattern.green().to_string();

        let red_old_content = parts.join(&red_pattern);
        let green_new_content = parts.join(&green_pattern);

        println!(
            "\n{}\n{} {}\n{} {}",
            filename.bold(),
            "--".red(),
            red_old_content,
            "++".green(),
            green_new_content
        );
    }

    pub fn warn_bare_written(&self) {
        println!(
            "{}",
            "\nYou used the --write flag but no match was found.
        Be careful as this command would write changes to disk without confirmation.
        Do not use --write when looking for content to replace."
                .red()
        );
    }

    pub fn print_match_counts(&self, matches_count: u32, operation: Operation) {
        let plural = if matches_count > 1 { "es" } else { "" };
        let count = matches_count.to_string().green().bold();

        match operation {
            Operation::Match => {
                println!("\n{}", format!("{} match{} found.", count, plural));
            }
            Operation::Replacement => {
                println!("\n{}", format!("{} match{} replaced.", count, plural));
            }
        }
    }
}
