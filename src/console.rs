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
    pub fn print_message(&self, message: &str) {
        println!("{message}");
    }

    /// Print an error message to the console
    /// (using stderr)
    pub fn print_error(&self, error: &str) {
        eprintln!("{error}");
    }
}