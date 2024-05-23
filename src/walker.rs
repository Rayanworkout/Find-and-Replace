use std::path::PathBuf;

use crate::{Console, Settings};


#[allow(dead_code)]
pub struct Walker {
    pattern: String,
    path: PathBuf,
    settings: Settings,
    console: Console,
}

impl Walker {
    pub fn new(pattern: String, path: PathBuf, settings: Settings, console: Console) -> Self {
        Self {
            pattern,
            path,
            settings,
            console,
        }
    }

    pub fn run(&self) {
        self.console.print_message("Running the walker");
    }
}