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
    pub fn print_match(&self, with_name: bool, filename: &str, line_number: &usize, line: &str) {
        if with_name {
            println!("{filename}:{line_number}: {line}");
        } else {
            println!("{line_number}: {line}");
        }
    }

    /// Print an error message to the console
    /// (using stderr)
    pub fn print_error(&self, error: &str) {
        eprintln!("An error occured: {error}");
    }
}