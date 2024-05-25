use colored::{ColoredString, Colorize};

#[derive(Clone)]
pub struct Console {}

impl Console {
    pub fn new() -> Self {
        Self {}
    }

    /// Print a message to the console
    /// (using stdout)
    pub fn print_match(&self, line_number: &ColoredString, line: &str) {
        println!("{}: {}", line_number, line);
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

    pub fn print_filename(&self, filename: &str) {
        println!("\n{}", filename.bold());
    }

    pub fn print_changes(&self, old_line: &str, new_line: &str, filename: &str) {
        println!(
            "\n{}\n{} {}\n{} {}",
            filename.bold(),
            "--".red(),
            old_line,
            "++".green(),
            new_line
        );
    }
}
