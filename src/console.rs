#[allow(dead_code)]
pub struct Console {
    verbose: bool,
}

impl Console {
    pub fn new(verbose: bool) -> Self {
        Self { verbose }
    }

    /// Print a message to the console
    /// (using stdout)
    pub fn print_match(&self, line_number: &usize, line: &str) {
        println!("{line_number}: {line}");
    }

    /// Print an error message to the console
    /// (using stderr)
    pub fn print_error(&self, error: &str, filename: &str) {
        eprintln!("\nAn error occured when reading \"{}\" => {}", filename, error);
    }

    pub fn print_filename(&self, filename: &str) {
        println!("\n{filename}");
    }
}
