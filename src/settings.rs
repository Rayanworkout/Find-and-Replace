use std::path::PathBuf;

#[derive(Debug, Clone)]
/// Settings applied for a DirectoryPatcher run
pub struct Settings {
    /// Control verbosity of console's output
    pub verbose: bool,
    /// If true, ignore case in your search (default: false)
    pub ignore_case: bool,
    /// If true, search hidden files and directories (default: false)
    pub search_hidden: bool,
    /// Files and directories to omit from the results (default: None)
    pub omit_pattern: Vec<PathBuf>,
    /// List of file types to select (default: empty)
    pub selected_file_types: Vec<String>,
    /// List of file types to ignore (default: empty)
    pub ignored_file_types: Vec<String>,
    /// If true, changes are written to disk
    /// (default: false)
    pub write: bool,
}
