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
}
